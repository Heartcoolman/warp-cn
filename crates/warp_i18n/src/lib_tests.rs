//! Unit tests for the warp_i18n runtime.
//!
//! Tests use the global singleton because that is the only supported entry path; calls
//! are sequenced with a `Mutex` so the suite remains deterministic under `cargo test`.

use crate::*;
use parking_lot::Mutex;
use std::sync::OnceLock;

fn lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn ensure_init() {
    let _g = lock().lock();
    let _ = init(Locale::ZhCn);
}

#[test]
fn render_zh_resolves_known_key() {
    ensure_init();
    let _g = lock().lock();
    set_locale(Locale::ZhCn);
    let s = render("ui-button-ok", None);
    assert_eq!(s, "确定", "key=ui-button-ok zh-CN");
}

#[test]
fn render_en_resolves_known_key() {
    ensure_init();
    let _g = lock().lock();
    set_locale(Locale::En);
    let s = render("ui-button-ok", None);
    assert_eq!(s, "OK");
}

#[test]
fn fallback_to_en_when_zh_missing() {
    // Use synthetic bundles so the production en/zh-CN bundles can stay
    // parity-clean for `cargo xtask check-i18n --check-parity`.
    let bundles = crate::loader::Bundles::from_sources(
        "fallback-only = Fallback only string\n",
        "other-key = 其他字符串\n",
    )
    .expect("test bundles load");
    assert_eq!(
        bundles.render(Locale::ZhCn, "fallback-only", None),
        "Fallback only string",
    );
}

#[test]
fn missing_key_returns_placeholder() {
    ensure_init();
    let _g = lock().lock();
    let s = render("does-not-exist-anywhere", None);
    assert_eq!(s, "{does-not-exist-anywhere}");
}

#[test]
fn locale_switch_changes_render() {
    ensure_init();
    let _g = lock().lock();
    set_locale(Locale::ZhCn);
    assert_eq!(render("ui-button-cancel", None), "取消");
    set_locale(Locale::En);
    assert_eq!(render("ui-button-cancel", None), "Cancel");
}

#[test]
fn watch_notifies_on_set() {
    ensure_init();
    let _g = lock().lock();
    let i18n = global();
    let mut rx = i18n.subscribe();
    set_locale(Locale::En);
    set_locale(Locale::ZhCn);
    // Mark the current value as seen, then ensure we observe a change.
    rx.mark_unchanged();
    set_locale(Locale::En);
    let observed = *rx.borrow_and_update();
    assert_eq!(observed, Locale::En);
}

#[test]
fn plural_selector_zh_one_vs_other() {
    ensure_init();
    let _g = lock().lock();
    set_locale(Locale::ZhCn);
    let one = render(
        "tabs-close-confirm",
        Some(&args_from([("n", FluentValue::from(1))])),
    );
    let many = render(
        "tabs-close-confirm",
        Some(&args_from([("n", FluentValue::from(3))])),
    );
    assert_eq!(one, "关闭 1 个标签页");
    assert_eq!(many, "关闭 3 个标签页");
}

#[test]
fn plural_selector_en_one_vs_other() {
    ensure_init();
    let _g = lock().lock();
    set_locale(Locale::En);
    let one = render(
        "tabs-close-confirm",
        Some(&args_from([("n", FluentValue::from(1))])),
    );
    let many = render(
        "tabs-close-confirm",
        Some(&args_from([("n", FluentValue::from(5))])),
    );
    assert_eq!(one, "Close 1 tab");
    assert_eq!(many, "Close 5 tabs");
}

#[test]
fn production_bundles_load_without_errors() {
    // Regression guard for #39/#44: a duplicate key in any bundle file makes
    // `FluentBundle::add_resource` fail, so `Bundles::load` (and thus
    // `warp_i18n::init`) returns Err and every UI string renders as `{key}`.
    // Loading the real embedded en/zh-CN bundles must always succeed.
    crate::loader::Bundles::load().expect("production bundles must load");
}

#[test]
fn init_is_idempotent() {
    ensure_init();
    let _g = lock().lock();
    let a = global() as *const I18n;
    let _ = init(Locale::En);
    let b = global() as *const I18n;
    assert_eq!(a, b, "init must be idempotent");
}
