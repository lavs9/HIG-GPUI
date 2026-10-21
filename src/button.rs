use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacButtonVariant {
    #[default]
    Push,
    Prominent,
    Borderless,
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacButtonSize {
    Small,
    #[default]
    Regular,
    Large,
}

impl MacButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            Self::Small => 20.0,
            Self::Regular => 24.0,
            Self::Large => 28.0,
        }
    }

    pub fn padding_x(&self) -> f32 {
        match self {
            Self::Small => 8.0,
            Self::Regular => 12.0,
            Self::Large => 16.0,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            Self::Small => 11.0,
            Self::Regular => 13.0,
            Self::Large => 14.0,
        }
    }
}

/// A macOS Push Button adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacButton {
    id: ElementId,
    style: StyleRefinement,
    variant: MacButtonVariant,
    size: MacButtonSize,
    label: Option<SharedString>,
    icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    selected: bool,
    tooltip: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            variant: MacButtonVariant::Push,
            size: MacButtonSize::Regular,
            label: None,
            icon: None,
            trailing_icon: None,
            disabled: false,
            selected: false,
            tooltip: None,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn variant(mut self, variant: MacButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn prominent(mut self) -> Self {
        self.variant = MacButtonVariant::Prominent;
        self
    }

    pub fn borderless(mut self) -> Self {
        self.variant = MacButtonVariant::Borderless;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.variant = MacButtonVariant::Destructive;
        self
    }

    pub fn size(mut self, size: MacButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = MacButtonSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = MacButtonSize::Large;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let height = self.size.height();
        let pad_x = if self.label.is_some() {
            self.size.padding_x()
        } else {
            (height - 14.0) / 2.0
        };
        let font_size = self.size.font_size();
        let disabled = self.disabled;

        let (bg, border_color, text_color, hover_bg) = match self.variant {
            MacButtonVariant::Prominent => (
                apple_colors.accent,
                apple_colors.accent,
                rgb(0xFFFFFF).into(),
                apple_colors.accent_hover,
            ),
            MacButtonVariant::Push => {
                if is_dark {
                    (
                        rgba(0xFFFFFF1F).into(),
                        rgba(0xFFFFFF2E).into(),
                        apple_colors.label,
                        rgba(0xFFFFFF2E).into(),
                    )
                } else {
                    (
                        rgb(0xFFFFFF).into(),
                        rgba(0x00000026).into(),
                        apple_colors.label,
                        rgb(0xF5F5F7).into(),
                    )
                }
            }
            MacButtonVariant::Borderless => {
                let text_c = if self.selected {
                    apple_colors.accent
                } else {
                    apple_colors.label
                };
                let sel_bg = if self.selected {
                    apple_colors.accent.opacity(0.12)
                } else {
                    rgba(0x00000000).into()
                };
                let hov_bg = if is_dark {
                    rgba(0xFFFFFF14).into()
                } else {
                    rgba(0x0000000F).into()
                };
                (sel_bg, rgba(0x00000000).into(), text_c, hov_bg)
            }
            MacButtonVariant::Destructive => (
                apple_colors.system_red.opacity(0.12),
                apple_colors.system_red.opacity(0.30),
                apple_colors.system_red,
                apple_colors.system_red.opacity(0.20),
            ),
        };

        let on_click = self.on_click.clone();

        div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(height))
            .px(px(pad_x))
            .rounded(px(5.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(5.0))
            .cursor_pointer()
            .when(self.variant == MacButtonVariant::Push && !is_dark, |this| {
                this.shadow_sm()
            })
            .when(disabled, |this| this.opacity(0.45).cursor_not_allowed())
            .when(!disabled, |this| {
                this.hover(|style| style.bg(hover_bg))
                    .on_click(move |event: &ClickEvent, window, cx| {
                        if let Some(ref handler) = on_click {
                            handler(event, window, cx);
                        }
                    })
            })
            .children(self.icon.map(|icon| {
                svg()
                    .path(icon.path())
                    .size(px(font_size + 1.0))
                    .text_color(text_color)
            }))
            .children(self.label.map(|label| {
                div()
                    .mac_body()
                    .text_size(px(font_size))
                    .text_color(text_color)
                    .child(label)
            }))
            .children(self.trailing_icon.map(|icon| {
                svg()
                    .path(icon.path())
                    .size(px(font_size + 1.0))
                    .text_color(text_color)
            }))
    }
}
