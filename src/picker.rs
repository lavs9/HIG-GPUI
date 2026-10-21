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
        let full_month_names = [
            "January", "February", "March", "April", "May", "June", "July", "August", "September",
            "October", "November", "December",
        ];
        let month_idx = (self.month as usize).saturating_sub(1);
        let month_str = month_names.get(month_idx).unwrap_or(&"Oct");
        let full_month_str = full_month_names.get(month_idx).unwrap_or(&"October");

        let date_str = format!("{} {:02}, {}", month_str, self.day, self.year);
        let time_str = format!("{:02}:{:02}", self.hour, self.minute);

        match self.picker_style {
            MacDatePickerStyle::Graphical => {
                let mut card = div()
                    .id(self.id)
                    .w(px(260.0))
                    .bg(if is_dark { rgba(0x282828F0) } else { rgb(0xFFFFFF).into() })
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(10.0))
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .shadow_lg();

                // Header
                card = card.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                .text_color(apple_colors.label)
                                .child(SharedString::from(format!("{} {}", full_month_str, self.year))),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    svg()
                                        .path(IconName::Calendar.path())
                                        .size(px(12.0))
                                        .text_color(apple_colors.secondary_label),
                                ),
                        ),
                );

                // Weekday Headers
                let weekdays = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
                let mut wd_row = div().flex().justify_between().text_size(px(10.0)).text_color(apple_colors.secondary_label);
                for wd in weekdays {
                    wd_row = wd_row.child(div().w(px(28.0)).text_align(gpui_kit::TextAlign::Center).child(SharedString::from(wd)));
                }
                card = card.child(wd_row);

                // Sample days grid around selected day
                let mut days_grid = div().flex().flex_wrap().gap_1();
                for d in 1..=31 {
                    let is_sel = d == self.day;
                    let day_btn = div()
                        .w(px(28.0))
                        .h(px(24.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(5.0))
                        .text_size(px(11.5))
                        .when(is_sel, |s| {
                            s.bg(apple_colors.accent).text_color(rgb(0xFFFFFF))
                        })
                        .when(!is_sel, |s| {
                            s.text_color(apple_colors.label)
                        })
                        .child(SharedString::from(d.to_string()));
                    days_grid = days_grid.child(day_btn);
                }
                card = card.child(days_grid);

                if self.show_time {
                    card = card.child(
                        div()
                            .pt_2()
                            .border_t_1()
                            .border_color(border_color)
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_size(px(11.0))
                            .text_color(apple_colors.secondary_label)
                            .child(SharedString::from("Time:"))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(svg().path(IconName::Timer.path()).size(px(11.0)))
                                    .child(SharedString::from(time_str)),
                            ),
                    );
                }

                card
            }
            MacDatePickerStyle::Stepper => {
                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .h(px(26.0))
                            .px_2()
                            .bg(bg)
                            .border_1()
                            .border_color(border_color)
                            .rounded(px(6.0))
                            .child(SharedString::from(date_str))
                            .child(
                                svg()
                                    .path(IconName::ChevronDown.path())
                                    .size(px(10.0))
                                    .text_color(apple_colors.secondary_label),
                            ),
                    )
            }
            MacDatePickerStyle::Compact => {
                let mut root = div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .h(px(26.0))
                            .px_2()
                            .bg(bg)
                            .border_1()
                            .border_color(border_color)
                            .rounded(px(6.0))
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
                            )
                            .child(
                                svg()
                                    .path(IconName::ChevronDown.path())
                                    .size(px(9.0))
                                    .text_color(apple_colors.secondary_label),
                            ),
                    );

                if self.show_time {
                    root = root.child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .h(px(26.0))
                            .px_2()
                            .bg(bg)
                            .border_1()
                            .border_color(border_color)
                            .rounded(px(6.0))
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
                    );
                }

                root
            }
        }
    }
}
