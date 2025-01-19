//! Configure your application.
use crate::core;

use std::borrow::Cow;

/// The settings of an application.
#[derive(Debug, Clone, Default)]
pub struct Settings {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,

    /// Platform specific settings.
    pub platform_specific: PlatformSpecific,
}

/// The platform specific window settings of an application.
#[cfg(not(target_os = "macos"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, DefaultDefault)]
pub struct PlatformSpecific;

/// The platform specific window settings of an application.
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformSpecific {
    /// Activation policy for the application.
    pub activation_policy: ActivationPolicy,

    /// Used to prevent the application from automatically activating when launched if
    /// another application is already active.
    ///
    /// The default behavior is to ignore other applications and activate when launched.
    pub activate_ignoring_other_apps: bool,
}

#[cfg(target_os = "macos")]
impl Default for PlatformSpecific {
    fn default() -> Self {
        Self {
            activation_policy: ActivationPolicy::default(),
            activate_ignoring_other_apps: true,
        }
    }
}

/// Corresponds to `NSApplicationActivationPolicy`.
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivationPolicy {
    /// Corresponds to `NSApplicationActivationPolicyRegular`.
    #[default]
    Regular,
    /// Corresponds to `NSApplicationActivationPolicyAccessory`.
    Accessory,
    /// Corresponds to `NSApplicationActivationPolicyProhibited`.
    Prohibited,
}

impl From<core::Settings> for Settings {
    fn from(settings: core::Settings) -> Self {
        Self {
            id: settings.id,
            fonts: settings.fonts,
            platform_specific: Default::default(),
        }
    }
}
