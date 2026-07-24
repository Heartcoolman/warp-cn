// We can use `std::process:Command` here because this is invoked within a build script,
// _not_ within the Warp binary (where it could cause a terminal to temporarily flash on
// Windows).
#![allow(clippy::disallowed_types)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use anyhow::Result;
use cfg_aliases::cfg_aliases;
use sha2::Digest;
use walkdir::WalkDir;
use warp_util::assets::{
    ASSETS_DIR, ASYNC_ASSETS_DIR, CONPTY_DLL_FILE, DXCOMPILER_DLL_FILE, DXIL_DLL_FILE,
    OPEN_CONSOLE_EXE_FILE, REMOTE_ASSETS_DIR, WINDOWS_ASSETS_DIR,
};
use warp_util::path::app_target_dir;

fn main() -> Result<()> {
    cfg_aliases! {
        linux_or_windows: { any(target_os = "linux", windows) },
        enable_crash_recovery: { linux_or_windows },
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
    println!("cargo:rerun-if-changed=../script/warp-update.pub");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_FAMILY");
    println!("cargo:rerun-if-env-changed=GIT_RELEASE_TAG");
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_SHA");
    println!("cargo:rerun-if-env-changed=WARP_UPDATE_PUBKEY");
    // WARP_RC selects a standalone Windows resource compiler (see
    // embed_resource_file); INCLUDE is what that compiler reads to find winres.h.
    // Register both so toggling the portable toolchain reruns this script instead
    // of reusing a stale .res from the previous resource-compiler branch.
    println!("cargo:rerun-if-env-changed=WARP_RC");
    println!("cargo:rerun-if-env-changed=INCLUDE");

    emit_git_release_tag_if_unset();
    emit_git_commit_sha_if_unset();
    emit_warp_update_pubkey_if_unset();

    let target_os = env::var("CARGO_CFG_TARGET_OS")?;
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY")?;

    add_features(&target_family, &target_os);

    if target_os == "macos" && target_family != "wasm" {
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=UserNotifications");
        build_and_link_sentry();

        println!("cargo:rerun-if-changed=src/platform/mac/objc/services.h");
        println!("cargo:rerun-if-changed=src/platform/mac/objc/services.m");

        cc::Build::new()
            .file("src/platform/mac/objc/services.m")
            .compile("warp_objc");

        // Build the dock tile plugin
        println!("cargo:rerun-if-changed=DockTilePlugin/WarpDockTilePlugin.m");
        println!("cargo:rerun-if-changed=DockTilePlugin/WarpDockTilePlugin.h");
        println!("cargo:rerun-if-changed=DockTilePlugin/Info.plist");
        println!("cargo:rerun-if-changed=DockTilePlugin/Makefile");

        let min_macos_version = env::var("MACOSX_DEPLOYMENT_TARGET")
            .expect("MACOSX_DEPLOYMENT_TARGET must be set for macos builds");
        let status = Command::new("make")
            .current_dir("DockTilePlugin")
            .env("MACOSX_DEPLOYMENT_TARGET", min_macos_version)
            .status()
            .expect("Failed to build dock tile plugin");
        if !status.success() {
            panic!("Dock tile plugin build failed");
        }

        // Copy the dock tile plugin to the output directory
        let profile = get_build_profile_name();
        let target_dir = app_target_dir(&profile).expect("Failed to get app target directory");
        let plugin_src = Path::new("DockTilePlugin/WarpDockTilePlugin.docktileplugin");
        let plugin_dst = target_dir.join("WarpDockTilePlugin.docktileplugin");

        if !status.success() {
            fs::remove_dir_all(plugin_src).expect("Failed to clean up plugin directory");
            panic!("Dock tile plugin build failed");
        }

        if plugin_src.exists() {
            fs::remove_dir_all(&plugin_dst).ok(); // Remove existing if any
            fs::create_dir_all(&plugin_dst).expect("Failed to create plugin directory");

            // Copy the plugin directory recursively
            for entry in WalkDir::new(plugin_src) {
                let entry = entry.expect("Failed to read plugin directory");
                let path = entry.path();
                let relative = path
                    .strip_prefix(plugin_src)
                    .expect("Failed to strip path prefix");
                let target = plugin_dst.join(relative);

                if path.is_dir() {
                    fs::create_dir_all(target).expect("Failed to create plugin subdirectory");
                } else {
                    fs::copy(path, target).expect("Failed to copy plugin file");
                }
            }

            // Clean up the source plugin directory after copying
            fs::remove_dir_all(plugin_src).expect("Failed to clean up plugin directory");
        }

        // In standalone mode, embed the Info.plist file. We don't use embed_plist! for this
        // because the plist file is dynamically generated.
        if env::var("CARGO_FEATURE_STANDALONE").is_ok() {
            // Don't fail if INFO_PLIST_PATH is unset, since CI runs clippy with --all-features.
            if let Ok(info_plist_path) = env::var("INFO_PLIST_PATH") {
                println!("cargo:rerun-if-env-changed=INFO_PLIST_PATH");
                println!("cargo:rerun-if-changed={info_plist_path}");
                println!("cargo:rustc-link-arg=-sectcreate");
                println!("cargo:rustc-link-arg=__TEXT");
                println!("cargo:rustc-link-arg=__info_plist");
                println!("cargo:rustc-link-arg={info_plist_path}");
            } else {
                eprintln!("Expected INFO_PLIST_PATH to be set")
            }
        }
    }

    if target_os == "windows" {
        // Retrieve the Cargo profile name so that we can put a copy of ConPTY in
        // the correct target subdirectory.
        //
        // `CARGO_FULL_PROFILE` is set by bundle scripts for custom profiles (e.g.
        // release-lto). Fall back to Cargo's built-in `PROFILE` ("debug"/"release")
        // for direct `cargo build` invocations. See also:
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
        //
        // Ideally we could access `CARGO_TARGET_DIR` but this doesn't exist at build time.
        // See https://github.com/rust-lang/cargo/issues/9661.
        let cargo_full_profile = env::var("CARGO_FULL_PROFILE")
            .or_else(|_| env::var("PROFILE"))
            .unwrap_or_else(|_| String::from("debug"));
        let target_dir =
            app_target_dir(&cargo_full_profile).expect("Could not get app target directory");
        copy_windows_assets(&target_dir);

        #[cfg(windows)]
        embed_resource_file(&target_dir);
    }

    if target_family == "wasm" {
        copy_async_assets();
    }

    generate_channel_config_if_needed(&target_family, &target_os);

    Ok(())
}

/// If `warp-channel-config` is available on PATH and the `release_bundle` feature is enabled,
/// invoke the config generator binary and write the JSON output to `OUT_DIR` so it can be
/// embedded via `include_str!` in the binary entry points.
fn generate_channel_config_if_needed(target_family: &str, target_os: &str) {
    if env::var("CARGO_FEATURE_RELEASE_BUNDLE").is_err() {
        // For non-bundled builds, config is loaded at runtime — nothing to embed.
        return;
    }

    let config_bin = "warp-channel-config";

    // Check if the config binary is available on PATH. If not, we can't generate embedded
    // configs. This is expected for external contributors building Warp OSS.
    if Command::new(config_bin)
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_err()
    {
        return;
    }

    // Only track these for bundled builds, where they affect the embedded config.
    // For non-bundled builds these are runtime variables and should not trigger recompilation.
    println!("cargo:rerun-if-env-changed=WITH_LOCAL_SERVER");
    println!("cargo:rerun-if-env-changed=WITH_LOCAL_SESSION_SHARING_SERVER");
    println!("cargo:rerun-if-env-changed=WITH_SANDBOX_TELEMETRY");
    println!("cargo:rerun-if-env-changed=SERVER_ROOT_URL");
    println!("cargo:rerun-if-env-changed=WS_SERVER_URL");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set");
    let family_arg = if target_family == "wasm" {
        "wasm"
    } else {
        "native"
    };

    // Generate config for all internal channels. The build script runs once per crate (not
    // once per binary), so we generate all configs here and each binary's include_str! picks
    // up its own file.
    for channel in ["local", "dev", "stable", "preview"] {
        let output = Command::new(config_bin)
            .arg("--channel")
            .arg(channel)
            .arg("--target-family")
            .arg(family_arg)
            .arg("--target-os")
            .arg(target_os)
            .output()
            .unwrap_or_else(|err| {
                panic!("Failed to execute config generator at '{config_bin}': {err}")
            });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Config generator failed for channel '{channel}':\n{stderr}");
        }

        let config_path = Path::new(&out_dir).join(format!("{channel}_config.json"));
        fs::write(&config_path, &output.stdout).unwrap_or_else(|err| {
            panic!("Failed to write config to {}: {err}", config_path.display())
        });
    }
}

