use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

pub const MAC_TRAFFIC_LIGHT_CLEARANCE: f32 = 96.0;

/// A macOS Unified Title Bar & Toolbar adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacToolbar {
    id: ElementId,
    style: StyleRefinement,
    title: Option<SharedString>,
    leading: Vec<AnyElement>,
    center: Vec<AnyElement>,
    trailing: Vec<AnyElement>,
}

impl MacToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: None,
            leading: Vec::new(),
            center: Vec::new(),
            trailing: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn leading(mut self, item: impl IntoElement) -> Self {
        self.leading.push(item.into_any_element());
        self
    }

    pub fn center(mut self, item: impl IntoElement) -> Self {
        self.center.push(item.into_any_element());
        self
    }

    pub fn trailing(mut self, item: impl IntoElement) -> Self {
        self.trailing.push(item.into_any_element());
        self
    }
}

impl Styled for MacToolbar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacToolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let has_center = !self.center.is_empty() || self.title.is_some();

        div()
            .id(self.id)
            .refine_style(&self.style)
            .relative()
            .w_full()
            .h(px(52.0))
            .bg(if is_dark {
                rgba(0x323235E6)
            } else {
                rgba(0xF6F6F6E6)
            })
            .border_b_1()
            .border_color(apple_colors.separator)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .pl(px(MAC_TRAFFIC_LIGHT_CLEARANCE))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .children(self.leading),
            )
            .child(
                div()
                    .pr(px(16.0))
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap(px(10.0))
                    .children(self.trailing),
            )
            .when(has_center, |this| {
                this.child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .children(self.title.map(|title| {
                                    div()
                                        .mac_headline()
                                        .text_color(apple_colors.label)
                                        .child(title)
                                }))
                                .children(self.center),
                        ),
                )
            })
    }
}
