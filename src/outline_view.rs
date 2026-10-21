use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacOutlineItem {
    pub id: ElementId,
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub level: usize,
    pub is_expanded: bool,
    pub has_children: bool,
}

impl MacOutlineItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>, level: usize) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            level,
            is_expanded: false,
            has_children: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    pub fn has_children(mut self, has: bool) -> Self {
        self.has_children = has;
        self
    }
}

/// A macOS Outline View adhering strictly to Apple Human Interface Guidelines.
/// Displays hierarchical trees with rotating disclosure chevrons, folder icons,
/// and indented items.
#[derive(IntoElement)]
pub struct MacOutlineView {
    id: ElementId,
    style: StyleRefinement,
    items: Vec<MacOutlineItem>,
    selected_id: Option<ElementId>,
    on_select: Option<Rc<dyn Fn(&ElementId, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Rc<dyn Fn(&ElementId, &mut Window, &mut App) + 'static>>,
}

impl MacOutlineView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            items: Vec::new(),
            selected_id: None,
            on_select: None,
            on_toggle: None,
        }
    }

    pub fn item(mut self, item: MacOutlineItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = MacOutlineItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn selected_id(mut self, id: Option<ElementId>) -> Self {
        self.selected_id = id;
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&ElementId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ElementId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacOutlineView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacOutlineView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(1.0))
            .w_full()
            .children(self.items.into_iter().map(|item| {
                let is_selected = self.selected_id.as_ref() == Some(&item.id);
                let indent = (item.level as f32) * 16.0;
                let row_bg: Hsla = if is_selected {
                    apple_colors.accent
                } else {
                    rgba(0x00000000).into()
                };

                div()
                    .flex()
                    .items_center()
                    .h(px(22.0))
                    .pl(px(indent + 4.0))
                    .pr_2()
                    .rounded(px(4.0))
                    .bg(row_bg)
                    .cursor_pointer()
                    .hover(|s| {
                        if !is_selected {
                            s.bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x0000000A) })
                        } else {
                            s
                        }
                    })
                    // Disclosure Chevron
                    .child(
                        div()
                            .w(px(14.0))
                            .h(px(14.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if item.has_children {
                                svg()
                                    .path(if item.is_expanded {
                                        IconName::ChevronDown.path()
                                    } else {
                                        IconName::ChevronRight.path()
                                    })
                                    .size(px(8.0))
                                    .text_color(if is_selected {
                                        rgb(0xFFFFFF).into()
                                    } else {
                                        apple_colors.secondary_label
                                    })
                            } else {
                                svg().size(px(0.0))
                            }),
                    )
                    // Optional Icon
                    .when_some(item.icon, |d, ico| {
                        d.child(
                            svg()
                                .path(ico.path())
                                .size(px(12.0))
                                .text_color(if is_selected {
                                    rgb(0xFFFFFF).into()
                                } else {
                                    apple_colors.accent
                                })
                                .mr_1(),
                        )
                    })
                    // Title
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if is_selected {
                                rgb(0xFFFFFF).into()
                            } else {
                                apple_colors.label
                            })
                            .child(item.title),
                    )
            }))
    }
}