/// When `GIT_RELEASE_TAG` is not provided by the build environment, try to
/// detect it from the working tree via `git describe --tags --exact-match`.
/// Only succeeds when HEAD is exactly on a tag matching the warp release
/// format. Silently no-ops on shallow clones / source tarballs / dev builds —
/// callers that hit `ChannelState::app_version() == None` already handle that.
///
/// CI note: GitHub Actions' default shallow checkout strips tags. To pick up
/// the tag here, the workflow must `actions/checkout` with `fetch-depth: 0`
/// (or `fetch-tags: true`) — or set `GIT_RELEASE_TAG` directly.
///
/// We deliberately do NOT emit `rerun-if-changed=.git/HEAD`: that would force
/// the `app` crate to recompile on every local commit, which dwarfs the value
/// of auto-detection during dev work. Use `cargo clean -p warp` or `touch
/// app/build.rs` to refresh after retagging locally.
/// Bakes the current commit SHA into the binary as `GIT_COMMIT_SHA`. Unlike
/// `GIT_RELEASE_TAG`, this almost always succeeds (only fails on tarball-only
/// source extracts with no `.git`), giving the runtime a stable handle that
/// the `github_update` resolver uses to reverse-lookup the matching release
/// tag via the GitHub API. That path lets us fix a tag mismatch by simply
/// retagging the commit on GitHub — no rebuild, no repackage.
fn emit_git_commit_sha_if_unset() {
    if env::var("GIT_COMMIT_SHA").is_ok() {
        return;
    }
    let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() else {
        return;
    };
    if !output.status.success() {
        return;
    }
    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !sha.is_empty() && sha.chars().all(|c| c.is_ascii_hexdigit()) {
        println!("cargo:rustc-env=GIT_COMMIT_SHA={sha}");
    }
}

