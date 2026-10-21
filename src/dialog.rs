use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px, rgb, rgba, svg,
};

use crate::button::{MacButton, MacButtonVariant};
use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacDialogVariant {
    #[default]
    Informational,
    Warning,
    Critical,
}

/// A macOS Alert Dialog adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacDialog {
    id: ElementId,
    style: StyleRefinement,
    variant: MacDialogVariant,
    icon: Option<IconName>,
    title: SharedString,
    message: Option<SharedString>,
    primary_label: SharedString,
    secondary_label: Option<SharedString>,
    destructive: bool,
    children: Vec<AnyElement>,
    on_primary: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_secondary: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacDialog {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            variant: MacDialogVariant::Informational,
            icon: None,
            title: title.into(),
            message: None,
            primary_label: "OK".into(),
            secondary_label: Some("Cancel".into()),
            destructive: false,
            children: Vec::new(),
            on_primary: None,
            on_secondary: None,
        }
    }

    pub fn variant(mut self, variant: MacDialogVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = MacDialogVariant::Warning;
        self
    }

    pub fn critical(mut self) -> Self {
        self.variant = MacDialogVariant::Critical;
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn primary_label(mut self, label: impl Into<SharedString>) -> Self {
        self.primary_label = label.into();
        self
    }

    pub fn secondary_label(mut self, label: Option<SharedString>) -> Self {
        self.secondary_label = label;
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn on_primary(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_primary = Some(Rc::new(handler));
        self
    }

    pub fn on_secondary(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_secondary = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacDialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let card_bg: Hsla = if is_dark {
            rgb(0x282828).into()
        } else {
            rgb(0xFFFFFF).into()
        };

        let border_color = if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x0000001F)
        };

        let icon_name = self.icon.unwrap_or(match self.variant {
            MacDialogVariant::Informational => IconName::Info,
            MacDialogVariant::Warning => IconName::TriangleAlert,
            MacDialogVariant::Critical => IconName::OctagonAlert,
        });

        let icon_color = match self.variant {
            MacDialogVariant::Informational => apple_colors.accent,
            MacDialogVariant::Warning => apple_colors.system_orange,
            MacDialogVariant::Critical => apple_colors.system_red,
        };

        let on_primary = self.on_primary;
        let on_secondary = self.on_secondary;

        let primary_btn_variant = if self.destructive {
            MacButtonVariant::Destructive
        } else {
            MacButtonVariant::Prominent
        };

        // Scrim backdrop
        div()
            .id(self.id)
            .refine_style(&self.style)
            .w_full()
            .h_full()
            .bg(rgba(0x0000004D)) // 30% black dim
            .flex()
            .items_center()
            .justify_center()
            .child(
                // Native macOS Alert dialog card
                div()
                    .w(px(280.0))
                    .p(px(20.0))
                    .rounded(px(14.0))
                    .bg(card_bg)
                    .border_1()
                    .border_color(border_color)
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(14.0))
                    .child(
                        // App / Alert icon
                        div()
                            .w(px(48.0))
                            .h(px(48.0))
                            .rounded(px(10.0))
                            .bg(icon_color.opacity(0.12))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path(icon_name.path())
                                    .size(px(26.0))
                                    .text_color(icon_color),
                            ),
                    )
                    .child(
                        // Title
                        div()
                            .mac_headline()
                            .text_color(apple_colors.label)
                            .text_center()
                            .child(self.title),
                    )
                    .children(self.message.map(|msg| {
                        div()
                            .mac_subheadline()
                            .text_color(apple_colors.secondary_label)
                            .text_center()
                            .child(msg)
                    }))
                    .children(self.children)
                    .child(
                        // Action buttons bar
                        div()
                            .w_full()
                            .pt(px(6.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .children(self.secondary_label.map(|label| {
                                let mut btn = MacButton::new("dialog-cancel")
                                    .label(label)
                                    .flex_1();
                                if let Some(handler) = on_secondary {
                                    btn = btn.on_click(move |ev, window, cx| handler(ev, window, cx));
                                }
                                btn
                            }))
                            .child({
                                let mut btn = MacButton::new("dialog-confirm")
                                    .label(self.primary_label)
                                    .variant(primary_btn_variant)
                                    .flex_1();
                                if let Some(handler) = on_primary {
                                    btn = btn.on_click(move |ev, window, cx| handler(ev, window, cx));
                                }
                                btn
                            }),
                    ),
            )
    }
}
