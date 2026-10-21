use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacTabItem {
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub badge: Option<usize>,
}

impl MacTabItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            badge: None,
        }
    }

    pub fn with_icon(title: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            title: title.into(),
            icon: Some(icon),
            badge: None,
        }
    }

    pub fn badge(mut self, count: usize) -> Self {
        self.badge = Some(count);
        self
    }
}

/// A macOS Tab Bar / Tab View adhering strictly to Apple Human Interface Guidelines.
/// Supports native segmented pill styling or elevated tabs with active indicator,
/// icons, and unread count badges.
#[derive(IntoElement)]
pub struct MacTabBar {
    id: ElementId,
    style: StyleRefinement,
    tabs: Vec<MacTabItem>,
    selected_index: usize,
    disabled: bool,
    on_select: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacTabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            tabs: Vec::new(),
            selected_index: 0,
            disabled: false,
            on_select: None,
        }
    }

    pub fn tab(mut self, title: impl Into<SharedString>) -> Self {
        self.tabs.push(MacTabItem::new(title));
        self
    }

    pub fn tab_item(mut self, item: MacTabItem) -> Self {
        self.tabs.push(item);
        self
    }

    pub fn tabs(mut self, items: impl IntoIterator<Item = MacTabItem>) -> Self {
        self.tabs.extend(items);
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

    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacTabBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacTabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0x00000030)
        } else {
            rgba(0x0000000F)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .h(px(26.0))
            .p(px(2.0))
            .bg(bg)
            .rounded(px(6.0))
            .children(self.tabs.into_iter().enumerate().map(|(idx, tab)| {
                let is_selected = idx == self.selected_index;
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_3()
                    .h_full()
                    .rounded(px(4.0))
                    .when(is_selected, |d| {
                        d.bg(if is_dark { rgba(0xFFFFFF33) } else { rgb(0xFFFFFF).into() })
                            .shadow_xs()
                    })
                    .when(!is_selected && !self.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000008) }))
                    })
                    .when_some(tab.icon, |d, ico| {
                        d.child(
                            svg()
                                .path(ico.path())
                                .size(px(12.0))
                                .text_color(if is_selected {
                                    apple_colors.label
                                } else {
                                    apple_colors.secondary_label
                                }),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(if is_selected {
                                gpui_kit::FontWeight::SEMIBOLD
                            } else {
                                gpui_kit::FontWeight::NORMAL
                            })
                            .text_color(if self.disabled {
                                apple_colors.disabled()
                            } else if is_selected {
                                apple_colors.label
                            } else {
                                apple_colors.secondary_label
                            })
                            .child(tab.title),
                    )
                    .when_some(tab.badge, |d, count| {
                        let badge_bg: Hsla = if is_selected {
                            apple_colors.accent
                        } else {
                            apple_colors.tertiary_grouped_background
                        };

                        d.child(
                            div()
                                .px(px(4.0))
                                .py(px(1.0))
                                .rounded_full()
                                .bg(badge_bg)
                                .text_size(px(9.0))
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(if is_selected {
                                    rgb(0xFFFFFF).into()
                                } else {
                                    apple_colors.secondary_label
                                })
                                .child(SharedString::from(count.to_string())),
                        )
                    })
            }))
    }
}