fn emit_git_release_tag_if_unset() {
    if env::var("GIT_RELEASE_TAG").is_ok() {
        return;
    }

    if let Some(tag) = git_describe(&["--tags", "--exact-match", "HEAD"]) {
        if looks_like_release_tag(&tag) {
            println!("cargo:rustc-env=GIT_RELEASE_TAG={tag}");
            return;
        }
    }

    if let Some(tag) = git_describe(&["--tags", "--abbrev=0"]) {
        if looks_like_release_tag(&tag) {
            println!("cargo:rustc-env=GIT_RELEASE_TAG={tag}");
        }
    }
}

fn git_describe(args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("describe")
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if tag.is_empty() {
        None
    } else {
        Some(tag)
    }
}

/// Bakes the minisign public key from `script/warp-update.pub` into the
/// binary as `WARP_UPDATE_PUBKEY`. The github_update installer reads it via
/// `option_env!` and refuses to apply an update if it is missing or empty —
/// so a fork that hasn't run `script/generate_update_keys.sh` gets a UI that
/// silently degrades to "Open on GitHub" without ever risking unsigned
/// installs.
fn emit_warp_update_pubkey_if_unset() {
    if env::var("WARP_UPDATE_PUBKEY").is_ok() {
        return;
    }
    let pub_path = Path::new("../script/warp-update.pub");
    let Ok(contents) = fs::read_to_string(pub_path) else {
        return;
    };
    let key_line = contents
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("untrusted comment:"));
    if let Some(line) = key_line {
        // minisign pubkey body is a single base64 line; reject anything that
        // doesn't decode cleanly so a corrupted file fails fast at build time
        // rather than at runtime.
        if line
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'='))
            && line.len() >= 40
        {
            println!("cargo:rustc-env=WARP_UPDATE_PUBKEY={line}");
        }
    }
}

