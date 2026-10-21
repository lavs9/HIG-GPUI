use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacToken {
    pub id: ElementId,
    pub title: SharedString,
    pub color: Option<MacAccentColor>,
}

impl MacToken {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            color: None,
        }
    }

    pub fn color(mut self, color: MacAccentColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// A macOS Token Field adhering strictly to Apple Human Interface Guidelines.
/// Renders an editable input container containing discrete token chips with
/// remove buttons ('×') and an active text input slot.
#[derive(IntoElement)]
pub struct MacTokenField {
    id: ElementId,
    style: StyleRefinement,
    tokens: Vec<MacToken>,
    placeholder: SharedString,
    input_value: SharedString,
    disabled: bool,
    on_remove: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacTokenField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            tokens: Vec::new(),
            placeholder: "Add tag...".into(),
            input_value: "".into(),
            disabled: false,
            on_remove: None,
        }
    }

    pub fn token(mut self, title: impl Into<SharedString>) -> Self {
        let count = self.tokens.len();
        self.tokens.push(MacToken::new(format!("token_{}", count), title));
        self
    }

    pub fn tokens(mut self, tokens: impl IntoIterator<Item = MacToken>) -> Self {
        self.tokens.extend(tokens);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn input_value(mut self, val: impl Into<SharedString>) -> Self {
        self.input_value = val.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_remove(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacTokenField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacTokenField {
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
            rgba(0xFFFFFF15)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .items_center()
            .gap_1()
            .min_h(px(26.0))
            .p(px(2.0))
            .px_2()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(5.0))
            .children(self.tokens.into_iter().enumerate().map(|(_idx, tok)| {
                let token_bg = if let Some(accent) = tok.color {
                    accent.color(is_dark)
                } else {
                    apple_colors.accent
                };

                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .h(px(18.0))
                    .px(px(6.0))
                    .rounded_full()
                    .bg(token_bg)
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(rgb(0xFFFFFF))
                            .child(tok.title),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(12.0))
                            .h(px(12.0))
                            .rounded_full()
                            .hover(|s| s.bg(rgba(0x00000030)))
                            .cursor_pointer()
                            .child(
                                svg()
                                    .path(IconName::Close.path())
                                    .size(px(7.0))
                                    .text_color(rgb(0xFFFFFF)),
                            ),
                    )
            }))
            .child(
                div()
                    .flex_1()
                    .min_w(px(60.0))
                    .h(px(20.0))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if self.input_value.is_empty() {
                                apple_colors.placeholder()
                            } else {
                                apple_colors.label
                            })
                            .child(if self.input_value.is_empty() {
                                self.placeholder
                            } else {
                                self.input_value
                            }),
                    ),
            )
    }
}
