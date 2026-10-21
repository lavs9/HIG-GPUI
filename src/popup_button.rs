use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

pub struct MacMenuItem {
    pub id: ElementId,
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub disabled: bool,
    pub separator_after: bool,
}

impl MacMenuItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            disabled: false,
            separator_after: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn separator_after(mut self, sep: bool) -> Self {
        self.separator_after = sep;
        self
    }
}

/// A macOS Pop-Up Button (with double up/down chevrons) adhering strictly to Apple HIG.
#[derive(IntoElement)]
pub struct MacPopupButton {
    id: ElementId,
    style: StyleRefinement,
    items: Vec<MacMenuItem>,
    selected_index: usize,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl MacPopupButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            items: Vec::new(),
            selected_index: 0,
            disabled: false,
            on_change: None,
        }
    }

    pub fn item(mut self, item: MacMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacPopupButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPopupButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let selected_title = self
            .items
            .get(self.selected_index)
            .map(|i| i.title.clone())
            .unwrap_or_else(|| "Select…".into());

        let bg = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        let next_idx = if !self.items.is_empty() {
            (self.selected_index + 1) % self.items.len()
        } else {
            0
        };

        let on_change = self.on_change;

        let mut btn = div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(24.0))
            .min_w(px(110.0))
            .px(px(8.0))
            .rounded(px(6.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .shadow_sm()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0));

        if !self.disabled {
            btn = btn
                .cursor_pointer()
                .hover(|s| s.bg(apple_colors.control_hover))
                .active(|s| s.bg(apple_colors.control_pressed))
                .on_click(move |_ev, window, cx| {
                    if let Some(handler) = &on_change {
                        handler(&next_idx, window, cx);
                    }
                });
        } else {
            btn = btn.opacity(0.45);
        }

        btn.child(
            div()
                .mac_body()
                .text_color(apple_colors.label)
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .child(selected_title),
        )
        .child(
            // Native macOS stacked double chevrons indicator
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(1.0))
                .child(
                    svg()
                        .path("icons/chevron-up.svg")
                        .size(px(8.0))
                        .text_color(apple_colors.tertiary_label),
                )
                .child(
                    svg()
                        .path("icons/chevron-down.svg")
                        .size(px(8.0))
                        .text_color(apple_colors.tertiary_label),
                ),
        )
    }
}

/// A macOS Pull-Down Button (with single down chevron) adhering strictly to Apple HIG.
#[derive(IntoElement)]
pub struct MacPullDownButton {
    id: ElementId,
    style: StyleRefinement,
    title: SharedString,
    icon: Option<IconName>,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacPullDownButton {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: title.into(),
            icon: None,
            disabled: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
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

impl Styled for MacPullDownButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPullDownButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        let mut btn = div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(24.0))
            .px(px(8.0))
            .rounded(px(6.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .shadow_sm()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(6.0));

        if !self.disabled {
            btn = btn
                .cursor_pointer()
                .hover(|s| s.bg(apple_colors.control_hover))
                .active(|s| s.bg(apple_colors.control_pressed));

            if let Some(handler) = self.on_click {
                btn = btn.on_click(move |ev, window, cx| handler(ev, window, cx));
            }
        } else {
            btn = btn.opacity(0.45);
        }

        let mut leading = div().flex().items_center().gap(px(6.0));
        if let Some(icon) = self.icon {
            leading = leading.child(
                svg()
                    .path(icon.path())
                    .size(px(13.0))
                    .text_color(apple_colors.label),
            );
        }
        leading = leading.child(
            div()
                .mac_body()
                .text_color(apple_colors.label)
                .child(self.title),
        );

        btn.child(leading).child(
            svg()
                .path("icons/chevron-down.svg")
                .size(px(10.0))
                .text_color(apple_colors.secondary_label),
        )
    }
}
