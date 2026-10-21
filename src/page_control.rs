use std::rc::Rc;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    StyleRefinement, Styled, Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Page Control adhering strictly to Apple Human Interface Guidelines.
/// Displays a row of indicator dots showing current page in a collection or preview.
#[derive(IntoElement)]
pub struct MacPageControl {
    id: ElementId,
    style: StyleRefinement,
    number_of_pages: usize,
    current_page: usize,
    interactive: bool,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacPageControl {
    pub fn new(id: impl Into<ElementId>, pages: usize) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            number_of_pages: pages.max(1),
            current_page: 0,
            interactive: true,
            on_change: None,
        }
    }

    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.min(self.number_of_pages.saturating_sub(1));
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacPageControl {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPageControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .py_1()
            .children((0..self.number_of_pages).map(|idx| {
                let is_active = idx == self.current_page;
                let dot_bg: Hsla = if is_active {
                    apple_colors.label
                } else if is_dark {
                    rgba(0xFFFFFF33).into()
                } else {
                    rgba(0x00000033).into()
                };

                div()
                    .w(px(if is_active { 8.0 } else { 6.0 }))
                    .h(px(if is_active { 8.0 } else { 6.0 }))
                    .rounded_full()
                    .bg(dot_bg)
                    .when(self.interactive, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(apple_colors.accent))
                    })
            }))
    }
}
