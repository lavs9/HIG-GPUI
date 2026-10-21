use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Image Well adhering strictly to Apple Human Interface Guidelines.
/// An image well displays an image or visual preview and acts as a drop target
/// for dragged image files.
#[derive(IntoElement)]
pub struct MacImageWell {
    id: ElementId,
    style: StyleRefinement,
    width: f32,
    height: f32,
    caption: Option<SharedString>,
    placeholder: SharedString,
    has_image: bool,
    disabled: bool,
    editable: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacImageWell {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            width: 120.0,
            height: 120.0,
            caption: None,
            placeholder: "Drop image here".into(),
            has_image: false,
            disabled: false,
            editable: true,
            on_click: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn has_image(mut self, has: bool) -> Self {
        self.has_image = has;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacImageWell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacImageWell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if self.has_image {
            if is_dark { rgb(0x1E1E1E) } else { rgb(0xEAEAEA) }
        } else if is_dark {
            rgba(0xFFFFFF0D)
        } else {
            rgba(0x00000008)
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
            .items_center()
            .gap_1()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .w(px(self.width))
                    .h(px(self.height))
                    .bg(bg)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(8.0))
                    .when(self.editable && !self.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.border_color(apple_colors.accent))
                    })
                    .child(
                        if self.has_image {
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .child(
                                    svg()
                                        .path(IconName::Image.path())
                                        .size(px(32.0))
                                        .text_color(apple_colors.accent),
                                )
                                .child(
                                    div()
                                        .mt_1()
                                        .text_size(px(10.0))
                                        .text_color(apple_colors.secondary_label)
                                        .child("RAW Image Preview"),
                                )
                        } else {
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .px_2()
                                .child(
                                    svg()
                                        .path(IconName::Plus.path())
                                        .size(px(20.0))
                                        .text_color(apple_colors.secondary_label),
                                )
                                .child(
                                    div()
                                        .mt_1()
                                        .text_size(px(10.0))
                                        .text_color(apple_colors.secondary_label)
                                        .child(self.placeholder),
                                )
                        },
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
