use std::rc::Rc;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement as _, RenderOnce,
    StyleRefinement, Styled, Window, div, px, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};

/// A macOS Rating Indicator adhering strictly to Apple Human Interface Guidelines.
/// Displays a 5-star rating with filled and empty star glyphs, supporting
/// interactive rating selection and read-only display.
#[derive(IntoElement)]
pub struct MacRatingIndicator {
    id: ElementId,
    style: StyleRefinement,
    max_rating: usize,
    rating: usize,
    interactive: bool,
    disabled: bool,
    on_rate: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl MacRatingIndicator {
    pub fn new(id: impl Into<ElementId>, rating: usize) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            max_rating: 5,
            rating: rating.min(5),
            interactive: true,
            disabled: false,
            on_rate: None,
        }
    }

    pub fn max_rating(mut self, max: usize) -> Self {
        self.max_rating = max.max(1);
        self
    }

    pub fn rating(mut self, rating: usize) -> Self {
        self.rating = rating.min(self.max_rating);
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_rate(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_rate = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacRatingIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacRatingIndicator {
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
            .gap(px(3.0))
            .children((1..=self.max_rating).map(|star_idx| {
                let is_filled = star_idx <= self.rating;
                let star_color = if self.disabled {
                    apple_colors.disabled()
                } else if is_filled {
                    apple_colors.yellow()
                } else if is_dark {
                    rgba(0xFFFFFF30).into()
                } else {
                    rgba(0x00000025).into()
                };

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(16.0))
                    .h(px(16.0))
                    .when(self.interactive && !self.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.text_color(apple_colors.yellow()))
                    })
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(star_color)
                            .child(if is_filled { "★" } else { "☆" }),
                    )
            }))
    }
}
