use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Mismatch {
    pub bundle: String,
    pub kind: MismatchKind,
    pub key: String,
}

#[derive(Debug)]
pub enum MismatchKind {
    MissingInZh,
    MissingInEn,
    /// Same message id defined more than once within a locale (across its
    /// `.ftl` files). `load_locale` merges every `<locale>/*.ftl` into one
    /// `FluentBundle`; `add_resource` rejects the duplicate, so
    /// `warp_i18n::init` fails and every UI string renders as `{key}` (#39/#44).
    Duplicate { locale: String, files: Vec<String> },
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MismatchKind::MissingInZh => {
                write!(f, "parity: {} key {:?} missing in zh-CN", self.bundle, self.key)
            }
            MismatchKind::MissingInEn => {
                write!(f, "parity: {} key {:?} missing in en", self.bundle, self.key)
            }
            MismatchKind::Duplicate { locale, files } => write!(
                f,
                "duplicate: key {:?} defined {} times in locale {} ({}) — \
                 breaks warp_i18n::init() at runtime, all UI renders as {{key}}",
                self.key,
                files.len(),
                locale,
                files.join(", "),
            ),
        }
    }
}

/// Compare en/zh-CN bundles file-by-file and flag duplicate keys within each
/// locale. Returns the union of missing keys plus any duplicates.
pub fn check(bundles_root: &Path) -> Result<Vec<Mismatch>> {
    let en = collect(&bundles_root.join("en"))?;
    let zh = collect(&bundles_root.join("zh-CN"))?;
    let mut mismatches = Vec::new();

    let names: BTreeSet<&String> = en.keys().chain(zh.keys()).collect();
    for name in names {
        let empty = BTreeSet::new();
        let en_keys = en.get(name).unwrap_or(&empty);
        let zh_keys = zh.get(name).unwrap_or(&empty);
        for k in en_keys.difference(zh_keys) {
            mismatches.push(Mismatch {
                bundle: name.clone(),
                kind: MismatchKind::MissingInZh,
                key: k.clone(),
            });
        }
        for k in zh_keys.difference(en_keys) {
            mismatches.push(Mismatch {
                bundle: name.clone(),
                kind: MismatchKind::MissingInEn,
                key: k.clone(),
            });
        }
    }

    mismatches.extend(duplicates(&bundles_root.join("en"), "en")?);
    mismatches.extend(duplicates(&bundles_root.join("zh-CN"), "zh-CN")?);
    Ok(mismatches)
}

/// Flag any message id defined more than once across a locale's `.ftl` files
/// (including twice in the same file). Mirrors `FluentBundle::add_resource`'s
/// duplicate-id rejection so the failure surfaces at lint time, not as a
/// runtime `{key}` blowup (#39/#44).
fn duplicates(dir: &Path, locale: &str) -> Result<Vec<Mismatch>> {
    let mut counts: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if dir.exists() {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.into_path();
            if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
                continue;
            }
            let name = path
                .strip_prefix(dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            for key in parse_keys_list(&text) {
                counts.entry(key).or_default().push(name.clone());
            }
        }
    }
    let mut out = Vec::new();
    for (key, files) in counts {
        if files.len() > 1 {
            out.push(Mismatch {
                bundle: locale.to_string(),
                key,
                kind: MismatchKind::Duplicate {
                    locale: locale.to_string(),
                    files,
                },
            });
        }
    }
    Ok(out)
}

fn collect(dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.into_path();
        if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        let name = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        out.entry(name).or_default().extend(parse_keys(&text));
    }
    Ok(out)
}

/// Minimal Fluent key extractor: a key starts at the beginning of a line and
/// matches `[A-Za-z][A-Za-z0-9_-]*` followed by ` =`. Comments (`#`) and
/// continuation lines are ignored. Deduplicates within a file.
fn parse_keys(text: &str) -> BTreeSet<String> {
    parse_keys_list(text).into_iter().collect()
}

/// Like [`parse_keys`] but preserves every occurrence in source order so callers
/// can detect keys defined more than once within a single file.
fn parse_keys_list(text: &str) -> Vec<String> {
    let mut keys = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        let trimmed = line.trim_end();
        let Some(eq) = trimmed.find('=') else {
            continue;
        };
        let head = trimmed[..eq].trim();
        if head.is_empty() {
            continue;
        }
        let mut chars = head.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        if !first.is_ascii_alphabetic() {
            continue;
        }
        if !chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            continue;
        }
        keys.push(head.to_string());
    }
    keys
}
