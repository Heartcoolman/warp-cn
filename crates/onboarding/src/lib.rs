// Onboarding library crate

mod agent_onboarding_view;
pub mod callout;
mod model;
pub mod slides;
pub mod telemetry;

/// The user's intention selected during onboarding slides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnboardingIntention {
    Terminal,
    AgentDrivenDevelopment,
}

impl std::fmt::Display for OnboardingIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnboardingIntention::AgentDrivenDevelopment => write!(f, "agent_driven"),
            OnboardingIntention::Terminal => write!(f, "terminal"),
        }
    }
}

pub use callout::{OnboardingCalloutView, OnboardingKeybindings};

/// i18n keys for the AI features enabled when the agent intention is selected.
/// Shared by the intention slide's agent card checklist and the login slide's
/// skip-login confirmation dialog so the two always stay in sync. Render via
/// `warp_i18n::tr!(key)` at the callsite.
pub const AI_FEATURES: &[&str] = &[
    "onboarding-feature-frontier-models",
    "onboarding-feature-oz-cloud",
    "onboarding-feature-auto-fix-errors",
    "onboarding-feature-agentic-control",
    "onboarding-feature-code-review",
    "onboarding-feature-remote-control",
];

/// i18n keys for Warp Drive features. Render via `warp_i18n::tr!(key)`.
pub const WARP_DRIVE_FEATURES: &[&str] = &[
    "onboarding-feature-warp-drive",
    "onboarding-feature-session-sharing",
];

cfg_if::cfg_if! {
    if #[cfg(feature = "bin")] {
        mod telemetry_provider;
        pub use telemetry_provider::MockTelemetryContextProvider;
    }
}

pub mod components;

/// The default mode for new sessions, chosen during onboarding.
/// Mapped to `DefaultSessionMode` at the application boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionDefault {
    #[default]
    Agent,
    Terminal,
}

impl std::fmt::Display for SessionDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionDefault::Agent => write!(f, "agent"),
            SessionDefault::Terminal => write!(f, "terminal"),
        }
    }
}

pub use agent_onboarding_view::{AgentOnboardingAction, AgentOnboardingEvent, AgentOnboardingView};
pub use model::{OnboardingAuthState, SelectedSettings, UICustomizationSettings};
pub use slides::OfferVariant;
pub use telemetry::OnboardingEvent;

pub fn init(app: &mut warpui_core::AppContext) {
    agent_onboarding_view::init(app);
    callout::init(app);
}
