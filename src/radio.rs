use std::rc::Rc;
use gpui_kit::base::{StyledExt, spring};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// A macOS Radio Button adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacRadio {
    id: ElementId,
    style: StyleRefinement,
    selected: bool,
    disabled: bool,
    label: Option<SharedString>,
    description: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl MacRadio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            selected: false,
            disabled: false,
            label: None,
            description: None,
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacRadio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacRadio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let selected = self.selected;
        let disabled = self.disabled;

        let border_color: Hsla = if selected {
            apple_colors.accent
        } else if is_dark {
            rgba(0xFFFFFF4D).into()
        } else {
            rgba(0x0000003D).into()
        };

        let bg_color: Hsla = if is_dark {
            rgba(0xFFFFFF14).into()
        } else {
            rgb(0xFFFFFF).into()
        };

        let dot_scale = spring(
            (self.id.clone(), "dot_scale"),
            if selected { 6.0 } else { 0.0 },
            cx.theme().motion_tokens().spring_control,
            window,
            cx,
        );

        let on_click = self.on_click.clone();

        div()
            .id(self.id)
            .refine_style(&self.style)
            .flex()
            .items_start()
            .gap_2()
            .when(disabled, |this| this.opacity(0.45).cursor_not_allowed())
            .when(!disabled, |this| {
                this.cursor_pointer().on_click(move |_: &ClickEvent, window, cx| {
                    if let Some(ref handler) = on_click {
                        handler(window, cx);
                    }
                })
            })
            .child(
                div().pt(px(2.0)).child(
                    div()
                        .w(px(14.0))
                        .h(px(14.0))
                        .rounded(px(7.0))
                        .bg(bg_color)
                        .border_1()
                        .border_color(border_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .flex_shrink_0()
                        .child(
                            div()
                                .w(px(dot_scale))
                                .h(px(dot_scale))
                                .rounded(px(3.0))
                                .bg(apple_colors.accent),
                        ),
                ),
            )
            .children((self.label.is_some() || self.description.is_some()).then(|| {
                div()
                    .flex()
                    .flex_col()
                    .when_some(self.label, |this, label| {
                        this.child(
                            div()
                                .mac_body()
                                .text_color(apple_colors.label)
                                .child(label),
                        )
                    })
                    .when_some(self.description, |this, desc| {
                        this.child(
                            div()
                                .mac_subheadline()
                                .text_color(apple_colors.secondary_label)
                                .child(desc),
                        )
                    })
            }))
    }
}
