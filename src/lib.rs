//! # Apple Human Interface Guidelines (macOS HIG) for GPUI
//!
//! A comprehensive, production-grade implementation of Apple HIG for GPUI applications.
//! Built according to Apple's design specifications for macOS:
//! - Complete dynamic semantic colors (Light / Dark appearances and system accents)
//! - Apple system typography (.AppleSystemUIFont and standard semantic hierarchy)
//! - Pixel-accurate toggle switches (Regular and Mini, smooth spring mechanics, standard green)
//! - Checkboxes with unchecked, checked, and mixed dash states
//! - Radio buttons with concentric selection dots
//! - Segmented controls with elevated cards and smart dividers
//! - Push, Prominent, and Borderless macOS buttons
//! - Native macOS capsule search fields with magnifying glass and clear actions
//! - Grouped settings forms with rounded cards and inset dividers
//! - Sliders with continuous/stepped tracks and drop-shadow thumbs
//! - Progress bars and circular activity spinners
//! - Steppers (dual chevron increment/decrement)
//! - Color wells with native inset swatches
//! - Text fields and secure fields with focus rings
//! - Pop-up buttons (double chevron) and pull-down action buttons
//! - Collapsible disclosure controls with rotating triangles
//! - Source list sidebars with selection pills and badges
//! - Alert sheets and modal dialogs with standard action hierarchy
//! - Unified title bars, toolbars with traffic light clearance, and status bars
//! - Keyboard shortcut glyphs (⌘, ⇧, ⌥, ⌃) and badges

pub extern crate gpui_kit as gpui;
pub use gpui_kit::base as gpui_base;

pub mod badge;
pub mod button;
pub mod checkbox;
pub mod color_well;
pub mod dialog;
pub mod disclosure;
pub mod kbd;
pub mod popup_button;
pub mod progress;
pub mod radio;
pub mod search_field;
pub mod segmented;
pub mod settings;
pub mod sidebar;
pub mod slider;
pub mod status_bar;
pub mod stepper;
pub mod switch;
pub mod text_field;
pub mod theme;
pub mod toolbar;
pub mod typography;

pub use badge::{MacBadge, MacBadgeVariant};
pub use button::{MacButton, MacButtonSize, MacButtonVariant};
pub use checkbox::{MacCheckbox, MacCheckboxState};
pub use color_well::MacColorWell;
pub use dialog::{MacDialog, MacDialogVariant};
pub use disclosure::MacDisclosure;
pub use kbd::{format_mac_shortcut, MacKbd};
pub use popup_button::{MacMenuItem, MacPopupButton, MacPullDownButton};
pub use progress::{MacProgress, MacSpinner, MacSpinnerSize};
pub use radio::MacRadio;
pub use search_field::MacSearchField;
pub use segmented::{MacSegment, MacSegmentedControl};
pub use settings::{MacSettingsGroup, MacSettingsRow};
pub use sidebar::{MacSidebar, MacSidebarItem, MacSidebarSection};
pub use slider::MacSlider;
pub use status_bar::MacStatusBar;
pub use stepper::MacStepper;
pub use switch::{MacSwitch, MacSwitchSize};
pub use text_field::{MacFieldSize, MacSecureField, MacTextField};
pub use theme::{apply_apple_theme, AppleAppearance, AppleColors, MacAccentColor};
pub use toolbar::{MacToolbar, MAC_TRAFFIC_LIGHT_CLEARANCE};
pub use typography::{MacTypographyExt, MONO_FONT_NAME, SYSTEM_FONT_NAME};
