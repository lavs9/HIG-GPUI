use gpui_kit::assets::IconName;
use gpui_kit::component::input::InputState;
use gpui_kit::component::TitleBar;
use gpui_kit::*;

use gpui_apple_hig::*;

struct ShowcaseApp {
    is_dark: bool,
    accent: MacAccentColor,
    switch_val: bool,
    mini_switch_val: bool,
    checkbox_val: bool,
    segmented_idx: usize,
    search_state: Entity<InputState>,
}

impl ShowcaseApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_state = cx.new(|cx| InputState::new(window, cx).placeholder("Search photos, clips, songs..."));
        Self {
            is_dark: false,
            accent: MacAccentColor::Blue,
            switch_val: true,
            mini_switch_val: false,
            checkbox_val: true,
            segmented_idx: 0,
            search_state,
        }
    }
}

impl Render for ShowcaseApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dark = self.is_dark;
        let apple_colors = if is_dark {
            AppleColors::dark(self.accent)
        } else {
            AppleColors::light(self.accent)
        };

        div()
            .size_full()
            .bg(apple_colors.window_background)
            .flex()
            .flex_col()
            // Toolbar with traffic light clearance
            .child(
                MacToolbar::new("toolbar")
                    .title("Apple HIG Showcase")
                    .center(
                        MacSegmentedControl::new("view_picker")
                            .segment(MacSegment::new("grid").icon(IconName::LayoutGrid).label("Grid"))
                            .segment(MacSegment::new("list").icon(IconName::LayoutList).label("List"))
                            .selected_index(self.segmented_idx)
                            .on_change(cx.listener(|this, idx: &usize, _, cx| {
                                this.segmented_idx = *idx;
                                cx.notify();
                            })),
                    )
                    .trailing(
                        MacSearchField::new("search", self.search_state.clone())
                            .shortcut_hint("⌘F"),
                    )
                    .trailing(
                        MacButton::new("theme_toggle")
                            .borderless()
                            .icon(if is_dark { IconName::Sun } else { IconName::Moon })
                            .tooltip(if is_dark { "Light mode" } else { "Dark mode" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.is_dark = !this.is_dark;
                                apply_apple_theme(this.is_dark, this.accent, cx);
                                cx.notify();
                            })),
                    ),
            )
            // Main content area
            .child(
                div()
                    .flex_1()
                    .p(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(20.0))
                    // Appearance Group
                    .child(
                        MacSettingsGroup::new("group_appearance")
                            .title("Appearance")
                            .row(
                                MacSettingsRow::new("row_mode", "Color Scheme")
                                    .subtitle("Match system, or force Light / Dark")
                                    .icon(IconName::Sun)
                                    .control(
                                        MacSegmentedControl::new("theme_segmented")
                                            .segment(MacSegment::new("light").label("Light"))
                                            .segment(MacSegment::new("dark").label("Dark"))
                                            .selected_index(if self.is_dark { 1 } else { 0 })
                                            .on_change(cx.listener(|this, idx: &usize, _, cx| {
                                                this.is_dark = *idx == 1;
                                                apply_apple_theme(this.is_dark, this.accent, cx);
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    // Toggles Group per Apple HIG
                    .child(
                        MacSettingsGroup::new("group_toggles")
                            .title("Toggles & Controls")
                            .footer("Switches emphasize important preferences; mini switches fit grouped rows.")
                            .row(
                                MacSettingsRow::new("row_switch_reg", "Background Watcher")
                                    .subtitle("Index new photos automatically while running")
                                    .icon(IconName::SlidersHorizontal)
                                    .control(
                                        MacSwitch::new("switch_reg")
                                            .checked(self.switch_val)
                                            .on_change(cx.listener(|this, val: &bool, _, cx| {
                                                this.switch_val = *val;
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_switch_mini", "Open at Login (Mini Switch)")
                                    .subtitle("Start silently in the menu bar")
                                    .control(
                                        MacSwitch::new("switch_mini")
                                            .mini()
                                            .checked(self.mini_switch_val)
                                            .on_change(cx.listener(|this, val: &bool, _, cx| {
                                                this.mini_switch_val = *val;
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_checkbox", "Automatic Software Updates")
                                    .subtitle("Check GitHub releases daily for updates")
                                    .control(
                                        MacCheckbox::new("checkbox_update")
                                            .checked(self.checkbox_val)
                                            .on_toggle(cx.listener(|this, val: &bool, _, cx| {
                                                this.checkbox_val = *val;
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    // Action Buttons
                    .child(
                        MacSettingsGroup::new("group_actions")
                            .title("Buttons")
                            .row(
                                MacSettingsRow::new("row_buttons", "Button Styles")
                                    .subtitle("Standard push, prominent accent, and destructive")
                                    .control(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                MacButton::new("btn_push")
                                                    .label("Cancel"),
                                            )
                                            .child(
                                                MacButton::new("btn_prominent")
                                                    .prominent()
                                                    .label("Save Changes"),
                                            )
                                            .child(
                                                MacButton::new("btn_destructive")
                                                    .destructive()
                                                    .label("Remove Index"),
                                            ),
                                    ),
                            ),
                    ),
            )
            // Status bar
            .child(
                MacStatusBar::new("status_bar")
                    .leading(div().child("Apple HIG Design System"))
                    .center(MacBadge::new("badge_status", "4,281 items indexed").success())
                    .trailing(MacKbd::new("kbd_hint", "⇧⌘Space")),
            )
    }
}

fn main() {
    application().with_assets(assets::AllAssets).run(|cx| {
        init(cx);
        apply_apple_theme(false, MacAccentColor::Blue, cx);

        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(600.0), px(400.0))),
            window_background: WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            ..TitleBar::window_options()
        };

        open_window(options, cx, |window, cx| {
            cx.new(|cx| ShowcaseApp::new(window, cx))
        }).expect("failed to open window");
    });
}
