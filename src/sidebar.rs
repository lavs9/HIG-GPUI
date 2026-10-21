use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

pub struct MacSidebarItem {
    pub id: ElementId,
    pub title: SharedString,
    pub icon: Option<IconName>,
    pub badge: Option<SharedString>,
    pub selected: bool,
    pub on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacSidebarItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            badge: None,
            selected: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

pub struct MacSidebarSection {
    pub title: Option<SharedString>,
    pub items: Vec<MacSidebarItem>,
}

impl MacSidebarSection {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn item(mut self, item: MacSidebarItem) -> Self {
        self.items.push(item);
        self
    }
}

/// A macOS Source List Sidebar adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSidebar {
    id: ElementId,
    style: StyleRefinement,
    sections: Vec<MacSidebarSection>,
}

impl MacSidebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            sections: Vec::new(),
        }
    }

    pub fn section(mut self, section: MacSidebarSection) -> Self {
        self.sections.push(section);
        self
    }
}

impl Styled for MacSidebar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let sidebar_bg = if is_dark {
            rgba(0x1E1E1EE6) // Translucent vibrant dark
        } else {
            rgba(0xF6F6F6E6) // Translucent vibrant light
        };

        let mut sidebar = div()
            .id(self.id)
            .refine_style(&self.style)
            .w(px(220.0))
            .h_full()
            .bg(sidebar_bg)
            .border_r_1()
            .border_color(apple_colors.separator)
            .px(px(8.0))
            .py(px(10.0))
            .flex()
            .flex_col()
            .gap(px(16.0));

        for section in self.sections {
            let mut sec_col = div().w_full().flex().flex_col().gap(px(1.0));

            if let Some(title) = section.title {
                sec_col = sec_col.child(
                    div()
                        .mac_caption()
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .text_color(apple_colors.tertiary_label)
                        .px(px(8.0))
                        .py(px(4.0))
                        .child(title.to_string().to_uppercase()),
                );
            }

            for item in section.items {
                let is_sel = item.selected;
                let item_bg: Hsla = if is_sel {
                    apple_colors.accent
                } else {
                    rgba(0x00000000).into()
                };

                let text_color: Hsla = if is_sel {
                    rgb(0xFFFFFF).into()
                } else {
                    apple_colors.label
                };

                let icon_color: Hsla = if is_sel {
                    rgb(0xFFFFFF).into()
                } else {
                    apple_colors.accent
                };

                let mut row = div()
                    .id(item.id)
                    .w_full()
                    .h(px(28.0))
                    .px(px(8.0))
                    .rounded(px(6.0))
                    .bg(item_bg)
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .cursor_pointer();

                if !is_sel {
                    row = row
                        .hover(|s| s.bg(apple_colors.control_hover))
                        .active(|s| s.bg(apple_colors.control_pressed));
                }

                if let Some(handler) = item.on_click {
                    row = row.on_click(move |ev, window, cx| handler(ev, window, cx));
                }

                let mut leading = div().flex().items_center().gap(px(8.0)).flex_1();
                if let Some(icon) = item.icon {
                    leading = leading.child(
                        svg()
                            .path(icon.path())
                            .size(px(15.0))
                            .text_color(icon_color),
                    );
                }
                leading = leading.child(
                    div()
                        .mac_body()
                        .text_color(text_color)
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child(item.title),
                );

                row = row.child(leading);

                if let Some(badge) = item.badge {
                    let badge_text_color: Hsla = if is_sel {
                        rgb(0xFFFFFF).into()
                    } else {
                        apple_colors.secondary_label
                    };
                    row = row.child(
                        div()
                            .mac_caption()
                            .text_color(badge_text_color)
                            .child(badge),
                    );
                }

                sec_col = sec_col.child(row);
            }

            sidebar = sidebar.child(sec_col);
        }

        sidebar
    }
}
