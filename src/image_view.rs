use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacImageContentMode {
    #[default]
    AspectFit,
    AspectFill,
    Center,
}

/// A macOS Image View adhering strictly to Apple Human Interface Guidelines.
/// Displays images with rounded corners, aspect ratios, subtle borders, and optional caption.
#[derive(IntoElement)]
pub struct MacImageView {
    id: ElementId,
    style: StyleRefinement,
    caption: Option<SharedString>,
    content_mode: MacImageContentMode,
    width: f32,
    height: f32,
    corner_radius: f32,
    bordered: bool,
}

impl MacImageView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            caption: None,
            content_mode: MacImageContentMode::AspectFit,
            width: 200.0,
            height: 140.0,
            corner_radius: 8.0,
            bordered: true,
        }
    }

    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn content_mode(mut self, mode: MacImageContentMode) -> Self {
        self.content_mode = mode;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
}

impl Styled for MacImageView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacImageView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgb(0x181818)
        } else {
            rgb(0xF0F0F2)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x00000015)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .w(px(self.width))
                    .h(px(self.height))
                    .bg(bg)
                    .when(self.bordered, |d| {
                        d.border_1().border_color(border_color)
                    })
                    .rounded(px(self.corner_radius))
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .path(IconName::Image.path())
                            .size(px(36.0))
                            .text_color(apple_colors.accent),
                    ),
            )
            .when_some(self.caption, |d, cap| {
                d.child(
                    div()
                        .text_size(px(11.0))
                        .text_color(apple_colors.secondary_label)
                        .child(cap),
                )
            })
    }
}
