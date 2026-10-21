//! # Apple Human Interface Guidelines (macOS HIG) for GPUI
//!
//! A comprehensive, production-grade implementation of Apple HIG for GPUI applications.
//! Built according to Apple's design specifications for macOS:
//! - Complete dynamic semantic colors (Light / Dark appearances and system accents)
//! - Apple system typography (.AppleSystemUIFont and standard semantic hierarchy)
//! - Pixel-accurate toggle switches (Regular and Mini, smooth spring mechanics, standard green)
//! - Checkboxes with uncheck, checked, and mixed dash states
//! - Radio buttons with concentric selection dots
//! - Segmented controls with elevated cards and smart dividers
//! - Push, Prominent, and Borderless macOS buttons
//! - Native macOS capsule search fields with magnifying glass and clear actions
//! - Grouped settings forms with rounded cards and inset dividers
//! - Unified title bars, toolbars with traffic light clearance, and status bars
//! - Keyboard shortcut glyphs (⌘, ⇧, ⌥, ⌃) and badges

pub extern crate gpui_kit as gpui;
pub use gpui_kit::base as gpui_base;

pub mod badge;
pub mod button;
pub mod checkbox;
pub mod kbd;
pub mod radio;
pub mod search_field;
pub mod segmented;
pub mod settings;
pub mod status_bar;
pub mod switch;
pub mod theme;
pub mod toolbar;
pub mod typography;

pub use badge::{MacBadge, MacBadgeVariant};
pub use button::{MacButton, MacButtonSize, MacButtonVariant};
pub use checkbox::{MacCheckbox, MacCheckboxState};
pub use kbd::{MacKbd, format_mac_shortcut};
pub use radio::MacRadio;
pub use search_field::MacSearchField;
pub use segmented::{MacSegment, MacSegmentedControl};
pub use settings::{MacSettingsGroup, MacSettingsRow};
pub use status_bar::MacStatusBar;
pub use switch::{MacSwitch, MacSwitchSize};
pub use theme::{AppleAppearance, AppleColors, MacAccentColor, apply_apple_theme};
pub use toolbar::{MAC_TRAFFIC_LIGHT_CLEARANCE, MacToolbar};
pub use typography::{MONO_FONT_NAME, MacTypographyExt, SYSTEM_FONT_NAME};
