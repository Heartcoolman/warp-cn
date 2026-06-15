//! Compile-time validation for the embedded Fluent bundles.
//!
//! Responsibilities:
//! - Parse every `bundles/<locale>/*.ftl` and abort the build on syntax errors.
//! - Emit `OUT_DIR/key_index.rs` containing a `phf::Set<&'static str>` of keys defined
//!   in `bundles/en/`. Downstream code may `include!` it for runtime sanity checks.
//! - Re-run when bundle contents change.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bundles_dir = manifest_dir.join("bundles");
    println!("cargo:rerun-if-changed={}", bundles_dir.display());

    let mut en_keys: BTreeSet<String> = BTreeSet::new();
    let mut errors: Vec<String> = Vec::new();
    // Per-locale message-id occurrences across ALL files of that locale.
    // `load_locale` (loader.rs) merges every `<locale>/*.ftl` into one
    // `FluentBundle`, and `add_resource` rejects any id already present —
    // within OR across files — with an `Overriding` error. That error aborts
    // `warp_i18n::init`, leaving the global unset so every `t!()` renders as
    // `{key}` (lib.rs:107-110). The syntactic `parse` below does NOT catch
    // duplicate ids, so detect them here and fail the build, mirroring exactly
    // what `add_resource` would reject at runtime.
    let mut occurrences: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for entry in walkdir::WalkDir::new(&bundles_dir).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        println!("cargo:rerun-if-changed={}", entry.path().display());
        let source = match fs::read_to_string(entry.path()) {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("read {}: {e}", entry.path().display()));
                continue;
            }
        };
        let rel = entry
            .path()
            .strip_prefix(&bundles_dir)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| entry.path().display().to_string());
        match fluent_syntax::parser::parse(source.as_str()) {
            Ok(resource) => {
                if let Some(locale) = locale_of(entry.path(), &bundles_dir) {
                    let is_en = locale == "en";
                    let per_key = occurrences.entry(locale).or_default();
                    for item in &resource.body {
                        if let fluent_syntax::ast::Entry::Message(msg) = item {
                            let name = msg.id.name.to_string();
                            if is_en {
                                en_keys.insert(name.clone());
                            }
                            per_key.entry(name).or_default().push(rel.clone());
                        }
                    }
                }
            }
            Err((_, parse_errors)) => {
                for err in parse_errors {
                    errors.push(format!("{}: {err:?}", entry.path().display()));
                }
            }
        }
    }

    for (locale, keys) in &occurrences {
        for (key, files) in keys {
            if files.len() > 1 {
                let mut unique: Vec<&String> = files.iter().collect();
                unique.dedup();
                errors.push(format!(
                    "duplicate key '{key}' in locale '{locale}' ({} occurrences in {}); \
                     FluentBundle::add_resource rejects this and warp_i18n::init() then \
                     fails, rendering every UI string as {{key}}",
                    files.len(),
                    unique
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("warp_i18n: ftl error: {e}");
        }
        panic!("warp_i18n: {} ftl error(s); aborting build", errors.len());
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let dest = out_dir.join("key_index.rs");
    let mut file = fs::File::create(&dest).expect("create key_index.rs");
    let mut builder = phf_codegen::Set::new();
    for k in &en_keys {
        builder.entry(k.as_str());
    }
    writeln!(
        file,
        "pub static EN_KEY_INDEX: phf::Set<&'static str> = {};",
        builder.build()
    )
    .expect("write key_index.rs");
    writeln!(file, "pub const EN_KEY_COUNT: usize = {};", en_keys.len()).unwrap();

    // Emit a static array of (locale, filename, content) for every
    // `bundles/<locale>/*.ftl` using `include_str!`. Replaces the previous
    // rust-embed-based loader, which on CI silently shipped only the `en/`
    // subtree — `zh-CN/` files never made it into the binary, so
    // `Bundles::load` bailed with "no .ftl files found", `warp_i18n::init`
    // returned Err, and every `t!()` rendered as `{key}` (lib.rs:107-110).
    // `include_str!` has no runtime path concept and no feature-flag chain
    // to misconfigure.
    let dest = out_dir.join("embedded_bundles.rs");
    let mut file = fs::File::create(&dest).expect("create embedded_bundles.rs");
    writeln!(
        file,
        "pub static EMBEDDED_BUNDLES: &[(&str, &str, &str)] = &["
    )
    .unwrap();
    let mut entries: Vec<(String, String, PathBuf)> = Vec::new();
    for entry in walkdir::WalkDir::new(&bundles_dir).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        let rel = match entry.path().strip_prefix(&bundles_dir) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let locale = match rel.components().next() {
            Some(c) => c.as_os_str().to_string_lossy().into_owned(),
            None => continue,
        };
        let filename = entry
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_owned();
        if filename.is_empty() {
            continue;
        }
        entries.push((locale, filename, entry.path().to_path_buf()));
    }
    // Sort so build output is deterministic across runs.
    entries.sort_by(|a, b| (a.0.as_str(), a.1.as_str()).cmp(&(b.0.as_str(), b.1.as_str())));
    for (locale, filename, abs_path) in &entries {
        writeln!(
            file,
            "    ({:?}, {:?}, include_str!({:?})),",
            locale, filename, abs_path
        )
        .unwrap();
    }
    writeln!(file, "];").unwrap();
}

/// Locale directory a bundle file lives in (the first path component under
/// `bundles/`), e.g. `en` or `zh-CN`. Returns `None` for files placed directly
/// in `bundles/`.
fn locale_of(path: &Path, bundles_dir: &Path) -> Option<String> {
    path.strip_prefix(bundles_dir)
        .ok()
        .and_then(|p| p.components().next())
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
}