/// Accepts two release tag shapes (mirrors `ParsedGithubVersion::parse`):
///   - `v<major>.<YYYY>.<MM>.<DD>.<HH>.<mm>.<channel>_<patch>`  (7-segment, CI)
///   - `v<major>.<YYYY>.<MM>.<DD>-<channel>.<patch>`            (5-segment, GitHub Releases)
fn looks_like_release_tag(tag: &str) -> bool {
    let Some(rest) = tag.strip_prefix('v') else {
        return false;
    };
    let parts: Vec<&str> = rest.split('.').collect();
    match parts.len() {
        5 => {
            for part in &parts[..3] {
                if part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()) {
                    return false;
                }
            }
            let Some((day, channel)) = parts[3].split_once('-') else {
                return false;
            };
            !day.is_empty()
                && day.chars().all(|c| c.is_ascii_digit())
                && !channel.is_empty()
                && channel.chars().all(|c| c.is_ascii_alphabetic())
                && !parts[4].is_empty()
                && parts[4].chars().all(|c| c.is_ascii_digit())
        }
        7 => {
            for part in &parts[..6] {
                if part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()) {
                    return false;
                }
            }
            let Some((channel, patch)) = parts[6].split_once('_') else {
                return false;
            };
            !channel.is_empty()
                && channel.chars().all(|c| c.is_ascii_alphabetic())
                && !patch.is_empty()
                && patch.chars().all(|c| c.is_ascii_digit())
        }
        _ => false,
    }
}

fn get_build_profile_name() -> String {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    env::var("OUT_DIR")
        .expect("OUT_DIR must be set")
        .split(std::path::MAIN_SEPARATOR)
        .nth_back(3)
        .expect("could not get profile name")
        .to_string()
}

fn add_features(target_family: &str, target_os: &str) {
    if target_family != "wasm" {
        println!("cargo:rustc-cfg=feature=\"local_fs\"");
        println!("cargo:rustc-cfg=feature=\"local_tty\"");
    }

    if target_os != "windows" {
        println!("cargo:rustc-cfg=feature=\"iterm_images\"");
    }

    if env::var("PROFILE").ok().is_some_and(|val| val == "debug") {
        println!("cargo:rustc-cfg=feature=\"agent_mode_debug\"");
    }
}

