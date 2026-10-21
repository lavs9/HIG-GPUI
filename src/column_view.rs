use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacColumnEntry {
    pub title: SharedString,
    pub is_folder: bool,
    pub is_selected: bool,
}

pub struct MacColumnData {
    pub entries: Vec<MacColumnEntry>,
}

/// A macOS Column View adhering strictly to Apple Human Interface Guidelines.
/// Displays hierarchical content across multiple side-by-side Miller columns
/// with chevron indicators for sub-folders.
#[derive(IntoElement)]
pub struct MacColumnView {
    id: ElementId,
    style: StyleRefinement,
    columns: Vec<MacColumnData>,
    column_width: f32,
}

impl MacColumnView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            columns: Vec::new(),
            column_width: 180.0,
        }
    }

    pub fn column_width(mut self, width: f32) -> Self {
        self.column_width = width;
        self
    }

    pub fn column(mut self, entries: Vec<MacColumnEntry>) -> Self {
        self.columns.push(MacColumnData { entries });
        self
    }
}

impl Styled for MacColumnView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacColumnView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let divider_color = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x00000015)
        };

        div()
            .id(self.id)
            .flex()
            .h_full()
            .border_1()
            .border_color(divider_color)
            .rounded(px(6.0))
            .overflow_hidden()
            .children(self.columns.into_iter().enumerate().map(|(_col_idx, col)| {
                div()
                    .flex()
                    .h_full()
                    .child(
                        div()
                            .w(px(self.column_width))
                            .h_full()
                            .p_1()
                            .children(col.entries.into_iter().map(|entry| {
                                let entry_bg: Hsla = if entry.is_selected {
                                    apple_colors.accent
                                } else {
                                    rgba(0x00000000).into()
                                };

                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .h(px(22.0))
                                    .px_2()
                                    .rounded(px(4.0))
                                    .bg(entry_bg)
                                    .cursor_pointer()
                                    .hover(|s| {
                                        if !entry.is_selected {
                                            s.bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x0000000A) })
                                        } else {
                                            s
                                        }
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child(
                                                svg()
                                                    .path(if entry.is_folder {
                                                        IconName::Folder.path()
                                                    } else {
                                                        IconName::File.path()
                                                    })
                                                    .size(px(12.0))
                                                    .text_color(if entry.is_selected {
                                                        rgb(0xFFFFFF).into()
                                                    } else {
                                                        apple_colors.accent
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(if entry.is_selected {
                                                        rgb(0xFFFFFF).into()
                                                    } else {
                                                        apple_colors.label
                                                    })
                                                    .child(entry.title),
                                            ),
                                    )
                                    .when(entry.is_folder, |d| {
                                        d.child(
                                            svg()
                                                .path(IconName::ChevronRight.path())
                                                .size(px(8.0))
                                                .text_color(if entry.is_selected {
                                                    rgb(0xFFFFFF).into()
                                                } else {
                                                    apple_colors.tertiary_label
                                                }),
                                        )
                                    })
                            })),
                    )
                    .child(
                        div()
                            .w(px(1.0))
                            .h_full()
                            .bg(divider_color),
                    )
            }))
    }
}
