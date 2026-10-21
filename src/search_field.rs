use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::{
    App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// A macOS Search Field adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSearchField {
    id: ElementId,
    style: StyleRefinement,
    state: Entity<InputState>,
    shortcut_hint: Option<SharedString>,
}

impl MacSearchField {
    pub fn new(id: impl Into<ElementId>, state: Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            state,
            shortcut_hint: None,
        }
    }

    pub fn shortcut_hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.shortcut_hint = Some(hint.into());
        self
    }
}

impl Styled for MacSearchField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSearchField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let field_bg = if is_dark {
            rgba(0xFFFFFF1F) // 12% white
        } else {
            rgba(0x0000000D) // 5% black
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x0000001A)
        };

        let search_icon = svg()
            .path(IconName::Search.path())
            .size(px(13.0))
            .text_color(apple_colors.secondary_label);

        let shortcut_el = self.shortcut_hint.map(|hint| {
            div()
                .mac_caption()
                .text_color(apple_colors.tertiary_label)
                .px(px(5.0))
                .py(px(1.0))
                .rounded(px(3.5))
                .bg(if is_dark {
                    rgba(0xFFFFFF14)
                } else {
                    rgba(0x0000000A)
                })
                .child(hint)
        });

        div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(28.0))
            .rounded(px(14.0))
            .bg(field_bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .px(px(8.0))
            .gap(px(6.0))
            .child(search_icon)
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(
                        Input::new(&self.state)
                            .appearance(false)
                            .bordered(false)
                            .cleanable(true)
                            .h_full(),
                    ),
            )
            .children(shortcut_el)
    }
}
