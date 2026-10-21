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

pub struct MacSegment {
    pub id: ElementId,
    pub label: Option<SharedString>,
    pub icon: Option<IconName>,
    pub tooltip: Option<SharedString>,
}

impl MacSegment {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon: None,
            tooltip: None,
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

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

/// A macOS Segmented Control adhering strictly to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSegmentedControl {
    id: ElementId,
    style: StyleRefinement,
    segments: Vec<MacSegment>,
    selected_index: usize,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl MacSegmentedControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            segments: Vec::new(),
            selected_index: 0,
            disabled: false,
            on_change: None,
        }
    }

    pub fn segment(mut self, seg: MacSegment) -> Self {
        self.segments.push(seg);
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for MacSegmentedControl {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSegmentedControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let track_bg = if is_dark {
            rgba(0xFFFFFF17) // 9% white
        } else {
            rgba(0x0000000F) // 6% black
        };

        let card_bg = if is_dark {
            rgba(0xFFFFFF29) // 16% white card
        } else {
            rgb(0xFFFFFF).into()
        };

        let card_shadow_color = if is_dark {
            rgba(0x00000033)
        } else {
            rgba(0x0000001F)
        };

        let divider_color = if is_dark {
            rgba(0xFFFFFF1F)
        } else {
            rgba(0x0000001A)
        };

        let selected = self.selected_index;
        let count = self.segments.len();
        let disabled = self.disabled;

        let on_change = self.on_change.clone();

        let mut row = div()
            .id(self.id)
            .refine_style(&self.style)
            .h(px(26.0))
            .p(px(2.0))
            .rounded(px(6.0))
            .bg(track_bg)
            .border_1()
            .border_color(if is_dark { rgba(0xFFFFFF14) } else { rgba(0x0000000D) })
            .flex()
            .items_center()
            .gap(px(0.0))
            .when(disabled, |this| this.opacity(0.45).cursor_not_allowed());

        for (idx, seg) in self.segments.into_iter().enumerate() {
            let is_selected = idx == selected;
            let on_change_clone = on_change.clone();

            let segment_el = div()
                .id(seg.id)
                .h_full()
                .px(px(10.0))
                .flex()
                .items_center()
                .justify_center()
                .gap(px(4.0))
                .rounded(px(4.5))
                .cursor_pointer()
                .when(is_selected, |this| {
                    this.bg(card_bg)
                        .border_1()
                        .border_color(card_shadow_color)
                        .shadow_sm()
                })
                .when(!disabled && !is_selected, |this| {
                    this.hover(|style| {
                        style.bg(if is_dark {
                            rgba(0xFFFFFF0D)
                        } else {
                            rgba(0x00000008)
                        })
                    })
                })
                .when(!disabled, |this| {
                    this.on_click(move |_: &ClickEvent, window, cx| {
                        if let Some(ref handler) = on_change_clone {
                            handler(&idx, window, cx);
                        }
                    })
                })
                .children(seg.icon.map(|icon| {
                    svg()
                        .path(icon.path())
                        .size(px(14.0))
                        .text_color(if is_selected {
                            apple_colors.label
                        } else {
                            apple_colors.secondary_label
                        })
                }))
                .children(seg.label.map(|label| {
                    div()
                        .mac_subheadline()
                        .font_weight(if is_selected {
                            gpui_kit::FontWeight::SEMIBOLD
                        } else {
                            gpui_kit::FontWeight::NORMAL
                        })
                        .text_color(if is_selected {
                            apple_colors.label
                        } else {
                            apple_colors.secondary_label
                        })
                        .child(label)
                }));

            row = row.child(segment_el);

            if idx + 1 < count {
                let show_divider = idx != selected && (idx + 1) != selected;
                let divider = div()
                    .w(px(1.0))
                    .h(px(14.0))
                    .bg(if show_divider { divider_color } else { rgba(0x00000000) });
                row = row.child(divider);
            }
        }

        row
    }
}
