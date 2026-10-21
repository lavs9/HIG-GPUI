use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacPathItem {
    pub title: SharedString,
    pub icon: Option<IconName>,
}

impl MacPathItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
        }
    }

    pub fn with_icon(title: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            title: title.into(),
            icon: Some(icon),
        }
    }
}

/// A macOS Path Control adhering strictly to Apple Human Interface Guidelines.
/// Renders standard Finder-style breadcrumb path bar with icons, folder names,
/// and chevron separators, allowing quick hierarchical navigation.
#[derive(IntoElement)]
pub struct MacPathControl {
    id: ElementId,
    style: StyleRefinement,
    items: Vec<MacPathItem>,
    disabled: bool,
    on_select: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacPathControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            items: Vec::new(),
            disabled: false,
            on_select: None,
        }
    }

    pub fn item(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(MacPathItem::new(title));
        self
    }

    pub fn item_with_icon(mut self, title: impl Into<SharedString>, icon: IconName) -> Self {
        self.items.push(MacPathItem::with_icon(title, icon));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = MacPathItem>) -> Self {
        self.items.extend(items);
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

impl Styled for MacPathControl {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPathControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let total_items = self.items.len();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .h(px(22.0))
            .px_2()
            .bg(if is_dark { rgba(0xFFFFFF0D) } else { rgba(0x00000008) })
            .border_1()
            .border_color(if is_dark { rgba(0xFFFFFF1F) } else { rgba(0x00000015) })
            .rounded(px(5.0))
            .children(self.items.into_iter().enumerate().map(|(idx, item)| {
                let is_last = idx + 1 == total_items;
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .px(px(4.0))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .when(!self.disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.bg(if is_dark { rgba(0xFFFFFF1F) } else { rgba(0x0000000F) }))
                            })
                            .when_some(item.icon, |d, ico| {
                                d.child(
                                    svg()
                                        .path(ico.path())
                                        .size(px(12.0))
                                        .text_color(if is_last {
                                            apple_colors.accent
                                        } else {
                                            apple_colors.secondary_label
                                        }),
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(if is_last {
                                        gpui_kit::FontWeight::SEMIBOLD
                                    } else {
                                        gpui_kit::FontWeight::NORMAL
                                    })
                                    .text_color(if self.disabled {
                                        apple_colors.disabled()
                                    } else if is_last {
                                        apple_colors.label
                                    } else {
                                        apple_colors.secondary_label
                                    })
                                    .child(item.title),
                            ),
                    )
                    .when(!is_last, |d| {
                        d.child(
                            svg()
                                .path(IconName::ChevronRight.path())
                                .size(px(9.0))
                                .text_color(apple_colors.tertiary_label)
                                .mx(px(2.0)),
                        )
                    })
            }))
    }
}
