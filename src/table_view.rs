use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacTableColumn {
    pub title: SharedString,
    pub width: f32,
    pub sortable: bool,
}

pub struct MacTableRow {
    pub cells: Vec<SharedString>,
}

/// A macOS Table View adhering strictly to Apple Human Interface Guidelines.
/// Displays tabular data with column headers, sort indicators, alternating
/// zebra row striping, and row selection.
#[derive(IntoElement)]
pub struct MacTableView {
    id: ElementId,
    style: StyleRefinement,
    columns: Vec<MacTableColumn>,
    rows: Vec<MacTableRow>,
    selected_row: Option<usize>,
    zebra_striping: bool,
    on_select_row: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacTableView {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            columns: Vec::new(),
            rows: Vec::new(),
            selected_row: None,
            zebra_striping: true,
            on_select_row: None,
        }
    }

    pub fn column(mut self, title: impl Into<SharedString>, width: f32) -> Self {
        self.columns.push(MacTableColumn {
            title: title.into(),
            width,
            sortable: true,
        });
        self
    }

    pub fn row(mut self, cells: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.rows.push(MacTableRow {
            cells: cells.into_iter().map(Into::into).collect(),
        });
        self
    }

    pub fn selected_row(mut self, index: Option<usize>) -> Self {
        self.selected_row = index;
        self
    }

    pub fn zebra_striping(mut self, striping: bool) -> Self {
        self.zebra_striping = striping;
        self
    }

    pub fn on_select_row(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select_row = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacTableView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacTableView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if is_dark {
            rgb(0x1E1E1E)
        } else {
            rgb(0xFFFFFF)
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
            .w_full()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(6.0))
            .overflow_hidden()
            // Header Row
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(24.0))
                    .bg(if is_dark { rgb(0x282828) } else { rgb(0xF2F2F2) })
                    .border_b_1()
                    .border_color(border_color)
                    .px_2()
                    .children(self.columns.iter().map(|col| {
                        div()
                            .w(px(col.width))
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(apple_colors.secondary_label)
                                    .child(col.title.clone()),
                            )
                            .when(col.sortable, |d| {
                                d.child(
                                    svg()
                                        .path(IconName::ChevronDown.path())
                                        .size(px(8.0))
                                        .text_color(apple_colors.tertiary_label),
                                )
                            })
                    })),
            )
            // Table Rows
            .children(self.rows.into_iter().enumerate().map(|(idx, row)| {
                let is_selected = self.selected_row == Some(idx);
                let is_even = idx % 2 == 0;
                let row_bg: Hsla = if is_selected {
                    apple_colors.accent
                } else if self.zebra_striping && !is_even {
                    if is_dark { rgba(0xFFFFFF08).into() } else { rgba(0x00000005).into() }
                } else {
                    rgba(0x00000000).into()
                };

                let text_color = if is_selected {
                    rgb(0xFFFFFF).into()
                } else {
                    apple_colors.label
                };

                div()
                    .flex()
                    .items_center()
                    .h(px(24.0))
                    .px_2()
                    .bg(row_bg)
                    .cursor_pointer()
                    .hover(|s| {
                        if !is_selected {
                            s.bg(if is_dark { rgba(0xFFFFFF15) } else { rgba(0x0000000A) })
                        } else {
                            s
                        }
                    })
                    .children(row.cells.into_iter().enumerate().map(|(col_idx, cell_val)| {
                        let col_width = self.columns.get(col_idx).map(|c| c.width).unwrap_or(120.0);
                        div()
                            .w(px(col_width))
                            .text_size(px(12.0))
                            .text_color(text_color)
                            .truncate()
                            .child(cell_val)
                    }))
            }))
    }
}
