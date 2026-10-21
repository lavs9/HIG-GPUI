use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub enum MacMenuItemKind {
    Item {
        title: SharedString,
        icon: Option<IconName>,
        shortcut: Option<SharedString>,
        checked: bool,
        disabled: bool,
        destructive: bool,
    },
    Separator,
    Header(SharedString),
}

/// A macOS Context Menu adhering strictly to Apple Human Interface Guidelines.
/// Renders standard macOS translucent blur card with standard row heights (20-22px),
/// checkmarks, SF Symbols/icons, keyboard shortcuts, and separators.
#[derive(IntoElement)]
pub struct MacContextMenu {
    id: ElementId,
    style: StyleRefinement,
    items: Vec<MacMenuItemKind>,
    width: f32,
    on_select: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            items: Vec::new(),
            width: 200.0,
            on_select: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn item(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(MacMenuItemKind::Item {
            title: title.into(),
            icon: None,
            shortcut: None,
            checked: false,
            disabled: false,
            destructive: false,
        });
        self
    }

    pub fn item_with_icon(mut self, title: impl Into<SharedString>, icon: IconName) -> Self {
        self.items.push(MacMenuItemKind::Item {
            title: title.into(),
            icon: Some(icon),
            shortcut: None,
            checked: false,
            disabled: false,
            destructive: false,
        });
        self
    }

    pub fn item_full(
        mut self,
        title: impl Into<SharedString>,
        icon: Option<IconName>,
        shortcut: Option<SharedString>,
        checked: bool,
        disabled: bool,
        destructive: bool,
    ) -> Self {
        self.items.push(MacMenuItemKind::Item {
            title: title.into(),
            icon,
            shortcut,
            checked,
            disabled,
            destructive,
        });
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(MacMenuItemKind::Separator);
        self
    }

    pub fn header(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(MacMenuItemKind::Header(title.into()));
        self
    }

    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacContextMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacContextMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgba(0x282828F2)
        } else {
            rgba(0xF8F8F8F2)
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(self.width))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(7.0))
            .shadow_lg()
            .py(px(4.0))
            .children(self.items.into_iter().enumerate().map(|(_idx, kind)| {
                match kind {
                    MacMenuItemKind::Separator => div()
                        .h(px(1.0))
                        .bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x00000015) })
                        .my(px(4.0))
                        .mx(px(8.0)),
                    MacMenuItemKind::Header(title) => div()
                        .px(px(10.0))
                        .py(px(2.0))
                        .text_size(px(10.0))
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .text_color(apple_colors.secondary_label)
                        .child(title),
                    MacMenuItemKind::Item {
                        title,
                        icon,
                        shortcut,
                        checked,
                        disabled,
                        destructive,
                    } => {
                        let text_color = if disabled {
                            apple_colors.disabled()
                        } else if destructive {
                            apple_colors.red()
                        } else {
                            apple_colors.label
                        };

                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(22.0))
                            .px(px(8.0))
                            .mx(px(4.0))
                            .rounded(px(4.0))
                            .when(!disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.bg(apple_colors.accent).text_color(rgb(0xFFFFFF)))
                            })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(if checked {
                                                svg()
                                                    .path(IconName::Check.path())
                                                    .size(px(10.0))
                                                    .text_color(text_color)
                                            } else {
                                                svg().size(px(0.0))
                                            }),
                                    )
                                    .when_some(icon, |d, ico| {
                                        d.child(
                                            svg()
                                                .path(ico.path())
                                                .size(px(12.0))
                                                .text_color(text_color),
                                        )
                                    })
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(text_color)
                                            .child(title),
                                    ),
                            )
                            .when_some(shortcut, |d, sc| {
                                d.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(apple_colors.secondary_label)
                                        .child(sc),
                                )
                            })
                    }
                }
            }))
    }
}
