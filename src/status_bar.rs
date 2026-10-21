use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// A macOS Bottom Status Bar adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacStatusBar {
    id: ElementId,
    style: StyleRefinement,
    leading: Vec<AnyElement>,
    center: Vec<AnyElement>,
    trailing: Vec<AnyElement>,
}

impl MacStatusBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            leading: Vec::new(),
            center: Vec::new(),
            trailing: Vec::new(),
        }
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

impl Styled for MacStatusBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacStatusBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        div()
            .id(self.id)
            .refine_style(&self.style)
            .w_full()
            .h(px(28.0))
            .bg(apple_colors.toolbar_background)
            .border_t_1()
            .border_color(apple_colors.separator)
            .flex()
            .items_center()
            .justify_between()
            .px(px(14.0))
            .child(
                div()
                    .mac_subheadline()
                    .text_color(apple_colors.secondary_label)
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .children(self.leading),
            )
            .child(
                div()
                    .mac_subheadline()
                    .text_color(apple_colors.secondary_label)
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .children(self.center),
            )
            .child(
                div()
                    .mac_subheadline()
                    .text_color(apple_colors.secondary_label)
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap(px(8.0))
                    .children(self.trailing),
            )
    }
}
