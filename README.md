# gpui-apple-hig

Apple Human Interface Guidelines (macOS HIG) component library for GPUI applications.

Designed in accordance with [Apple HIG](https://developer.apple.com/design/human-interface-guidelines/) and [platform-design-skills](https://github.com/ehmo/platform-design-skills).

## Features

- **Semantic Color Tokens**: Apple macOS dynamic system colors (Light and Dark appearances) and full macOS accent palette (Blue, Purple, Pink, Red, Orange, Yellow, Green, Graphite).
- **System Typography**: Semantic typographic hierarchy with `.AppleSystemUIFont` (Large Title, Title 1-3, Headline, Body, Callout, Subheadline, Footnote, Caption) and `SF Mono`.
- **Pixel-Accurate Toggles**:
  - `MacSwitch`: Apple toggle switch in Regular and Mini sizes (mini for grouped rows per HIG), pure white thumb with Apple drop shadow, smooth spring animation, standard green `#34C759` fill.
  - `MacCheckbox`: 14×14pt rounded square with 3.5pt radius, unchecked border, checked accent blue `#007AFF` with white checkmark, mixed state with horizontal dash.
  - `MacRadio`: 14×14pt circular button with concentric accent dot.
- **Segmented Controls**: `MacSegmentedControl` with recessed track, elevated white/gray card for active selection, and smart hairline dividers that hide adjacent to selected items.
- **Push & Prominent Buttons**: `MacButton` with standard 5pt corner radius and macOS heights (20pt, 24pt, 28pt) supporting Push, Prominent (accent fill), Borderless (toolbar), and Destructive.
- **Search Field**: `MacSearchField` capsule shape (28pt height, 14pt radius) with leading magnifying glass, clear button, and focus ring.
- **Grouped Settings Forms**: `MacSettingsGroup` and `MacSettingsRow` implementing macOS System Settings layout with uppercase category headers, rounded 10pt cards, and 16pt inset dividers.
- **Title Bar & Toolbar**: `MacToolbar` with unified styling and left clearance (`MAC_TRAFFIC_LIGHT_CLEARANCE`) for native macOS traffic light window buttons.
- **Status Bar & Badges**: `MacStatusBar`, `MacBadge`, and `MacKbd` for native shortcut glyphs (⌘, ⇧, ⌥, ⌃).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gpui-apple-hig = { path = "../gpui-apple-hig" }
gpui-kit = "0.7.3"
```

## Quick Example

```rust
use gpui_apple_hig::*;
use gpui_kit::*;

fn apply_theme(is_dark: bool, cx: &mut App) {
    apply_apple_theme(is_dark, MacAccentColor::Blue, cx);
}
```
