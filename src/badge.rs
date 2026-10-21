use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacBadgeVariant {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

/// A macOS Pill Badge adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacBadge {
    id: ElementId,
    style: StyleRefinement,
    text: SharedString,
    variant: MacBadgeVariant,
}

impl MacBadge {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            text: text.into(),
            variant: MacBadgeVariant::Neutral,
        }
    }

    pub fn variant(mut self, variant: MacBadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn neutral(mut self) -> Self {
        self.variant = MacBadgeVariant::Neutral;
        self
    }

    pub fn accent(mut self) -> Self {
        self.variant = MacBadgeVariant::Accent;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = MacBadgeVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = MacBadgeVariant::Warning;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = MacBadgeVariant::Danger;
        self
    }
}

impl Styled for MacBadge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let (bg, text_color): (Hsla, Hsla) = match self.variant {
            MacBadgeVariant::Neutral => {
                if is_dark {
                    (rgba(0xFFFFFF1F).into(), apple_colors.secondary_label)
                } else {
                    (rgba(0x0000000D).into(), apple_colors.secondary_label)
                }
            }
            MacBadgeVariant::Accent => (apple_colors.accent.opacity(0.15), apple_colors.accent),
            MacBadgeVariant::Success => (
                apple_colors.system_green.opacity(0.15),
                apple_colors.system_green,
            ),
            MacBadgeVariant::Warning => (
                apple_colors.system_orange.opacity(0.15),
                apple_colors.system_orange,
            ),
            MacBadgeVariant::Danger => (
                apple_colors.system_red.opacity(0.15),
                apple_colors.system_red,
            ),
        };

        div()
            .id(self.id)
            .refine_style(&self.style)
            .mac_caption()
            .text_color(text_color)
            .px(px(7.0))
            .py(px(1.5))
            .rounded(px(9999.0))
            .bg(bg)
            .flex()
            .items_center()
            .justify_center()
            .child(self.text)
    }
}
