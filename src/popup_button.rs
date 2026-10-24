use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::Popover;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    Anchor, App, ClickEvent, ElementId, InteractiveElement, IntoElement, MouseButton,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement,
    Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

pub struct MacMenuItem {
    pub id: ElementId,
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub shortcut: Option<SharedString>,
    pub disabled: bool,
    pub separator_after: bool,
    pub on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacMenuItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            separator_after: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
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

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
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
    anchor: Anchor,
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
            anchor: Anchor::TopLeft,
            on_change: None,
        }
    }

    pub fn item(mut self, item: MacMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<MacMenuItem>) -> Self {
        self.items = items;
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

    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
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

        let bg: gpui_kit::Hsla = if is_dark {
            rgba(0xFFFFFF1F).into()
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        if self.disabled || self.items.is_empty() {
            return div()
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
                .gap(px(8.0))
                .opacity(0.45)
                .child(
                    div()
                        .mac_body()
                        .text_color(apple_colors.disabled())
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child(selected_title),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(1.0))
                        .child(
                            svg()
                                .path(IconName::ChevronUp.path())
                                .size(px(8.0))
                                .text_color(apple_colors.tertiary_label),
                        )
                        .child(
                            svg()
                                .path(IconName::ChevronDown.path())
                                .size(px(8.0))
                                .text_color(apple_colors.tertiary_label),
                        ),
                )
                .into_any_element();
        }

        let id = self.id;
        let style = self.style;
        let items = self.items;
        let selected_index = self.selected_index;
        let on_change = self.on_change;
        let anchor = self.anchor;

        let menu_bg = if is_dark {
            rgba(0x282828F8)
        } else {
            rgba(0xFDFDFDF8)
        };
        let menu_border = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        Popover::new(id.clone())
            .anchor(anchor)
            .offset(px(3.0))
            .overlay_closable(true)
            .trigger_with(move |is_open, _window, _cx| {
                let trigger_bg = if is_open {
                    apple_colors.control_pressed
                } else {
                    bg
                };

                div()
                    .id(id.clone())
                    .refine_style(&style)
                    .h(px(24.0))
                    .min_w(px(110.0))
                    .px(px(8.0))
                    .rounded(px(6.0))
                    .bg(trigger_bg)
                    .border_1()
                    .border_color(border_color)
                    .shadow_sm()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(apple_colors.control_hover))
                    .active(|s| s.bg(apple_colors.control_pressed))
                    .child(
                        div()
                            .mac_body()
                            .text_color(apple_colors.label)
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .child(selected_title),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(1.0))
                            .child(
                                svg()
                                    .path(IconName::ChevronUp.path())
                                    .size(px(8.0))
                                    .text_color(apple_colors.tertiary_label),
                            )
                            .child(
                                svg()
                                    .path(IconName::ChevronDown.path())
                                    .size(px(8.0))
                                    .text_color(apple_colors.tertiary_label),
                            ),
                    )
                    .into_any_element()
            })
            .content(move |_state, window, cx| {
                let popover_entity = cx.entity().clone();
                let parent_view_id = window.current_view();

                let mut menu_div = div()
                    .id("popup-menu-panel")
                    .occlude()
                    .min_w(px(140.0))
                    .bg(menu_bg)
                    .border_1()
                    .border_color(menu_border)
                    .rounded(px(7.0))
                    .shadow_xl()
                    .py(px(4.0))
                    .flex()
                    .flex_col();

                for (idx, item) in items.into_iter().enumerate() {
                    let is_selected = idx == selected_index;
                    let is_disabled = item.disabled;
                    let popover = popover_entity.clone();
                    let on_change = on_change.clone();
                    let item_on_click = item.on_click.clone();

                    let text_color = if is_disabled {
                        apple_colors.disabled()
                    } else {
                        apple_colors.label
                    };

                    let mut row = div()
                        .id(item.id.clone())
                        .h(px(22.0))
                        .mx(px(4.0))
                        .px(px(6.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(8.0));

                    if !is_disabled {
                        row = row
                            .cursor_pointer()
                            .hover(|s| s.bg(apple_colors.accent).text_color(rgb(0xFFFFFF)))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                cx.stop_propagation();
                                popover.update(cx, |state, cx| {
                                    state.dismiss(window, cx);
                                });
                                cx.notify(parent_view_id);

                                if let Some(handler) = &item_on_click {
                                    let ev = ClickEvent::default();
                                    handler(&ev, window, cx);
                                }
                                if let Some(handler) = &on_change {
                                    handler(&idx, window, cx);
                                }
                            });
                    }

                    let mut leading = div().flex().items_center().gap(px(6.0));

                    leading = leading.child(
                        div()
                            .w(px(14.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if is_selected {
                                svg()
                                    .path(IconName::Check.path())
                                    .size(px(11.0))
                                    .text_color(text_color)
                            } else {
                                svg().size(px(0.0))
                            }),
                    );

                    if let Some(icon) = item.icon {
                        leading = leading.child(
                            svg()
                                .path(icon.path())
                                .size(px(12.0))
                                .text_color(text_color),
                        );
                    }

                    leading = leading.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(text_color)
                            .whitespace_nowrap()
                            .child(item.title),
                    );

                    row = row.child(leading);

                    if let Some(shortcut) = item.shortcut {
                        row = row.child(
                            div()
                                .text_size(px(11.0))
                                .text_color(apple_colors.secondary_label)
                                .child(shortcut),
                        );
                    }

                    menu_div = menu_div.child(row);

                    if item.separator_after {
                        menu_div = menu_div.child(
                            div()
                                .h(px(1.0))
                                .bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000015) })
                                .my(px(3.0))
                                .mx(px(6.0)),
                        );
                    }
                }

                menu_div
            })
            .into_any_element()
    }
}

