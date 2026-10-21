use std::rc::Rc;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Digit Entry View adhering strictly to Apple Human Interface Guidelines.
/// Designed for PINs, 2FA security codes, or one-time verification tokens with
/// individually boxed character cells, focused ring, and secure concealment option.
#[derive(IntoElement)]
pub struct MacDigitEntry {
    id: ElementId,
    style: StyleRefinement,
    digits: usize,
    value: SharedString,
    secure: bool,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_complete: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl MacDigitEntry {
    pub fn new(id: impl Into<ElementId>, digits: usize) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            digits: digits.max(1),
            value: "".into(),
            secure: false,
            disabled: false,
            on_change: None,
            on_complete: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_complete(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_complete = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacDigitEntry {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacDigitEntry {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let val_chars: Vec<char> = self.value.chars().collect();
        let cell_size = 36.0;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .children((0..self.digits).map(|idx| {
                let char_opt = val_chars.get(idx);
                let is_current = idx == val_chars.len() && !self.disabled;
                let has_value = char_opt.is_some();

                let bg: Hsla = if self.disabled {
                    if is_dark { rgba(0xFFFFFF08).into() } else { rgba(0x00000008).into() }
                } else if is_dark {
                    rgba(0xFFFFFF15).into()
                } else {
                    rgb(0xFFFFFF).into()
                };

                let border_color: Hsla = if is_current {
                    apple_colors.focus_ring
                } else if has_value {
                    if is_dark { rgba(0xFFFFFF40).into() } else { rgba(0x00000040).into() }
                } else if is_dark {
                    rgba(0xFFFFFF20).into()
                } else {
                    rgba(0x00000020).into()
                };

                let display_char = match (char_opt, self.secure) {
                    (Some(_), true) => "●".to_string(),
                    (Some(c), false) => c.to_string(),
                    (None, _) => "".to_string(),
                };

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(cell_size))
                    .h(px(cell_size))
                    .bg(bg)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(6.0))
                    .when(is_current, |d| {
                        d.border_2().border_color(apple_colors.accent)
                    })
                    .child(
                        div()
                            .text_size(px(if self.secure { 11.0 } else { 16.0 }))
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(if self.disabled {
                                apple_colors.disabled()
                            } else {
                                apple_colors.label
                            })
                            .child(SharedString::from(display_char)),
                    )
            }))
    }
}
