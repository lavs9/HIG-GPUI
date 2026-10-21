use std::rc::Rc;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// A macOS Collapsible Disclosure Control adhering strictly to Apple HIG.
#[derive(IntoElement)]
pub struct MacDisclosure {
    id: ElementId,
    style: StyleRefinement,
    title: SharedString,
    expanded: bool,
    children: Vec<AnyElement>,
    on_toggle: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl MacDisclosure {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: title.into(),
            expanded: false,
            children: Vec::new(),
            on_toggle: None,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacDisclosure {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacDisclosure {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let is_expanded = self.expanded;
        let new_state = !is_expanded;
        let on_toggle = self.on_toggle;

        let icon_path = if is_expanded {
            "icons/chevron-down.svg"
        } else {
            "icons/chevron-right.svg"
        };

        let mut header = div()
            .id("disclosure-header")
            .w_full()
            .py(px(4.0))
            .flex()
            .items_center()
            .gap(px(6.0))
            .cursor_pointer()
            .hover(|s| s.text_color(apple_colors.accent))
            .child(
                svg()
                    .path(icon_path)
                    .size(px(12.0))
                    .text_color(apple_colors.secondary_label),
            )
            .child(
                div()
                    .mac_headline()
                    .text_color(apple_colors.label)
                    .child(self.title),
            );

        if let Some(handler) = on_toggle {
            header = header.on_click(move |_ev, window, cx| {
                handler(&new_state, window, cx);
            });
        }

        let mut container = div()
            .id(self.id)
            .refine_style(&self.style)
            .w_full()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(header);

        if is_expanded {
            let body = div()
                .id("disclosure-content")
                .w_full()
                .pl(px(18.0))
                .pt(px(4.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .children(self.children);

            container = container.child(body);
        }

        container
    }
}
