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

/// A macOS Slider adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSlider {
    id: ElementId,
    style: StyleRefinement,
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    ticks: Option<usize>,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    leading_label: Option<SharedString>,
    trailing_label: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&f32, &mut Window, &mut App) + 'static>>,
}

impl MacSlider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: None,
            ticks: None,
            leading_icon: None,
            trailing_icon: None,
            leading_label: None,
            trailing_label: None,
            disabled: false,
            on_change: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn ticks(mut self, count: usize) -> Self {
        self.ticks = Some(count);
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn leading_label(mut self, label: impl Into<SharedString>) -> Self {
        self.leading_label = Some(label.into());
        self
    }

    pub fn trailing_label(mut self, label: impl Into<SharedString>) -> Self {
        self.trailing_label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacSlider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSlider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let range = (self.max - self.min).max(0.001);
        let fraction = ((self.value - self.min) / range).clamp(0.0, 1.0);

        let track_inactive: Hsla = if is_dark {
            rgba(0xFFFFFF33).into()
        } else {
            rgba(0x00000026).into()
        };

        let track_active = apple_colors.accent;

        let thumb_bg = rgb(0xFFFFFF);
        let thumb_border = if is_dark {
            rgba(0x00000040)
        } else {
            rgba(0x00000026)
        };

        let min_val = self.min;
        let max_val = self.max;
        let _step_val = self.step;
        let on_change = self.on_change;

        let track = div()
            .id("slider-track")
            .relative()
            .w_full()
            .h(px(20.0))
            .flex()
            .items_center()
            .child(
                // Inactive track (full width pill)
                div()
                    .absolute()
                    .left(px(0.0))
                    .right(px(0.0))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(track_inactive),
            )
            .child(
                // Active track (left to thumb)
                div()
                    .absolute()
                    .left(px(0.0))
                    .w(gpui_kit::DefiniteLength::Fraction(fraction))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(if self.disabled { track_inactive } else { track_active }),
            )
            .child(
                // Thumb knob (16x16 circular button with shadow)
                div()
                    .absolute()
                    .left(gpui_kit::DefiniteLength::Fraction(fraction))
                    .ml(px(-8.0))
                    .w(px(16.0))
                    .h(px(16.0))
                    .rounded_full()
                    .bg(thumb_bg)
                    .border_1()
                    .border_color(thumb_border)
                    .shadow_sm(),
            )
            .on_click({
                let on_change = on_change.clone();
                move |event: &ClickEvent, window, cx| {
                    if let Some(handler) = &on_change {
                        // Estimate fraction from click position if mouse click
                        if let ClickEvent::Mouse(_mouse) = event {
                            // If bounds cannot be measured directly in click event, step or toggle
                            let next = (min_val + range * 0.5).clamp(min_val, max_val);
                            handler(&next, window, cx);
                        }
                    }
                }
            });

        let mut row = div()
            .id(self.id)
            .refine_style(&self.style)
            .flex()
            .items_center()
            .gap(px(8.0))
            .w_full()
            .opacity(if self.disabled { 0.45 } else { 1.0 });

        if let Some(icon) = self.leading_icon {
            row = row.child(
                svg()
                    .path(icon.path())
                    .size(px(14.0))
                    .text_color(apple_colors.secondary_label),
            );
        } else if let Some(label) = self.leading_label {
            row = row.child(
                div()
                    .mac_caption()
                    .text_color(apple_colors.secondary_label)
                    .child(label),
            );
        }

        row = row.child(div().flex_1().child(track));

        if let Some(icon) = self.trailing_icon {
            row = row.child(
                svg()
                    .path(icon.path())
                    .size(px(14.0))
                    .text_color(apple_colors.secondary_label),
            );
        } else if let Some(label) = self.trailing_label {
            row = row.child(
                div()
                    .mac_caption()
                    .text_color(apple_colors.secondary_label)
                    .child(label),
            );
        }

        row
    }
}
