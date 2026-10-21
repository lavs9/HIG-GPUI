use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacSpinnerSize {
    Small,
    #[default]
    Regular,
    Large,
}

impl MacSpinnerSize {
    pub fn size_px(&self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Regular => 24.0,
            Self::Large => 32.0,
        }
    }
}

/// A macOS Progress Bar adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacProgress {
    id: ElementId,
    style: StyleRefinement,
    value: f32, // 0.0 to 1.0
    indeterminate: bool,
    color: Option<Hsla>,
    label: Option<SharedString>,
}

impl MacProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            value: 0.0,
            indeterminate: false,
            color: None,
            label: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Styled for MacProgress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let bar_color = self.color.unwrap_or(apple_colors.accent);

        let track_bg = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x0000001A)
        };

        let progress_frac = self.value.clamp(0.0, 1.0);

        let mut container = div()
            .id(self.id)
            .refine_style(&self.style)
            .w_full()
            .flex()
            .flex_col()
            .gap(px(4.0));

        if let Some(label) = self.label {
            container = container.child(
                div()
                    .mac_caption()
                    .text_color(apple_colors.secondary_label)
                    .child(label),
            );
        }

        let bar = div()
            .w_full()
            .h(px(4.0))
            .rounded(px(2.0))
            .bg(track_bg)
            .relative()
            .overflow_hidden()
            .child(if self.indeterminate {
                // Indeterminate moving shimmer
                div()
                    .absolute()
                    .h_full()
                    .w(gpui_kit::DefiniteLength::Fraction(0.35))
                    .rounded(px(2.0))
                    .bg(bar_color)
            } else {
                div()
                    .h_full()
                    .w(gpui_kit::DefiniteLength::Fraction(progress_frac))
                    .rounded(px(2.0))
                    .bg(bar_color)
            });

        container.child(bar)
    }
}

/// A macOS Circular Activity Indicator (Spinner) adhering to Apple HIG.
#[derive(IntoElement)]
pub struct MacSpinner {
    id: ElementId,
    style: StyleRefinement,
    size: MacSpinnerSize,
    color: Option<Hsla>,
}

impl MacSpinner {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            size: MacSpinnerSize::Regular,
            color: None,
        }
    }

    pub fn size(mut self, size: MacSpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = MacSpinnerSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = MacSpinnerSize::Large;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Styled for MacSpinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSpinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let dim = self.size.size_px();
        let spinner_color = self.color.unwrap_or(apple_colors.secondary_label);

        div()
            .id(self.id)
            .refine_style(&self.style)
            .w(px(dim))
            .h(px(dim))
            .flex()
            .items_center()
            .justify_center()
            .text_color(spinner_color)
            .child(
                svg()
                    .path("icons/loader-2.svg")
                    .size(px(dim))
                    .text_color(spinner_color),
            )
    }
}
