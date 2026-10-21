//! # Apple Human Interface Guidelines (macOS HIG) for GPUI
//!
//! A comprehensive, production-grade implementation of Apple HIG for GPUI applications.
//! Built according to Apple's complete official component taxonomy across all 7 categories:
//! - **Selection & Input**: Toggles (Switches, Checkboxes, Radio Buttons), Text Fields, Secure Fields,
//!   Combo Boxes, Digit Entry (PIN/2FA), Pickers & Date Pickers, Image Wells, Color Wells,
//!   Segmented Controls, Sliders, Steppers.
//! - **Menus & Actions**: Push/Prominent/Borderless Buttons, Pop-up Buttons, Pull-down Buttons,
//!   Context Menus, The Menu Bar, Toolbars.
//! - **Navigation & Search**: Search Fields, Source List Sidebars, Tab Bars & Tab Views,
//!   Path Controls (Finder Breadcrumbs), Token Fields.
//! - **Presentation**: Alert Dialogs & Sheets, Popovers with Arrow Indicators, Floating Utility Panels / HUD,
//!   Page Controls (Dot Indicators), macOS Window Frames with Traffic Lights.
//! - **Status**: Determinate & Indeterminate Progress Bars, Activity Spinners, Linear & Capacity Gauges,
//!   5-Star Rating Indicators, Concentric Activity Rings, Badges, Status Bars.
//! - **Layout & Organization**: Grouped Settings Cards & Rows, Data Table Views with Sortable Columns & Zebra Stripes,
//!   Resizable Split Views, Collapsible Outline Tree Views, Finder-style Column Views, Disclosure Controls.
//! - **Content**: Multiline Text Views with Line Numbers, Framed Image Views, Apple Charts Bar & Line Visualizations.
//! - **Typography & Keys**: SF Pro typographic hierarchy and native keyboard shortcut glyphs (⌘, ⇧, ⌥, ⌃).

pub extern crate gpui_kit as gpui;
pub use gpui_kit::base as gpui_base;

pub mod activity_rings;
pub mod badge;
pub mod button;
pub mod chart;
pub mod checkbox;
pub mod color_well;
pub mod column_view;
pub mod combo_box;
pub mod context_menu;
pub mod dialog;
pub mod digit_entry;
pub mod disclosure;
pub mod gauge;
pub mod image_view;
pub mod image_well;
pub mod kbd;
pub mod material;
pub mod menu_bar;
pub mod outline_view;
pub mod page_control;
pub mod panel;
pub mod path_control;
pub mod picker;
pub mod popover;
pub mod popup_button;
pub mod progress;
pub mod radio;
pub mod rating_indicator;
pub mod search_field;
pub mod segmented;
pub mod settings;
pub mod sidebar;
pub mod slider;
pub mod split_view;
pub mod status_bar;
pub mod stepper;
pub mod switch;
pub mod tab_view;
pub mod table_view;
pub mod text_field;
pub mod text_view;
pub mod theme;
pub mod token_field;
pub mod toolbar;
pub mod typography;
pub mod window;

pub use activity_rings::{MacActivityRings, MacRingMetric};
pub use badge::{MacBadge, MacBadgeVariant};
pub use button::{MacButton, MacButtonSize, MacButtonVariant};
pub use chart::{MacChart, MacChartDataPoint, MacChartType};
pub use checkbox::{MacCheckbox, MacCheckboxState};
pub use color_well::MacColorWell;
pub use column_view::{MacColumnData, MacColumnEntry, MacColumnView};
pub use combo_box::MacComboBox;
pub use context_menu::{MacContextMenu, MacMenuItemKind};
pub use dialog::{MacDialog, MacDialogVariant};
pub use digit_entry::MacDigitEntry;
pub use disclosure::MacDisclosure;
pub use gauge::{MacGauge, MacGaugeStyle};
pub use image_view::{MacImageContentMode, MacImageView};
pub use image_well::MacImageWell;
pub use kbd::{format_mac_shortcut, MacKbd};
pub use material::{
    MacBlendingMode, MacMaterial, MacMaterialExt, MacMaterialRole, MacMaterialSurface,
    MacMaterialThickness,
};
pub use menu_bar::{MacMenuBar, MacMenuBarMenu};
pub use outline_view::{MacOutlineItem, MacOutlineView};
pub use page_control::MacPageControl;
pub use panel::{MacPanel, MacPanelStyle};
pub use path_control::{MacPathControl, MacPathItem};
pub use picker::{MacDatePicker, MacDatePickerStyle};
pub use popover::{MacPopover, MacPopoverArrowPosition};
pub use popup_button::{MacMenuItem, MacPopupButton, MacPullDownButton};
pub use progress::{MacProgress, MacSpinner, MacSpinnerSize};
pub use radio::MacRadio;
pub use rating_indicator::MacRatingIndicator;
pub use search_field::MacSearchField;
pub use segmented::{MacSegment, MacSegmentedControl};
pub use settings::{MacSettingsGroup, MacSettingsRow};
pub use sidebar::{MacSidebar, MacSidebarItem, MacSidebarSection};
pub use slider::MacSlider;
pub use split_view::MacSplitView;
pub use status_bar::MacStatusBar;
pub use stepper::MacStepper;
pub use switch::{MacSwitch, MacSwitchSize};
pub use tab_view::{MacTabBar, MacTabItem};
pub use table_view::{MacTableColumn, MacTableRow, MacTableView};
pub use text_field::{MacFieldSize, MacSecureField, MacTextField};
pub use text_view::MacTextView;
pub use theme::{apply_apple_theme, AppleAppearance, AppleColors, MacAccentColor};
pub use token_field::{MacToken, MacTokenField};
pub use toolbar::{MacToolbar, MAC_TRAFFIC_LIGHT_CLEARANCE};
pub use typography::{MacTypographyExt, MONO_FONT_NAME, SYSTEM_FONT_NAME};
pub use window::MacWindow;
