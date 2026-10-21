use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div,
    px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// Formats a keystroke chord with native Apple keyboard glyphs (⌘, ⇧, ⌥, ⌃).
pub fn format_mac_shortcut(chord: &str) -> String {
    chord
        .replace("cmd", "⌘")
        .replace("command", "⌘")
        .replace("super", "⌘")
        .replace("secondary", "⌘")
        .replace("shift", "⇧")
        .replace("alt", "⌥")
        .replace("opt", "⌥")
        .replace("option", "⌥")
        .replace("ctrl", "⌃")
        .replace("control", "⌃")
        .replace("esc", "⎋")
        .replace("enter", "↵")
        .replace("return", "↵")
        .replace("+", "")
        .replace("-", "")
        .replace(" ", "")
}

/// A macOS Keyboard Shortcut Badge adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacKbd {
    id: ElementId,
    style: StyleRefinement,
    keys: SharedString,
}

impl MacKbd {
    pub fn new(id: impl Into<ElementId>, keys: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            keys: keys.into(),
        }
    }
}

impl Styled for MacKbd {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacKbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let formatted = format_mac_shortcut(&self.keys);

        let bg = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x0000000D)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x0000001A)
        };

        div()
            .id(self.id)
            .refine_style(&self.style)
            .mac_caption()
            .text_color(apple_colors.secondary_label)
            .px(px(5.0))
            .py(px(1.0))
            .rounded(px(4.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .child(formatted)
    }
}