/// A macOS Pull-Down Button (with single down chevron) adhering strictly to Apple HIG.
#[derive(IntoElement)]
pub struct MacPullDownButton {
    id: ElementId,
    style: StyleRefinement,
    title: SharedString,
    icon: Option<IconName>,
    items: Vec<MacMenuItem>,
    disabled: bool,
    anchor: Anchor,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_select: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl MacPullDownButton {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: title.into(),
            icon: None,
            items: Vec::new(),
            disabled: false,
            anchor: Anchor::TopLeft,
            on_click: None,
            on_select: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn item(mut self, item: MacMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<MacMenuItem>) -> Self {
        self.items = items;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
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

        let bg: gpui_kit::Hsla = if is_dark {
            rgba(0xFFFFFF1F).into()
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF29)
        } else {
            rgba(0x00000026)
        };

        if self.disabled {
            let mut leading = div().flex().items_center().gap(px(6.0));
            if let Some(icon) = self.icon {
                leading = leading.child(
                    svg()
                        .path(icon.path())
                        .size(px(13.0))
                        .text_color(apple_colors.disabled()),
                );
            }
            leading = leading.child(
                div()
                    .mac_body()
                    .text_color(apple_colors.disabled())
                    .child(self.title),
            );

            return div()
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
                .gap(px(6.0))
                .opacity(0.45)
                .child(leading)
                .child(
                    svg()
                        .path(IconName::ChevronDown.path())
                        .size(px(10.0))
                        .text_color(apple_colors.tertiary_label),
                )
                .into_any_element();
        }

        // If no items are attached, behave as a regular button with on_click
        if self.items.is_empty() {
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
                .gap(px(6.0))
                .cursor_pointer()
                .hover(|s| s.bg(apple_colors.control_hover))
                .active(|s| s.bg(apple_colors.control_pressed));

            if let Some(handler) = self.on_click {
                btn = btn.on_click(move |ev, window, cx| handler(ev, window, cx));
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

            return btn
                .child(leading)
                .child(
                    svg()
                        .path(IconName::ChevronDown.path())
                        .size(px(10.0))
                        .text_color(apple_colors.secondary_label),
                )
                .into_any_element();
        }

        let id = self.id;
        let style = self.style;
        let title = self.title;
        let icon = self.icon;
        let items = self.items;
        let anchor = self.anchor;
        let on_select = self.on_select;

        let menu_bg = if is_dark {
            rgba(0x282828F8)
        } else {
            rgba(0xFDFDFDF8)
        };
        let menu_border = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        Popover::new(id.clone())
            .anchor(anchor)
            .offset(px(3.0))
            .overlay_closable(true)
            .trigger_with(move |is_open, _window, _cx| {
                let trigger_bg = if is_open {
                    apple_colors.control_pressed
                } else {
                    bg
                };

                let mut leading = div().flex().items_center().gap(px(6.0));
                if let Some(icon) = icon {
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
                        .child(title),
                );

                div()
                    .id(id.clone())
                    .refine_style(&style)
                    .h(px(24.0))
                    .px(px(8.0))
                    .rounded(px(6.0))
                    .bg(trigger_bg)
                    .border_1()
                    .border_color(border_color)
                    .shadow_sm()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(6.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(apple_colors.control_hover))
                    .active(|s| s.bg(apple_colors.control_pressed))
                    .child(leading)
                    .child(
                        svg()
                            .path(IconName::ChevronDown.path())
                            .size(px(10.0))
                            .text_color(apple_colors.secondary_label),
                    )
                    .into_any_element()
            })
            .content(move |_state, window, cx| {
                let popover_entity = cx.entity().clone();
                let parent_view_id = window.current_view();

                let mut menu_div = div()
                    .id("pulldown-menu-panel")
                    .occlude()
                    .min_w(px(180.0))
                    .bg(menu_bg)
                    .border_1()
                    .border_color(menu_border)
                    .rounded(px(7.0))
                    .shadow_xl()
                    .py(px(4.0))
                    .flex()
                    .flex_col();

                for (idx, item) in items.into_iter().enumerate() {
                    let is_disabled = item.disabled;
                    let popover = popover_entity.clone();
                    let on_select = on_select.clone();
                    let item_on_click = item.on_click.clone();

                    let text_color = if is_disabled {
                        apple_colors.disabled()
                    } else {
                        apple_colors.label
                    };

                    let mut row = div()
                        .id(item.id.clone())
                        .h(px(22.0))
                        .mx(px(4.0))
                        .px(px(8.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(12.0));

                    if !is_disabled {
                        row = row
                            .cursor_pointer()
                            .hover(|s| s.bg(apple_colors.accent).text_color(rgb(0xFFFFFF)))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                cx.stop_propagation();
                                popover.update(cx, |state, cx| {
                                    state.dismiss(window, cx);
                                });
                                cx.notify(parent_view_id);

                                if let Some(handler) = &item_on_click {
                                    let ev = ClickEvent::default();
                                    handler(&ev, window, cx);
                                }
                                if let Some(handler) = &on_select {
                                    handler(&idx, window, cx);
                                }
                            });
                    }

                    let mut leading = div().flex().items_center().gap(px(6.0));

                    if let Some(icon) = item.icon {
                        leading = leading.child(
                            svg()
                                .path(icon.path())
                                .size(px(12.0))
                                .text_color(text_color),
                        );
                    }

                    leading = leading.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(text_color)
                            .whitespace_nowrap()
                            .child(item.title),
                    );

                    row = row.child(leading);

                    if let Some(shortcut) = item.shortcut {
                        row = row.child(
                            div()
                                .text_size(px(11.0))
                                .text_color(apple_colors.secondary_label)
                                .child(shortcut),
                        );
                    }

                    menu_div = menu_div.child(row);

                    if item.separator_after {
                        menu_div = menu_div.child(
                            div()
                                .h(px(1.0))
                                .bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000015) })
                                .my(px(3.0))
                                .mx(px(6.0)),
                        );
                    }
                }

                menu_div
            })
            .into_any_element()
    }
}
