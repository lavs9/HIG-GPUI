use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::text_field::MacFieldSize;
use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Combo Box adhering to Apple Human Interface Guidelines.
/// Combines a text field with a drop-down list to let users either enter a custom value
/// or select from a list of predefined choices.
#[derive(IntoElement)]
pub struct MacComboBox {
    id: ElementId,
    style: StyleRefinement,
    value: SharedString,
    placeholder: SharedString,
    options: Vec<SharedString>,
    selected_index: Option<usize>,
    size: MacFieldSize,
    disabled: bool,
    is_open: bool,
    on_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_select: Option<Rc<dyn Fn(usize, &str, &mut Window, &mut App) + 'static>>,
}

impl MacComboBox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            value: "".into(),
            placeholder: "Select or type...".into(),
            options: Vec::new(),
            selected_index: None,
            size: MacFieldSize::Regular,
            disabled: false,
            is_open: false,
            on_change: None,
            on_select: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn option(mut self, option: impl Into<SharedString>) -> Self {
        self.options.push(option.into());
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        if let Some(i) = index {
            if let Some(item) = self.options.get(i) {
                self.value = item.clone();
            }
        }
        self
    }

    pub fn size(mut self, size: MacFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(usize, &str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacComboBox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacComboBox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let height = self.size.height();
        let font_size = self.size.font_size();
        let disabled = self.disabled;

        let bg = if disabled {
            if is_dark { rgba(0xFFFFFF08) } else { rgba(0x00000008) }
        } else if is_dark {
            rgba(0xFFFFFF15)
        } else {
            rgba(0xFFFFFFFF)
        };

        let border_color = if disabled {
            rgba(0x00000010)
        } else if is_dark {
            rgba(0xFFFFFF2E)
        } else {
            rgba(0x00000026)
        };

        let display_text = if !self.value.is_empty() {
            self.value.clone()
        } else {
            self.placeholder.clone()
        };

        let text_color = if self.value.is_empty() {
            apple_colors.placeholder()
        } else if disabled {
            apple_colors.disabled()
        } else {
            apple_colors.label
        };

        let root = div()
            .id(self.id)
            .relative()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(height))
                    .px_2()
                    .bg(bg)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(5.0))
                    .when(!disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.border_color(apple_colors.accent))
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(font_size))
                            .text_color(text_color)
                            .truncate()
                            .child(display_text),
                    )
                    // Down chevron button
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(14.0))
                            .h(px(14.0))
                            .ml_1()
                            .child(
                                svg()
                                    .path(IconName::ChevronDown.path())
                                    .size(px(10.0))
                                    .text_color(if disabled {
                                        apple_colors.disabled()
                                    } else {
                                        apple_colors.secondary_label
                                    }),
                            ),
                    ),
            );

        if self.is_open && !disabled && !self.options.is_empty() {
            root.child(
                div()
                    .absolute()
                    .top(px(height + 3.0))
                    .left_0()
                    .right_0()
                    .bg(if is_dark { rgb(0x282828) } else { rgb(0xFFFFFF) })
                    .border_1()
                    .border_color(if is_dark { rgba(0xFFFFFF26) } else { rgba(0x0000001F) })
                    .rounded(px(6.0))
                    .shadow_md()
                    .py_1()
                    .children(self.options.into_iter().enumerate().map(|(idx, opt)| {
                        let is_selected = self.selected_index == Some(idx);
                        let item_bg: Hsla = if is_selected {
                            apple_colors.accent
                        } else {
                            rgba(0x00000000).into()
                        };

                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_3()
                            .py(px(4.0))
                            .text_size(px(12.0))
                            .text_color(if is_selected {
                                rgb(0xFFFFFF).into()
                            } else {
                                apple_colors.label
                            })
                            .bg(item_bg)
                            .rounded(px(4.0))
                            .mx_1()
                            .hover(|s| {
                                if !is_selected {
                                    s.bg(if is_dark { rgba(0xFFFFFF1F) } else { rgba(0x0000000F) })
                                } else {
                                    s
                                }
                            })
                            .child(opt)
                    })),
            )
        } else {
            root
        }
    }
}
