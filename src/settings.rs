use gpui_kit::assets::IconName;
use gpui_kit::base::StyledExt;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, rgb, rgba, svg,
};

use crate::theme::{AppleColors, MacAccentColor};
use crate::typography::MacTypographyExt;

/// A row inside a macOS Settings grouped card.
pub struct MacSettingsRow {
    pub id: ElementId,
    pub title: SharedString,
    pub subtitle: Option<SharedString>,
    pub icon: Option<IconName>,
    pub icon_color: Option<Hsla>,
    pub control: Option<AnyElement>,
}

impl MacSettingsRow {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            icon: None,
            icon_color: None,
            control: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_color(mut self, color: impl Into<Hsla>) -> Self {
        self.icon_color = Some(color.into());
        self
    }

    pub fn control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }
}

/// A macOS Settings Group card adhering to Apple Human Interface Guidelines.
#[derive(IntoElement)]
pub struct MacSettingsGroup {
    id: ElementId,
    style: StyleRefinement,
    title: Option<SharedString>,
    footer: Option<SharedString>,
    rows: Vec<MacSettingsRow>,
    children: Vec<AnyElement>,
}

impl MacSettingsGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: None,
            footer: None,
            rows: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn row(mut self, row: MacSettingsRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Styled for MacSettingsGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacSettingsGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().mode.is_dark();
        let apple_colors = if is_dark {
            AppleColors::dark(MacAccentColor::Blue)
        } else {
            AppleColors::light(MacAccentColor::Blue)
        };

        let card_bg = if is_dark {
            rgba(0xFFFFFF0F) // 6% white card
        } else {
            rgb(0xFFFFFF).into()
        };

        let card_border = if is_dark {
            rgba(0xFFFFFF1F) // 12% white border
        } else {
            rgba(0x00000014) // 8% black border
        };

        let row_count = self.rows.len();
        let mut card_content = div()
            .w_full()
            .rounded(px(10.0))
            .bg(card_bg)
            .border_1()
            .border_color(card_border)
            .flex()
            .flex_col();

        for (idx, row) in self.rows.into_iter().enumerate() {
            let row_el = div()
                .id(row.id)
                .w_full()
                .min_h(px(42.0))
                .px(px(16.0))
                .py(px(10.0))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(12.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .flex_1()
                        .children(row.icon.map(|icon| {
                            let icon_bg = row.icon_color.unwrap_or(apple_colors.accent);
                            div()
                                .w(px(26.0))
                                .h(px(26.0))
                                .rounded(px(6.0))
                                .bg(icon_bg)
                                .flex()
                                .items_center()
                                .justify_center()
                                .flex_shrink_0()
                                .child(
                                    svg()
                                        .path(icon.path())
                                        .size(px(15.0))
                                        .text_color(rgb(0xFFFFFF)),
                                )
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .child(
                                    div()
                                        .mac_body()
                                        .text_color(apple_colors.label)
                                        .child(row.title),
                                )
                                .children(row.subtitle.map(|sub| {
                                    div()
                                        .mac_subheadline()
                                        .text_color(apple_colors.secondary_label)
                                        .child(sub)
                                })),
                        ),
                )
                .children(row.control.map(|ctrl| {
                    div().flex_shrink_0().child(ctrl)
                }));

            card_content = card_content.child(row_el);

            // Inset hairline divider between rows
            if idx + 1 < row_count {
                let divider = div()
                    .w_full()
                    .pl(px(16.0))
                    .child(
                        div()
                            .w_full()
                            .h(px(1.0))
                            .bg(apple_colors.separator),
                    );
                card_content = card_content.child(divider);
            }
        }

        for child in self.children {
            card_content = card_content.child(child);
        }

        div()
            .id(self.id)
            .refine_style(&self.style)
            .w_full()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(self.title.map(|title| {
                div()
                    .mac_caption()
                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                    .text_color(apple_colors.secondary_label)
                    .px(px(6.0))
                    .child(title.to_string().to_uppercase())
            }))
            .child(card_content)
            .children(self.footer.map(|footer| {
                div()
                    .mac_footnote()
                    .text_color(apple_colors.secondary_label)
                    .px(px(6.0))
                    .child(footer)
            }))
    }
}
