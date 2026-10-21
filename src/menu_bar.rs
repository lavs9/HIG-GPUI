use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacMenuBarMenu {
    pub title: SharedString,
    pub is_app_menu: bool,
    pub is_apple_logo: bool,
}

/// A macOS Menu Bar adhering strictly to Apple Human Interface Guidelines.
/// Renders standard 24px frosted menu bar across the top of the screen with
/// Apple logo, bold app menu, standard top-level menus, and trailing status items.
#[derive(IntoElement)]
pub struct MacMenuBar {
    id: ElementId,
    style: StyleRefinement,
    app_name: SharedString,
    menus: Vec<SharedString>,
    active_menu: Option<usize>,
    on_select_menu: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacMenuBar {
    pub fn new(id: impl Into<ElementId>, app_name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            app_name: app_name.into(),
            menus: vec![
                "File".into(),
                "Edit".into(),
                "View".into(),
                "Window".into(),
                "Help".into(),
            ],
            active_menu: None,
            on_select_menu: None,
        }
    }

    pub fn menu(mut self, menu: impl Into<SharedString>) -> Self {
        self.menus.push(menu.into());
        self
    }

    pub fn menus(mut self, menus: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.menus = menus.into_iter().map(Into::into).collect();
        self
    }

    pub fn active_menu(mut self, index: Option<usize>) -> Self {
        self.active_menu = index;
        self
    }

    pub fn on_select_menu(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select_menu = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacMenuBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacMenuBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0x1E1E1EE6)
        } else {
            rgba(0xFFFFFFE6)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x00000015)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_between()
            .h(px(24.0))
            .px_3()
            .bg(bg)
            .border_b_1()
            .border_color(border_color)
            .child(
                // Leading menu items
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Apple Icon
                    .child(
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0xFFFFFF1F)))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(apple_colors.label)
                                    .child(""),
                            ),
                    )
                    // App Name (bold)
                    .child(
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0xFFFFFF1F)))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(apple_colors.label)
                                    .child(self.app_name),
                            ),
                    )
                    // Standard Menus
                    .children(self.menus.into_iter().enumerate().map(|(idx, title)| {
                        let is_active = self.active_menu == Some(idx);
                        let item_bg: Hsla = if is_active {
                            apple_colors.accent
                        } else {
                            rgba(0x00000000).into()
                        };

                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .bg(item_bg)
                            .hover(|s| {
                                if !is_active {
                                    s.bg(if is_dark { rgba(0xFFFFFF1F) } else { rgba(0x0000000F) })
                                } else {
                                    s
                                }
                            })
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(if is_active {
                                        rgb(0xFFFFFF).into()
                                    } else {
                                        apple_colors.label
                                    })
                                    .child(title),
                            )
                    })),
            )
            .child(
                // Trailing system items (Search, Control Center, Battery, Time)
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        svg()
                            .path(IconName::Search.path())
                            .size(px(12.0))
                            .text_color(apple_colors.secondary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(apple_colors.label)
                            .child("Wed 12:00 PM"),
                    ),
            )
    }
}