fn build_and_link_sentry() {
    // Ensure we re-run the build script if the target framework directory changes.
    println!("cargo:rerun-if-env-changed=FRAMEWORK_OVERRIDE");

    // If the cocoa_sentry feature is not enabled, there's nothing more to do here.
    if env::var("CARGO_FEATURE_COCOA_SENTRY").is_err() {
        return;
    }

    // Download/update the Sentry framework.
    let dir_name = env::var("FRAMEWORK_OVERRIDE").unwrap_or_else(|_| "default".to_string());
    let frameworks_dir = format!("frameworks/{dir_name}");
    let standalone = env::var("CARGO_FEATURE_STANDALONE").is_ok();
    download_sentry_framework(&frameworks_dir, &dir_name, standalone);

    let sentry_dir = if standalone {
        "Sentry.xcframework"
    } else {
        "Sentry-Dynamic-WithARM64e.xcframework"
    };
    let sentry_framework_path = format!("{frameworks_dir}/{sentry_dir}/macos-arm64_arm64e_x86_64");

    // Make sure we re-run the build script if the framework directory changes (e.g.: it gets
    // deleted).
    println!("cargo:rerun-if-changed={sentry_framework_path}");

    // Link the Sentry framework, and compile our objc logic that interfaces with it.
    println!("cargo:rustc-link-search=framework=app/{sentry_framework_path}");
    println!("cargo:rustc-link-lib=framework=Sentry");

    // If building standalone, we need to copy some flags from the Sentry build that the static library requires.
    if standalone {
        println!("cargo:rustc-link-lib=c++");
        let swift_library_path = get_xcode_toolchain().join("usr/lib/swift/macosx");
        println!(
            "cargo:rustc-link-search=all={}",
            swift_library_path.display()
        );
        // Embed /usr/lib/swift as LC_RPATH so the deployed standalone CLI binary can
        // find Swift runtime dylibs on the remote host. On macOS 12.3+, Apple ships
        // the Swift runtime as an OS-level framework at /usr/lib/swift. Without this,
        // remote macOS hosts fail with:
        //   dyld: Library not loaded: @rpath/libswiftCore.dylib
        //   Reason: no LC_RPATH's found
        // See: https://github.com/warpdotdev/warp/issues/12631
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }

    compile_sentry_objc_lib(&sentry_framework_path);
}

