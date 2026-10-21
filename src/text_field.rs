use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacFieldSize {
    Small,
    #[default]
    Regular,
    Large,
}

impl MacFieldSize {
    pub fn height(&self) -> f32 {
        match self {
            Self::Small => 20.0,
            Self::Regular => 24.0,
            Self::Large => 28.0,
        }
    }
}

/// A macOS Single-line Text Field adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacTextField {
    id: ElementId,
    style: StyleRefinement,
    state: Entity<InputState>,
    size: MacFieldSize,
    placeholder: Option<SharedString>,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
}

impl MacTextField {
    pub fn new(id: impl Into<ElementId>, state: Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            state,
            size: MacFieldSize::Regular,
            placeholder: None,
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
        }
    }

    pub fn size(mut self, size: MacFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = MacFieldSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = MacFieldSize::Large;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for MacTextField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacTextField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0xFFFFFF14)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        let h = self.size.height();

        let mut field = div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(h))
            .rounded(px(6.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .px(px(8.0))
            .gap(px(6.0))
            .shadow_sm()
            .focus(|s| s.border_color(apple_colors.accent));

        if let Some(icon) = self.leading_icon {
            field = field.child(
                svg()
                    .path(icon.path())
                    .size(px(13.0))
                    .text_color(apple_colors.tertiary_label),
            );
        }

        field = field.child(
            div()
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .child(
                    Input::new(&self.state)
                        .appearance(false)
                        .bordered(false)
                        .h_full(),
                ),
        );

        if let Some(icon) = self.trailing_icon {
            field = field.child(
                svg()
                    .path(icon.path())
                    .size(px(13.0))
                    .text_color(apple_colors.tertiary_label),
            );
        }

        field
    }
}

/// A macOS Password / Secure Text Field adhering to Apple HIG.
#[derive(IntoElement)]
pub struct MacSecureField {
    id: ElementId,
    style: StyleRefinement,
    state: Entity<InputState>,
    size: MacFieldSize,
    placeholder: Option<SharedString>,
    disabled: bool,
}

impl MacSecureField {
    pub fn new(id: impl Into<ElementId>, state: Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            state,
            size: MacFieldSize::Regular,
            placeholder: None,
            disabled: false,
        }
    }

    pub fn size(mut self, size: MacFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for MacSecureField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSecureField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0xFFFFFF14)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        let h = self.size.height();

        div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(h))
            .rounded(px(6.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .px(px(8.0))
            .gap(px(6.0))
            .shadow_sm()
            .focus(|s| s.border_color(apple_colors.accent))
            .child(
                svg()
                    .path("icons/lock.svg")
                    .size(px(13.0))
                    .text_color(apple_colors.tertiary_label),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(
                        Input::new(&self.state)
                            .appearance(false)
                            .bordered(false)
                            .h_full(),
                    ),
            )
    }
}
