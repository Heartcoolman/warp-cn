//! UI language preference (D13).
//!
//! Disk shape: top-level TOML key `language = "zh-CN" | "en" | "system"`. Missing value
//! is mapped to `Language::default() == Zh` per fork policy.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp_core::settings::{
    macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud,
};
use warpui::{AppContext, SingletonEntity};

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    settings_value::SettingsValue,
)]
pub enum Language {
    #[default]
    #[serde(rename = "zh-CN")]
    Zh,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "system")]
    System,
}

impl Language {
    pub fn resolve_locale(self) -> warp_i18n::Locale {
        match self {
            Language::Zh => warp_i18n::Locale::ZhCn,
            Language::En => warp_i18n::Locale::En,
            Language::System => warp_i18n::detect_system_locale()
                .as_deref()
                .and_then(warp_i18n::Locale::parse_bcp47)
                .unwrap_or(warp_i18n::Locale::En),
        }
    }
}

define_settings_group!(LanguageSettings, settings: [
    language: LanguageSetting {
        type: Language,
        default: Language::Zh,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "language",
        description: "UI language preference. \"zh-CN\" or \"en\" forces a locale; \"system\" follows the host OS.",
    },
]);

/// Apply the current [`Language`] setting to [`warp_i18n`] at startup.
///
/// Runtime language changes are saved to settings, but take effect after restart. Rebuilding all
/// translated UI while menus and settings controls are active can leave the app in a partially
/// translated state.
pub fn bind_to_warp_i18n(ctx: &mut AppContext) {
    let initial = LanguageSettings::as_ref(ctx).language.resolve_locale();
    warp_i18n::set_locale(initial);
}
