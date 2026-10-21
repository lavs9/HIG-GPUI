use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Text View adhering strictly to Apple Human Interface Guidelines.
/// Supports multiline text editing, optional line numbers, character counts,
/// and placeholder text.
#[derive(IntoElement)]
pub struct MacTextView {
    id: ElementId,
    style: StyleRefinement,
    text: SharedString,
    placeholder: SharedString,
    show_line_numbers: bool,
    width: f32,
    height: f32,
    disabled: bool,
}

impl MacTextView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            text: "".into(),
            placeholder: "Enter notes or code...".into(),
            show_line_numbers: false,
            width: 320.0,
            height: 120.0,
            disabled: false,
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for MacTextView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacTextView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if self.disabled {
            if is_dark { rgba(0xFFFFFF08) } else { rgba(0x00000008) }
        } else if is_dark {
            rgba(0xFFFFFF12)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        let lines: Vec<&str> = if self.text.is_empty() {
            vec![]
        } else {
            self.text.lines().collect()
        };

        let mut root = div()
            .id(self.id)
            .flex()
            .w(px(self.width))
            .h(px(self.height))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(6.0))
            .overflow_hidden();

        if self.show_line_numbers {
            root = root.child(
                div()
                    .w(px(32.0))
                    .h_full()
                    .bg(if is_dark { rgba(0x00000030) } else { rgba(0x00000008) })
                    .border_r_1()
                    .border_color(border_color)
                    .p_2()
                    .children((1..=lines.len().max(3)).map(|l_num| {
                        div()
                            .text_size(px(11.0))
                            .text_color(apple_colors.tertiary_label)
                            .child(SharedString::from(l_num.to_string()))
                    })),
            );
        }

        root.child(
            div()
                .flex_1()
                .h_full()
                .p_2()
                .child(
                    if self.text.is_empty() {
                        div()
                            .text_size(px(12.0))
                            .text_color(apple_colors.placeholder())
                            .child(self.placeholder)
                    } else {
                        div()
                            .text_size(px(12.0))
                            .text_color(if self.disabled {
                                apple_colors.disabled()
                            } else {
                                apple_colors.label
                            })
                            .child(self.text)
                    },
                ),
        )
    }
}
