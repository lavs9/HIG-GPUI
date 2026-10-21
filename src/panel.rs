use std::rc::Rc;
use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacPanelStyle {
    #[default]
    Utility,
    HUD,
}

/// A macOS Panel (Inspector / HUD Palette) adhering strictly to Apple Human Interface Guidelines.
/// Renders a floating tool palette with compact title bar, close button, vibrancy,
/// and embedded inspector controls.
#[derive(IntoElement)]
pub struct MacPanel {
    id: ElementId,
    style: StyleRefinement,
    title: SharedString,
    panel_style: MacPanelStyle,
    width: f32,
    children: Vec<AnyElement>,
    on_close: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MacPanel {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: title.into(),
            panel_style: MacPanelStyle::Utility,
            width: 260.0,
            children: Vec::new(),
            on_close: None,
        }
    }

    pub fn panel_style(mut self, style: MacPanelStyle) -> Self {
        self.panel_style = style;
        self
    }

    pub fn hud(mut self) -> Self {
        self.panel_style = MacPanelStyle::HUD;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacPanel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacPanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let is_hud = self.panel_style == MacPanelStyle::HUD;

        let bg = if is_hud {
            rgba(0x181818E6)
        } else if is_dark {
            rgba(0x282828F0)
        } else {
            rgba(0xF6F6F6F0)
        };

        let border_color = if is_hud {
            rgba(0xFFFFFF33)
        } else if is_dark {
            rgba(0xFFFFFF26)
        } else {
            rgba(0x00000020)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(self.width))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded(px(10.0))
            .shadow_xl()
            .overflow_hidden()
            // Panel Header / Title Bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(26.0))
                    .px_3()
                    .bg(if is_hud { rgba(0x00000040) } else { rgba(0x00000008) })
                    .border_b_1()
                    .border_color(if is_hud { rgba(0xFFFFFF15) } else { rgba(0x00000010) })
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .text_color(if is_hud { rgb(0xFFFFFF).into() } else { apple_colors.label })
                            .child(self.title),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(14.0))
                            .h(px(14.0))
                            .rounded_full()
                            .bg(if is_hud { rgba(0xFFFFFF20) } else { rgba(0x00000015) })
                            .hover(|s| s.bg(if is_hud { rgba(0xFFFFFF40) } else { rgba(0x00000030) }))
                            .cursor_pointer()
                            .child(
                                svg()
                                    .path(IconName::Close.path())
                                    .size(px(7.0))
                                    .text_color(if is_hud { rgb(0xFFFFFF).into() } else { apple_colors.secondary_label }),
                            ),
                    ),
            )
            // Panel Content
            .child(
                div()
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(self.children),
            )
    }
}
