use std::rc::Rc;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Window adhering strictly to Apple Human Interface Guidelines.
/// Complete window chrome simulation with native macOS traffic light buttons
/// (close, minimize, zoom), titlebar, subtitle, and vibrant background.
#[derive(IntoElement)]
pub struct MacWindow {
    id: ElementId,
    style: StyleRefinement,
    title: SharedString,
    subtitle: Option<SharedString>,
    width: f32,
    height: f32,
    children: Vec<AnyElement>,
    on_close: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacWindow {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: title.into(),
            subtitle: None,
            width: 480.0,
            height: 320.0,
            children: Vec::new(),
            on_close: None,
        }
    }

    pub fn subtitle(mut self, sub: impl Into<SharedString>) -> Self {
        self.subtitle = Some(sub.into());
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacWindow {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacWindow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgb(0x1E1E1E)
        } else {
            rgb(0xFFFFFF)
        };

        let titlebar_bg = if is_dark {
            rgba(0x2D2D2DE6)
        } else {
            rgba(0xF6F6F6E6)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(self.width))
            .h(px(self.height))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(10.0))
            .shadow_2xl()
            .overflow_hidden()
            // Titlebar with Traffic Lights
            .child(
                div()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(38.0))
                    .px_3()
                    .bg(titlebar_bg)
                    .border_b_1()
                    .border_color(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000010) })
                    // Traffic lights (Close, Minimize, Zoom)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            // Close (Red)
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(apple_colors.traffic_light_close())
                                    .border_1()
                                    .border_color(rgba(0x00000020))
                                    .cursor_pointer(),
                            )
                            // Minimize (Yellow)
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(apple_colors.traffic_light_minimize())
                                    .border_1()
                                    .border_color(rgba(0x00000020))
                                    .cursor_pointer(),
                            )
                            // Zoom (Green)
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(apple_colors.traffic_light_zoom())
                                    .border_1()
                                    .border_color(rgba(0x00000020))
                                    .cursor_pointer(),
                            ),
                    )
                    // Centered Window Title & Subtitle
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(apple_colors.label)
                                    .child(self.title),
                            )
                            .when_some(self.subtitle, |d, sub| {
                                d.child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(apple_colors.secondary_label)
                                        .child(sub),
                                )
                            }),
                    )
                    // Spacer for balance
                    .child(div().w(px(52.0))),
            )
            // Window Content Area
            .child(
                div()
                    .flex_1()
                    .p_3()
                    .overflow_hidden()
                    .children(self.children),
            )
    }
}
