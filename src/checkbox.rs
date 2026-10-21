use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::{StyledExt, spring};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacCheckboxState {
    #[default]
    Unchecked,
    Checked,
    Mixed,
}

impl From<bool> for MacCheckboxState {
    fn from(b: bool) -> Self {
        if b { Self::Checked } else { Self::Unchecked }
    }
}

/// A macOS Checkbox adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacCheckbox {
    id: ElementId,
    style: StyleRefinement,
    state: MacCheckboxState,
    disabled: bool,
    label: Option<SharedString>,
    description: Option<SharedString>,
    on_change: Option<Rc<dyn Fn(&MacCheckboxState, &mut Window, &mut App) + 'static>>,
}

impl MacCheckbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            state: MacCheckboxState::Unchecked,
            disabled: false,
            label: None,
            description: None,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.state = if checked {
            MacCheckboxState::Checked
        } else {
            MacCheckboxState::Unchecked
        };
        self
    }

    pub fn state(mut self, state: MacCheckboxState) -> Self {
        self.state = state;
        self
    }

    pub fn mixed(mut self) -> Self {
        self.state = MacCheckboxState::Mixed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn on_change(
        mut self,
        handler: impl Fn(&MacCheckboxState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(move |state, window, cx| {
            let next_bool = matches!(state, MacCheckboxState::Checked);
            handler(&next_bool, window, cx);
        }));
        self
    }
}

impl Styled for MacCheckbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacCheckbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let state = self.state;
        let is_active = matches!(state, MacCheckboxState::Checked | MacCheckboxState::Mixed);
        let disabled = self.disabled;

        let bg_color: Hsla = if is_active {
            apple_colors.accent
        } else if is_dark {
            rgba(0xFFFFFF1F).into()
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color: Hsla = if is_active {
            apple_colors.accent
        } else if is_dark {
            rgba(0xFFFFFF4D).into()
        } else {
            rgba(0x0000003D).into()
        };

        let icon_opacity = spring(
            (self.id.clone(), "mark_opacity"),
            if is_active { 1.0 } else { 0.0 },
            cx.theme().motion_tokens().spring_control,
            window,
            cx,
        );

        let on_change = self.on_change.clone();

        let icon_element = if matches!(state, MacCheckboxState::Mixed) {
            svg()
                .path(IconName::Minus.path())
                .size(px(10.0))
                .text_color(rgb(0xFFFFFF))
                .opacity(icon_opacity)
        } else {
            svg()
                .path(IconName::Check.path())
                .size(px(10.0))
                .text_color(rgb(0xFFFFFF))
                .opacity(icon_opacity)
        };

        let box_element = div()
            .w(px(14.0))
            .h(px(14.0))
            .rounded(px(3.5))
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .child(icon_element);

        div()
            .id(self.id)
            .refine_style(&self.style)
            .flex()
            .items_start()
            .gap_2()
            .when(disabled, |this| this.opacity(0.45).cursor_not_allowed())
            .when(!disabled, |this| {
                this.cursor_pointer().on_click(move |_: &ClickEvent, window, cx| {
                    if let Some(ref handler) = on_change {
                        let next = match state {
                            MacCheckboxState::Unchecked => MacCheckboxState::Checked,
                            MacCheckboxState::Checked => MacCheckboxState::Unchecked,
                            MacCheckboxState::Mixed => MacCheckboxState::Checked,
                        };
                        handler(&next, window, cx);
                    }
                })
            })
            .child(div().pt(px(1.0)).child(box_element))
            .children((self.label.is_some() || self.description.is_some()).then(|| {
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
                    })
            }))
    }
}
