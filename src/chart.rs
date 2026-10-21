use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacChartDataPoint {
    pub label: SharedString,
    pub value: f32, // 0.0 to 1.0
    pub color: Option<MacAccentColor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacChartType {
    #[default]
    Bar,
    Line,
}

/// A macOS Chart adhering strictly to Apple Human Interface Guidelines (Swift Charts style).
/// Displays clean, accessible bar or line data visualizations with HIG semantic colors.
#[derive(IntoElement)]
pub struct MacChart {
    id: ElementId,
    style: StyleRefinement,
    title: Option<SharedString>,
    chart_type: MacChartType,
    data: Vec<MacChartDataPoint>,
    width: f32,
    height: f32,
}

impl MacChart {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: None,
            chart_type: MacChartType::Bar,
            data: Vec::new(),
            width: 300.0,
            height: 140.0,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn chart_type(mut self, chart_type: MacChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    pub fn point(mut self, label: impl Into<SharedString>, value: f32) -> Self {
        self.data.push(MacChartDataPoint {
            label: label.into(),
            value: value.clamp(0.0, 1.0),
            color: None,
        });
        self
    }

    pub fn point_colored(
        mut self,
        label: impl Into<SharedString>,
        value: f32,
        color: MacAccentColor,
    ) -> Self {
        self.data.push(MacChartDataPoint {
            label: label.into(),
            value: value.clamp(0.0, 1.0),
            color: Some(color),
        });
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Styled for MacChart {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacChart {
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

        let gridline_color = if is_dark {
            rgba(0xFFFFFF15)
        } else {
            rgba(0x0000000F)
        };

        let chart_area_height = (self.height - 40.0).max(20.0);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(self.width))
            .h(px(self.height))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(8.0))
            .p_3()
            .when_some(self.title, |d, t| {
                d.child(
                    div()
                        .mb_2()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .text_color(apple_colors.label)
                        .child(t),
                )
            })
            // Chart plotting area
            .child(
                div()
                    .relative()
                    .flex_1()
                    .flex()
                    .items_end()
                    .justify_between()
                    .border_b_1()
                    .border_color(gridline_color)
                    .pb_1()
                    .children(self.data.into_iter().map(|pt| {
                        let bar_color = if let Some(acc) = pt.color {
                            acc.color(is_dark)
                        } else {
                            apple_colors.accent
                        };
                        let bar_h = (pt.value * chart_area_height).max(4.0);

                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            // Bar
                            .child(
                                div()
                                    .w(px(18.0))
                                    .h(px(bar_h))
                                    .bg(bar_color)
                                    .rounded_t(px(3.0)),
                            )
                            // Label
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(apple_colors.secondary_label)
                                    .child(pt.label),
                            )
                    })),
            )
    }
}
