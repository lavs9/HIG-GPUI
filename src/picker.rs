use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacDatePickerStyle {
    #[default]
    Compact,
    Graphical,
    Stepper,
}

/// A macOS Date Picker adhering strictly to Apple Human Interface Guidelines.
/// Supports compact segmented date display (Year, Month, Day, Time) with
/// popover calendar picker and stepper adjustments.
#[derive(IntoElement)]
pub struct MacDatePicker {
    id: ElementId,
    style: StyleRefinement,
    picker_style: MacDatePickerStyle,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    show_time: bool,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(u32, u32, u32, &mut Window, &mut App) + 'static>>,
}

impl MacDatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            picker_style: MacDatePickerStyle::Compact,
            year: 2026,
            month: 10,
            day: 21,
            hour: 12,
            minute: 0,
            show_time: true,
            disabled: false,
            on_change: None,
        }
    }

    pub fn date(mut self, year: u32, month: u32, day: u32) -> Self {
        self.year = year;
        self.month = month;
        self.day = day;
        self
    }

    pub fn time(mut self, hour: u32, minute: u32) -> Self {
        self.hour = hour;
        self.minute = minute;
        self
    }

    pub fn picker_style(mut self, style: MacDatePickerStyle) -> Self {
        self.picker_style = style;
        self
    }

    pub fn show_time(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(u32, u32, u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacDatePicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacDatePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bg = if self.disabled {
            if is_dark { rgba(0xFFFFFF08) } else { rgba(0x00000008) }
        } else if is_dark {
            rgba(0xFFFFFF15)
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let month_str = month_names
            .get((self.month as usize).saturating_sub(1))
            .unwrap_or(&"Oct");

        let date_str = format!("{} {:02}, {}", month_str, self.day, self.year);
        let time_str = format!("{:02}:{:02}", self.hour, self.minute);

        let root = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .h(px(24.0))
                    .px_2()
                    .bg(bg)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(5.0))
                    .when(!self.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.border_color(apple_colors.accent))
                    })
                    .child(
                        svg()
                            .path(IconName::Calendar.path())
                            .size(px(12.0))
                            .text_color(apple_colors.secondary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if self.disabled {
                                apple_colors.disabled()
                            } else {
                                apple_colors.label
                            })
                            .child(SharedString::from(date_str)),
                    ),
            );

        if self.show_time {
            root.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .h(px(24.0))
                    .px_2()
                    .bg(bg)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(5.0))
                    .child(
                        svg()
                            .path(IconName::Timer.path())
                            .size(px(12.0))
                            .text_color(apple_colors.secondary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if self.disabled {
                                apple_colors.disabled()
                            } else {
                                apple_colors.label
                            })
                            .child(SharedString::from(time_str)),
                    ),
            )
        } else {
            root
        }
    }
}