fn download_sentry_framework(frameworks_dir: &str, dir_name: &str, standalone: bool) {
    // Build absolute path to the script from workspace root.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();
    let script_path = workspace_root.join("script/macos/update_sentry_cocoa");

    let cocoa_sentry_version = match dir_name {
        "default" | "dev" => "9.4.1",
        name => panic!("Invalid framework override: {name}"),
    };

    let mut cmd = Command::new(&script_path);
    cmd.current_dir(workspace_root)
        .arg("--dir")
        .arg(format!("app/{frameworks_dir}"))
        .arg("--version")
        .arg(cocoa_sentry_version);

    if standalone {
        cmd.arg("--static");
    }

    let output = cmd
        .output()
        .expect("Failed to run update_sentry_cocoa script");
    if !output.status.success() {
        panic!(
            "Failed to download/update Sentry frameworks:\n--- stdout\n{}\n--- stderr\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn compile_sentry_objc_lib(sentry_framework_path: &str) {
    println!("cargo:rerun-if-changed=src/platform/mac/objc/crash_reporting.h");
    println!("cargo:rerun-if-changed=src/platform/mac/objc/crash_reporting.m");

    // We need to tell `Clang` to build with a specific framework path. This is represented within
    // Clang by the `-F` flag, which is not supported directly in the `cc::Build` API, so we
    // directly pass the flag and its value instead.
    cc::Build::new()
        .file("src/platform/mac/objc/crash_reporting.m")
        .flag(format!("-F{sentry_framework_path}").as_str())
        .compile("warp_sentry_objc");
}

#[cfg(unix)]
fn get_xcode_toolchain() -> PathBuf {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let mut output = Command::new("xcode-select")
        .arg("-p")
        .output()
        .expect("Could not run xcode-select");
    if !output.status.success() {
        panic!(
            "`xcode-select -p` failed:\n\n--- stdout\n{}\n\n--- stderr\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Trim trailing whitespace.
    while output
        .stdout
        .last()
        .is_some_and(|b| b.is_ascii_whitespace())
    {
        output.stdout.pop();
    }

    PathBuf::from(OsString::from_vec(output.stdout)).join("Toolchains/XcodeDefault.xctoolchain")
}

#[cfg(not(unix))]
fn get_xcode_toolchain() -> PathBuf {
    panic!("get_xcode_toolchain is only supported on macOS")
}

fn copy_async_assets() {
    println!("cargo:rerun-if-changed=assets/async");
    println!("cargo:rerun-if-env-changed=ASSET_TARGET_DIR");
    let Ok(out_dir_str) = env::var("ASSET_TARGET_DIR") else {
        // Don't build assets if no target dir specified.
        return;
    };
    let out_dir = Path::new(&out_dir_str);

    let remote_asset_subdirs = &[ASYNC_ASSETS_DIR, REMOTE_ASSETS_DIR];
    for remote_asset_subdir in remote_asset_subdirs {
        let asset_dir = Path::new(ASSETS_DIR).join(remote_asset_subdir);

        for asset in WalkDir::new(&asset_dir) {
            let asset = asset.expect("access error");
            let asset_path = asset.path();
            if asset_path.is_file() {
                let contents = fs::read(asset_path).expect("could not read file");

                let mut hasher = sha2::Sha256::new();
                hasher.update(&contents);
                let hash: [u8; 32] = hasher.finalize().into();
                let new_relative_path = warp_util::assets::hashed_asset_path(
                    asset_path
                        .strip_prefix(&asset_dir)
                        .expect("asset in unexpected location"),
                    &hash,
                );
                let new_path = out_dir.join(new_relative_path);

                fs::create_dir_all(new_path.parent().unwrap())
                    .expect("failed to create directories");
                fs::write(new_path, contents).expect("failed to copy file");
            }
        }
    }
}

/// Copies the DLLs needed to run Warp on Windows.
///
/// They are organized as follows:
/// - `conpty.dll`
/// - `{platform}/OpenConsole.exe` (ex: `x64/OpenConsole.exe`)
/// - `dxcompiler.dll` (ex: `dxcompiler.dll`)
/// - `dxil.dll` (ex: `dxil.dll`)
fn copy_windows_assets(target_dir: &Path) {
    println!("cargo:rerun-if-changed=assets/windows");

    let target_arch = match std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Target arhcitecture expected")
        .as_str()
    {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        _ => {
            panic!("Unsupported architecture");
        }
    };

    // This directory is architecture-specific.
    let windows_asset_dir = Path::new(ASSETS_DIR)
        .join(WINDOWS_ASSETS_DIR)
        .join(target_arch);

    // Copy conpty.dll into target directory.
    fs::copy(
        windows_asset_dir.join(CONPTY_DLL_FILE),
        target_dir.join(CONPTY_DLL_FILE),
    )
    .unwrap_or_else(|err| {
        panic!("Could not copy conpty.dll from {windows_asset_dir:?} to {target_dir:?}: {err:#}")
    });

    // Copy the DXC DLLs into the target directory.
    for dxc_file in [DXCOMPILER_DLL_FILE, DXIL_DLL_FILE] {
        fs::copy(
            windows_asset_dir.join(dxc_file),
            target_dir.join(dxc_file),
        )
        .unwrap_or_else(|err| {
            panic!("Could not copy {dxc_file} from {windows_asset_dir:?} to {target_dir:?}: {err:#}")
        });
    }

    // Copy OpenConsole.exe into {target_directory}/{arch}.
    let old_open_console_exe = windows_asset_dir.join(OPEN_CONSOLE_EXE_FILE);
    let new_platform_dir = target_dir.join(target_arch);
    let new_open_console_exe = new_platform_dir.join(OPEN_CONSOLE_EXE_FILE);
    fs::create_dir_all(&new_platform_dir).expect("Could not create new platform directory");
    fs::copy(old_open_console_exe, new_open_console_exe)
        .expect("Could not copy platform OpenConsole.exe");
}

#[cfg(windows)]
fn embed_resource_file(target_dir: &Path) {
    use std::io::Write;

    let version = env::var("GIT_RELEASE_TAG").unwrap_or("v0".to_owned());
    let app_name = env::var("WARP_APP_NAME").unwrap_or("Warp".to_owned());
    let bin_name = env::var("CARGO_BIN_NAME").unwrap_or("local".to_owned());

    // The icon lives under channels/<channel>/, where <channel> is the bin name
    // without the optional `warp-` prefix: the OSS binary is `warp-oss` but its
    // channel directory is `oss` (likewise the internal `local`/`dev` bins map
    // directly). Stripping the prefix keeps `cargo build --bin warp-oss` working
    // on Windows instead of panicking on a missing channels/warp-oss/ path.
    let channel = bin_name.strip_prefix("warp-").unwrap_or(&bin_name);

    let icon_path = Path::new("channels")
        .join(channel)
        .join("icon")
        .join("no-padding")
        .join("icon.ico");

    fs::copy(icon_path, target_dir.join("icon.ico"))
        .unwrap_or_else(|err| panic!("Could not copy icon: {err:#}"));

    let resource_file_path = target_dir.join("resource.rc");
    let mut rcfile = fs::File::create(&resource_file_path).unwrap();
    write!(
        rcfile,
        r#"
#pragma code_page(65001)
#include <winres.h>
#define IDI_ICON 0x101

IDI_ICON ICON "icon.ico"
VS_VERSION_INFO VERSIONINFO
FILEVERSION     1,0,0,0
PRODUCTVERSION  1,0,0,0
FILEFLAGSMASK   VS_FFI_FILEFLAGSMASK
FILEFLAGS       0
FILEOS          VOS__WINDOWS32
FILETYPE        VFT_APP
FILESUBTYPE     VFT2_UNKNOWN
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904E4"
        BEGIN
            VALUE "CompanyName",      "Denver Technologies, Inc\0"
            VALUE "FileDescription",  "{app_name}\0"
            VALUE "FileVersion",      "{version}\0"
            VALUE "LegalCopyright",   "© 2025, Denver Technologies, Inc\0"
            VALUE "InternalName",     "\0"
            VALUE "OriginalFilename", "\0"
            VALUE "ProductName",      "Warp\0"
            VALUE "ProductVersion",   "{version}\0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1252
    END
END
"#,
    )
    .unwrap();
    drop(rcfile);

    // If `WARP_RC` names a standalone resource compiler (e.g. LLVM's `llvm-rc`),
    // use it directly instead of locating RC.EXE via the Windows SDK / Visual
    // Studio registry. This enables building the Windows resource with a portable
    // toolchain (e.g. xwin + LLVM) that has no registered SDK install. The
    // compiler locates `winres.h` through the `INCLUDE` environment variable, and
    // `/C 65001` makes it read the UTF-8 `resource.rc` (which contains `©`).
    if let Some(rc_exe) = env::var_os("WARP_RC").filter(|v| !v.is_empty()) {
        let res_path = target_dir.join("resource.res");
        let status = Command::new(&rc_exe)
            .current_dir(target_dir)
            .arg("/nologo")
            .arg("/C")
            .arg("65001")
            .arg("/fo")
            .arg(&res_path)
            .arg(&resource_file_path)
            .status()
            .unwrap_or_else(|err| panic!("Failed to run WARP_RC compiler {rc_exe:?}: {err:#}"));
        assert!(
            status.success(),
            "WARP_RC compiler {rc_exe:?} failed to compile {resource_file_path:?}"
        );
        // `.res` files link directly under both MSVC `link` and LLVM `lld-link`.
        println!("cargo:rustc-link-arg={}", res_path.display());
        return;
    }

    // Obtain MSVC environment so that the rc compiler can find the right headers.
    // https://github.com/nabijaczleweli/rust-embed-resource/issues/11#issuecomment-603655972
    let target = env::var("TARGET").unwrap();
    if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
        for (key, value) in tool.env() {
            unsafe {
                env::set_var(key, value);
            }
        }
    }
    embed_resource::compile(resource_file_path, embed_resource::NONE)
        .manifest_required()
        .unwrap_or_else(|err| panic!("Unable to embed resource file: {err:#}"));
}
