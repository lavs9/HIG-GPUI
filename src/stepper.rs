use std::rc::Rc;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Stepper adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacStepper {
    id: ElementId,
    style: StyleRefinement,
    disabled: bool,
    can_increment: bool,
    can_decrement: bool,
    on_increment: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_decrement: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacStepper {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            disabled: false,
            can_increment: true,
            can_decrement: true,
            on_increment: None,
            on_decrement: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn can_increment(mut self, can: bool) -> Self {
        self.can_increment = can;
        self
    }

    pub fn can_decrement(mut self, can: bool) -> Self {
        self.can_decrement = can;
        self
    }

    pub fn on_increment(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_increment = Some(Rc::new(handler));
        self
    }

    pub fn on_decrement(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_decrement = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacStepper {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacStepper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x0000001A)
        };

        let on_inc = self.on_increment;
        let on_dec = self.on_decrement;
        let inc_enabled = !self.disabled && self.can_increment;
        let dec_enabled = !self.disabled && self.can_decrement;

        let mut up_btn = div()
            .id("stepper-up")
            .w_full()
            .h(px(11.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path("icons/chevron-up.svg")
                    .size(px(10.0))
                    .text_color(if inc_enabled {
                        apple_colors.label
                    } else {
                        apple_colors.tertiary_label
                    }),
            );

        if inc_enabled {
            if let Some(handler) = on_inc {
                up_btn = up_btn
                    .cursor_pointer()
                    .hover(|s| s.bg(apple_colors.control_hover))
                    .active(|s| s.bg(apple_colors.control_pressed))
                    .on_click(move |ev, window, cx| handler(ev, window, cx));
            }
        }

        let mut down_btn = div()
            .id("stepper-down")
            .w_full()
            .h(px(11.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path("icons/chevron-down.svg")
                    .size(px(10.0))
                    .text_color(if dec_enabled {
                        apple_colors.label
                    } else {
                        apple_colors.tertiary_label
                    }),
            );

        if dec_enabled {
            if let Some(handler) = on_dec {
                down_btn = down_btn
                    .cursor_pointer()
                    .hover(|s| s.bg(apple_colors.control_hover))
                    .active(|s| s.bg(apple_colors.control_pressed))
                    .on_click(move |ev, window, cx| handler(ev, window, cx));
            }
        }

        div()
            .id(self.id)
            .refine_style(&self.style)
            .w(px(18.0))
            .h(px(24.0))
            .rounded(px(5.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .shadow_sm()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(up_btn)
            .child(div().w_full().h(px(1.0)).bg(apple_colors.separator))
            .child(down_btn)
    }
}
