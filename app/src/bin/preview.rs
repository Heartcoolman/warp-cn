// On Windows, we don't want to display a console window when the application is running in release
// builds. See https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute.
#![cfg_attr(feature = "release_bundle", windows_subsystem = "windows")]

#[path = "channel_config.rs"]
mod channel_config;

use anyhow::Result;
use warp_core::{
    channel::{Channel, ChannelState},
    features,
};

// Simple wrapper around warp::run() for feature preview channel builds.
fn main() -> Result<()> {
    // Install before any Cocoa call (ChannelState::new reads the bundle).
    #[cfg(target_os = "macos")]
    warp::install_uncaught_exception_handler();

    ChannelState::set(
        ChannelState::new(Channel::Preview, channel_config::load_config!("preview"))
            .with_additional_features(features::PREVIEW_FLAGS)
            .with_additional_features(&[features::FeatureFlag::ForceLogin]),
    );

    warp::run()
}
