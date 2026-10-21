# Apple HIG for GPUI (`gpui-apple-hig`)

[![Documentation](https://img.shields.io/badge/Documentation-GitHub%20Pages-007AFF?style=flat&logo=apple)](https://lavs9.github.io/hig-gpui/)
[![Platform](https://img.shields.io/badge/Platform-macOS%20Sonoma%20%7C%20Sequoia-black?style=flat&logo=apple)](https://developer.apple.com/design/human-interface-guidelines/)
[![Rust](https://img.shields.io/badge/Rust-GPUI-dea584?style=flat&logo=rust)](https://github.com/lavs9/hig-gpui)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

> **Live Interactive Showcase & Documentation:**  
> **👉 [https://lavs9.github.io/hig-gpui/](https://lavs9.github.io/hig-gpui/)**  
> Explore interactive components, visual state galleries (Light & Dark appearances), live accent switcher, HIG specs, and copyable Rust GPUI snippets.

---

A comprehensive, production-grade implementation of Apple Human Interface Guidelines for desktop applications written in Rust with GPUI. 
Built according to the official [Apple HIG Component Taxonomy](https://developer.apple.com/design/human-interface-guidelines/components) across all 7 categories:

## Official Apple HIG Component Taxonomy

### 1. Selection and Input
- **[`MacSwitch`](https://lavs9.github.io/hig-gpui/#switch)**: Binary toggle switch with smooth spring physics (Regular 38×22pt, Mini 28×16pt, `#34C759` Green).
- **[`MacCheckbox`](https://lavs9.github.io/hig-gpui/#checkbox)**: Tri-state checkbox (Unchecked, Checked, Mixed indeterminate dash).
- **[`MacRadio`](https://lavs9.github.io/hig-gpui/#radio)**: 14pt mutually exclusive circular radio buttons with concentric selection dots.
- **[`MacColorWell`](https://lavs9.github.io/hig-gpui/#color-well)**: Native inset color swatch trigger with system palette integration.
- **[`MacComboBox`](https://lavs9.github.io/hig-gpui/#combo-box)**: Text field paired with drop-down picker for custom or preset input.
- **[`MacDigitEntry`](https://lavs9.github.io/hig-gpui/#digit-entry)**: PIN / 2FA verification boxes with active focus rings and secure bullet concealing.
- **[`MacImageWell`](https://lavs9.github.io/hig-gpui/#image-well)**: Inset image well drop target for files with thumbnail and caption preview.
- **[`MacDatePicker`](https://lavs9.github.io/hig-gpui/#date-picker)**: Segmented date & time controls with calendar popover and stepper buttons.
- **[`MacSegmentedControl`](https://lavs9.github.io/hig-gpui/#segmented)**: Elevated white card pill tabs with hairline divider management.
- **[`MacSlider`](https://lavs9.github.io/hig-gpui/#slider)**: Continuous and stepped range sliders with 18pt drop-shadow thumbs.
- **[`MacStepper`](https://lavs9.github.io/hig-gpui/#stepper)**: Stacked dual chevron increment/decrement control.
- **[`MacTextField` & `MacSecureField`](https://lavs9.github.io/hig-gpui/#text-field)**: Single-line text inputs and secure password fields.

### 2. Menus and Actions
- **[`MacButton`](https://lavs9.github.io/hig-gpui/#button)**: Push, Prominent Accent, Destructive, and Borderless buttons (20pt, 24pt, 28pt).
- **[`MacPopupButton`](https://lavs9.github.io/hig-gpui/#popup-button)**: Double chevron (`⇕`) pop-up menu button for mutually exclusive selection.
- **[`MacPullDownButton`](https://lavs9.github.io/hig-gpui/#popup-button)**: Down chevron (`▾`) pull-down action button for context commands.
- **[`MacContextMenu`](https://lavs9.github.io/hig-gpui/#context-menu)**: Floating contextual menu with keyboard shortcuts, checkmarks, and destructive rows.
- **[`MacMenuBar`](https://lavs9.github.io/hig-gpui/#menu-bar)**: Full 24pt frosted macOS menu bar with Apple logo, bold app title, and menus.
- **[`MacToolbar`](https://lavs9.github.io/hig-gpui/#chrome)**: Unified toolbar with 78pt traffic light clearance.

### 3. Navigation and Search
- **[`MacSearchField`](https://lavs9.github.io/hig-gpui/#search-field)**: 14pt capsule search field with magnifying glass and keyboard shortcut badge.
- **[`MacSidebar`](https://lavs9.github.io/hig-gpui/#sidebar-comp)**: Source list sidebar with uppercase section headers, selection pills, and badges.
- **[`MacTabBar` & `MacTabView`](https://lavs9.github.io/hig-gpui/#tab-bar)**: Segmented/pill tab bar with count badges and view switching.
- **[`MacPathControl`](https://lavs9.github.io/hig-gpui/#path-control)**: Finder-style breadcrumbs trail with icons, folder names, and chevrons.
- **[`MacTokenField`](https://lavs9.github.io/hig-gpui/#token-field)**: Tokenized chip input container with remove buttons for tags and recipients.

### 4. Presentation
- **[`MacDialog`](https://lavs9.github.io/hig-gpui/#dialog)**: Sheet alerts dropping down from window chrome with primary, cancel, and destructive actions.
- **[`MacPopover`](https://lavs9.github.io/hig-gpui/#popover)**: Floating callout card with top/bottom pointer arrow and shadow.
- **[`MacPanel`](https://lavs9.github.io/hig-gpui/#panel)**: Floating utility tool palettes and dark semi-translucent HUD inspectors.
- **[`MacPageControl`](https://lavs9.github.io/hig-gpui/#page-control)**: Horizontal dot indicator sequence for flat collection navigation.
- **[`MacWindow`](https://lavs9.github.io/hig-gpui/#window-comp)**: Complete macOS window chrome with red/yellow/green traffic lights, titlebar, and subtitle.

### 5. Status & Telemetry
- **[`MacProgress` & `MacSpinner`](https://lavs9.github.io/hig-gpui/#progress)**: Determinate and indeterminate shimmer progress bars and circular activity spinners.
- **[`MacGauge`](https://lavs9.github.io/hig-gpui/#gauge)**: Linear capacity meter with warning (orange) and critical (red) thresholds.
- **[`MacRatingIndicator`](https://lavs9.github.io/hig-gpui/#rating)**: 5-star rating control with interactive selection and filled/empty glyphs.
- **[`MacActivityRings`](https://lavs9.github.io/hig-gpui/#activity-rings)**: Concentric multi-ring circular telemetry indicators.
- **[`MacBadge`](https://lavs9.github.io/hig-gpui/#badge)**: Semantic pill badges (Neutral, Accent, Success, Warning, Danger).
- **[`MacStatusBar`](https://lavs9.github.io/hig-gpui/#status-bar)**: 28pt window bottom status bar with hairline divider.

### 6. Layout and Organization
- **[`MacSettingsGroup` & `MacSettingsRow`](https://lavs9.github.io/hig-gpui/#settings)**: Grouped settings cards with 10pt radius and 52pt inset dividers.
- **[`MacTableView`](https://lavs9.github.io/hig-gpui/#table-view)**: Multi-column data grid with headers, sort arrows, and zebra striping.
- **[`MacSplitView`](https://lavs9.github.io/hig-gpui/#split-view)**: Resizable dual-pane container with draggable 1px splitter divider.
- **[`MacOutlineView`](https://lavs9.github.io/hig-gpui/#outline-view)**: Expandable hierarchical tree navigator with rotating disclosure triangles.
- **[`MacColumnView`](https://lavs9.github.io/hig-gpui/#column-view)**: Finder Miller columns browser for cascading hierarchical navigation.
- **[`MacDisclosure`](https://lavs9.github.io/hig-gpui/#disclosure)**: Collapsible accordion disclosure section with animated triangle.

### 7. Content
- **[`MacTextView`](https://lavs9.github.io/hig-gpui/#text-view)**: Multiline text editor with optional line numbers gutter and word counting.
- **[`MacImageView`](https://lavs9.github.io/hig-gpui/#image-view)**: Framed image container with corner radius and aspect fit/fill options.
- **[`MacChart`](https://lavs9.github.io/hig-gpui/#chart)**: Swift Charts-style clean bar and line data visualizations.

### 8. Typography and Keys
- **[`MacTypography`](https://lavs9.github.io/hig-gpui/#typography)**: SF Pro font hierarchy (`.mac_large_title()`, `.mac_title1()`, `.mac_body()`, `.mac_caption()`).
- **[`MacKbd`](https://lavs9.github.io/hig-gpui/#kbd)**: Formats shortcut strings into native glyphs (⌘, ⇧, ⌥, ⌃, ↵).

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gpui-apple-hig = { git = "https://github.com/lavs9/hig-gpui", branch = "main" }
gpui-kit = "0.1"
```

## Running the Native Desktop Showcase

```bash
cargo run --example showcase
```

To filter for a specific component:
```bash
cargo run --example showcase switch
cargo run --example showcase button
cargo run --example showcase table
```
