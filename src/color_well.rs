use std::rc::Rc;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Color Well adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacColorWell {
    id: ElementId,
    style: StyleRefinement,
    color: Hsla,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacColorWell {
    pub fn new(id: impl Into<ElementId>, color: impl Into<Hsla>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            color: color.into(),
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacColorWell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacColorWell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF33)
        } else {
            rgba(0x00000026)
        };

        let mut el = div()
            .id(self.id)
            .refine_style(&self.style)
            .w(px(26.0))
            .h(px(22.0))
            .rounded(px(5.0))
            .p(px(2.0))
            .bg(if is_dark { rgba(0xFFFFFF1F) } else { rgb(0xFFFFFF).into() })
            .border_1()
            .border_color(border_color)
            .shadow_sm()
            .child(
                div()
                    .w_full()
                    .h_full()
                    .rounded(px(3.0))
                    .bg(self.color)
                    .border_1()
                    .border_color(rgba(0x0000001F)),
            );

        if !self.disabled {
            el = el
                .cursor_pointer()
                .hover(|s| s.border_color(apple_colors.accent));

            if let Some(handler) = self.on_click {
                el = el.on_click(move |ev, window, cx| handler(ev, window, cx));
            }
        } else {
            el = el.opacity(0.45);
        }

        el
    }
}
