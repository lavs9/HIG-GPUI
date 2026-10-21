use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacPopoverArrowPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// A macOS Popover adhering strictly to Apple Human Interface Guidelines.
/// Renders a transient floating card with arrow pointer, subtle drop shadow,
/// vibrant background, and custom content slot.
#[derive(IntoElement)]
pub struct MacPopover {
    id: ElementId,
    style: StyleRefinement,
    title: Option<SharedString>,
    arrow_position: MacPopoverArrowPosition,
    width: f32,
    children: Vec<AnyElement>,
    on_close: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacPopover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: None,
            arrow_position: MacPopoverArrowPosition::Top,
            width: 240.0,
            children: Vec::new(),
            on_close: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn arrow_position(mut self, pos: MacPopoverArrowPosition) -> Self {
        self.arrow_position = pos;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
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

impl Styled for MacPopover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPopover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0x282828F2)
        } else {
            rgba(0xFFFFFFF2)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .w(px(self.width));

        if self.arrow_position == MacPopoverArrowPosition::Top {
            root = root.child(
                div()
                    .w(px(14.0))
                    .h(px(8.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .path(IconName::Triangle.path())
                            .size(px(10.0))
                            .text_color(border_color),
                    ),
            );
        }

        root = root.child(
            div()
                .w_full()
                .bg(bg)
                .border_1()
                .border_color(border_color)
                .rounded(px(10.0))
                .shadow_xl()
                .p_3()
                .when_some(self.title, |d, t| {
                    d.child(
                        div()
                            .pb_2()
                            .mb_2()
                            .border_b_1()
                            .border_color(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000010) })
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(apple_colors.label)
                                    .child(t),
                            ),
                    )
                })
                .children(self.children),
        );

        if self.arrow_position == MacPopoverArrowPosition::Bottom {
            root = root.child(
                div()
                    .w(px(14.0))
                    .h(px(8.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .path(IconName::Triangle.path())
                            .size(px(10.0))
                            .text_color(border_color),
                    ),
            );
        }

        root
    }
}
