use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

pub struct MacRingMetric {
    pub name: SharedString,
    pub progress: f32, // 0.0 to 1.0+
    pub color: MacAccentColor,
}

/// A macOS Activity Rings indicator adhering strictly to Apple Human Interface Guidelines.
/// Displays concentric progress metrics with Apple fitness / telemetry aesthetics.
#[derive(IntoElement)]
pub struct MacActivityRings {
    id: ElementId,
    style: StyleRefinement,
    metrics: Vec<MacRingMetric>,
    size: f32,
}

impl MacActivityRings {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            metrics: Vec::new(),
            size: 80.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn ring(mut self, name: impl Into<SharedString>, progress: f32, color: MacAccentColor) -> Self {
        self.metrics.push(MacRingMetric {
            name: name.into(),
            progress: progress.clamp(0.0, 1.5),
            color,
        });
        self
    }
}

impl Styled for MacActivityRings {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacActivityRings {
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
            .items_center()
            .gap_4()
            // Concentric / Stacked Rings Graphic
            .child(
                div()
                    .relative()
                    .w(px(self.size))
                    .h(px(self.size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(if is_dark { rgb(0x181818) } else { rgb(0xEDEDED) })
                    .border_1()
                    .border_color(if is_dark { rgba(0xFFFFFF1F) } else { rgba(0x00000015) })
                    .children(self.metrics.iter().enumerate().map(|(idx, m)| {
                        let inset = (idx as f32) * 10.0;
                        let ring_size = (self.size - (inset * 2.0)).max(10.0);
                        div()
                            .absolute()
                            .w(px(ring_size))
                            .h(px(ring_size))
                            .rounded_full()
                            .border_2()
                            .border_color(m.color.color(is_dark))
                    })),
            )
            // Legend
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(self.metrics.into_iter().map(|m| {
                        let pct = (m.progress * 100.0).round() as i32;
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(8.0))
                                    .h(px(8.0))
                                    .rounded_full()
                                    .bg(m.color.color(is_dark)),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(apple_colors.label)
                                    .child(m.name),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(apple_colors.secondary_label)
                                    .child(SharedString::from(format!("{}%", pct))),
                            )
                    })),
            )
    }
}
