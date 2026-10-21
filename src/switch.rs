use std::rc::Rc;
use gpui_kit::base::{StyledExt, spring};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, rgb, rgba,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacSwitchSize {
    #[default]
    Regular,
    Mini,
}

impl MacSwitchSize {
    pub fn dimensions(&self) -> (f32, f32, f32, f32) {
        match self {
            Self::Regular => (38.0, 22.0, 18.0, 2.0),
            Self::Mini => (28.0, 16.0, 12.0, 2.0),
        }
    }
}

/// A macOS Toggle Switch adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSwitch {
    id: ElementId,
    style: StyleRefinement,
    checked: bool,
    disabled: bool,
    size: MacSwitchSize,
    color: Option<Hsla>,
    label: Option<SharedString>,
    description: Option<SharedString>,
    label_trailing: bool,
    on_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl MacSwitch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            checked: false,
            disabled: false,
            size: MacSwitchSize::Regular,
            color: None,
            label: None,
            description: None,
            label_trailing: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: MacSwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn mini(mut self) -> Self {
        self.size = MacSwitchSize::Mini;
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

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn label_trailing(mut self, trailing: bool) -> Self {
        self.label_trailing = trailing;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacSwitch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSwitch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (track_w, track_h, thumb_size, inset) = self.size.dimensions();
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let checked = self.checked;
        let disabled = self.disabled;

        let active_bg: Hsla = self.color.unwrap_or(apple_colors.switch_green);
        let inactive_bg: Hsla = if is_dark {
            rgba(0xFFFFFF3D).into()
        } else {
            rgba(0x00000029).into()
        };

        let border_color: Hsla = if checked {
            active_bg.opacity(0.8)
        } else if is_dark {
            rgba(0xFFFFFF2E).into()
        } else {
            rgba(0x0000001F).into()
        };

        let target_x = if checked {
            track_w - thumb_size - inset * 2.0
        } else {
            0.0
        };

        let thumb_x = spring(
            (self.id.clone(), "thumb_x"),
            px(target_x),
            cx.theme().motion_tokens().spring_move,
            window,
            cx,
        );

        let on_change = self.on_change.clone();
        let switch_id = self.id.clone();

        let track = div()
            .id((switch_id.clone(), "track"))
            .w(px(track_w))
            .h(px(track_h))
            .rounded(px(track_h / 2.0))
            .bg(if checked { active_bg } else { inactive_bg })
            .border_1()
            .border_color(border_color)
            .p(px(inset))
            .flex_shrink_0()
            .relative()
            .cursor_pointer()
            .child(
                div()
                    .w(px(thumb_size))
                    .h(px(thumb_size))
                    .rounded(px(thumb_size / 2.0))
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgba(0x0000000D))
                    .shadow_sm()
                    .left(thumb_x)
                    .top(px(0.0))
                    .absolute(),
            );

        let interactive_wrapper = div()
            .id(switch_id)
            .refine_style(&self.style)
            .flex()
            .items_center()
            .gap_2()
            .when(disabled, |this| this.opacity(0.45).cursor_not_allowed())
            .when(!disabled, |this| {
                this.cursor_pointer().on_click(move |_: &ClickEvent, window, cx| {
                    if let Some(ref handler) = on_change {
                        let next = !checked;
                        handler(&next, window, cx);
                    }
                })
            });

        let label_block = if self.label.is_some() || self.description.is_some() {
            Some(
                div()
                    .flex()
                    .flex_col()
                    .when_some(self.label, |this, label| {
                        this.child(
                            div()
                                .mac_body()
                                .text_color(apple_colors.label)
                                .child(label),
                        )
                    })
                    .when_some(self.description, |this, desc| {
                        this.child(
                            div()
                                .mac_subheadline()
                                .text_color(apple_colors.secondary_label)
                                .child(desc),
                        )
                    }),
            )
        } else {
            None
        };

        if self.label_trailing {
            interactive_wrapper.child(track).children(label_block)
        } else if let Some(labels) = label_block {
            interactive_wrapper.child(labels).child(track)
        } else {
            interactive_wrapper.child(track)
        }
    }
}
