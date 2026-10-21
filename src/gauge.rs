use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacGaugeStyle {
    #[default]
    Linear,
    Capacity,
    Stepped,
}

/// A macOS Gauge adhering strictly to Apple Human Interface Guidelines.
/// Displays a quantity within a range with color-coded warning/critical thresholds.
#[derive(IntoElement)]
pub struct MacGauge {
    id: ElementId,
    style: StyleRefinement,
    value: f32, // 0.0 to 1.0
    label: Option<SharedString>,
    value_label: Option<SharedString>,
    gauge_style: MacGaugeStyle,
    warning_threshold: f32,
    critical_threshold: f32,
    width: f32,
}

impl MacGauge {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            value: value.clamp(0.0, 1.0),
            label: None,
            value_label: None,
            gauge_style: MacGaugeStyle::Linear,
            warning_threshold: 0.75,
            critical_threshold: 0.90,
            width: 180.0,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value_label(mut self, text: impl Into<SharedString>) -> Self {
        self.value_label = Some(text.into());
        self
    }

    pub fn gauge_style(mut self, style: MacGaugeStyle) -> Self {
        self.gauge_style = style;
        self
    }

    pub fn thresholds(mut self, warning: f32, critical: f32) -> Self {
        self.warning_threshold = warning;
        self.critical_threshold = critical;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Styled for MacGauge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacGauge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let fill_color = if self.value >= self.critical_threshold {
            apple_colors.red()
        } else if self.value >= self.warning_threshold {
            apple_colors.orange()
        } else {
            apple_colors.accent
        };

        let track_bg = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x00000015)
        };

        let fill_percent = (self.value * 100.0).round();
        let display_value = self.value_label.unwrap_or_else(|| {
            SharedString::from(format!("{}%", fill_percent as i32))
        });

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_1()
            .w(px(self.width))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_size(px(11.0))
                    .child(
                        div()
                            .text_color(apple_colors.secondary_label)
                            .child(self.label.unwrap_or_else(|| "Capacity".into())),
                    )
                    .child(
                        div()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(apple_colors.label)
                            .child(display_value),
                    ),
            )
            // Gauge Bar Track
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(8.0))
                    .bg(track_bg)
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(px(self.width * self.value))
                            .bg(fill_color)
                            .rounded_full(),
                    ),
            )
    }
}
