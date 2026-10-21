use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    StyleRefinement, Styled, Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Split View adhering strictly to Apple Human Interface Guidelines.
/// Manages two adjacent panes with a draggable 1px divider bar and subtle resize handle.
#[derive(IntoElement)]
pub struct MacSplitView {
    id: ElementId,
    style: StyleRefinement,
    leading_width: f32,
    min_leading_width: f32,
    max_leading_width: f32,
    leading_pane: Option<AnyElement>,
    trailing_pane: Option<AnyElement>,
}

impl MacSplitView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            leading_width: 220.0,
            min_leading_width: 160.0,
            max_leading_width: 360.0,
            leading_pane: None,
            trailing_pane: None,
        }
    }

    pub fn leading_width(mut self, width: f32) -> Self {
        self.leading_width = width.clamp(self.min_leading_width, self.max_leading_width);
        self
    }

    pub fn leading(mut self, pane: impl IntoElement) -> Self {
        self.leading_pane = Some(pane.into_any_element());
        self
    }

    pub fn trailing(mut self, pane: impl IntoElement) -> Self {
        self.trailing_pane = Some(pane.into_any_element());
        self
    }
}

impl Styled for MacSplitView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSplitView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let divider_color = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x00000015)
        };

        let mut root = div()
            .id(self.id)
            .flex()
            .w_full()
            .h_full()
            .overflow_hidden();

        if let Some(leading) = self.leading_pane {
            root = root.child(
                div()
                    .w(px(self.leading_width))
                    .h_full()
                    .overflow_hidden()
                    .child(leading),
            );
        }

        root = root.child(
            div()
                .w(px(1.0))
                .h_full()
                .bg(divider_color)
                .cursor_col_resize()
                .hover(|s| s.bg(apple_colors.accent)),
        );

        if let Some(trailing) = self.trailing_pane {
            root = root.child(
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(trailing),
            );
        }

        root
    }
}
