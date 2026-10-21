use gpui_kit::assets::IconName;
use gpui_kit::component::input::InputState;
use gpui_kit::component::scroll::ScrollableElement;
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
    slider_val: f32,
    stepper_val: i32,
    popup_idx: usize,
    disclosure_expanded: bool,
    show_dialog: bool,
    search_state: Entity<InputState>,
    text_state: Entity<InputState>,
    filter: Option<String>,
}

impl ShowcaseApp {
    fn new(window: &mut Window, filter: Option<String>, cx: &mut Context<Self>) -> Self {
        let search_state = cx.new(|cx| InputState::new(window, cx).placeholder("Search photos, clips, songs..."));
        let text_state = cx.new(|cx| InputState::new(window, cx).placeholder("Enter account name…"));
        Self {
            is_dark: false,
            accent: MacAccentColor::Blue,
            switch_val: true,
            mini_switch_val: false,
            checkbox_val: true,
            segmented_idx: 0,
            slider_val: 65.0,
            stepper_val: 42,
            popup_idx: 0,
            disclosure_expanded: false,
            show_dialog: false,
            search_state,
            text_state,
            filter,
        }
    }
}

impl Render for ShowcaseApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dark = self.is_dark;
        let _ = &self.filter;
        let apple_colors = if is_dark {
            AppleColors::dark(self.accent)
        } else {
            AppleColors::light(self.accent)
        };

        let mut root = div()
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
            // Main content area with scroll
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
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
                                    .subtitle("Match system, or toggle Light / Dark")
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
                    // Sliders & Adjustments Group
                    .child(
                        MacSettingsGroup::new("group_sliders")
                            .title("Sliders & Steppers")
                            .row(
                                MacSettingsRow::new("row_slider", "Brightness")
                                    .subtitle(format!("{:.0}%", self.slider_val))
                                    .icon(IconName::Sun)
                                    .control(
                                        div()
                                            .w(px(200.0))
                                            .child(
                                                MacSlider::new("brightness_slider")
                                                    .value(self.slider_val)
                                                    .min(0.0)
                                                    .max(100.0)
                                                    .on_change(cx.listener(|this, val: &f32, _, cx| {
                                                        this.slider_val = *val;
                                                        cx.notify();
                                                    })),
                                            ),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_stepper", "Cache Size")
                                    .subtitle(format!("{} GB", self.stepper_val))
                                    .control(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().mac_body().child(format!("{} GB", self.stepper_val)))
                                            .child(
                                                MacStepper::new("stepper")
                                                    .on_increment(cx.listener(|this, _, _, cx| {
                                                        this.stepper_val += 1;
                                                        cx.notify();
                                                    }))
                                                    .on_decrement(cx.listener(|this, _, _, cx| {
                                                        this.stepper_val = (this.stepper_val - 1).max(0);
                                                        cx.notify();
                                                    })),
                                            ),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_color", "Accent Swatch")
                                    .subtitle("Custom color well")
                                    .control(
                                        MacColorWell::new("swatch", apple_colors.accent),
                                    ),
                            ),
                    )
                    // Menus & Inputs Group
                    .child(
                        MacSettingsGroup::new("group_inputs")
                            .title("Menus & Inputs")
                            .row(
                                MacSettingsRow::new("row_popup", "Format")
                                    .subtitle("Mutual exclusion pop-up button")
                                    .control(
                                        MacPopupButton::new("format_popup")
                                            .item(MacMenuItem::new("jpeg", "JPEG Image"))
                                            .item(MacMenuItem::new("png", "PNG Image"))
                                            .item(MacMenuItem::new("heic", "HEIC Photo"))
                                            .selected_index(self.popup_idx)
                                            .on_change(cx.listener(|this, idx: &usize, _, cx| {
                                                this.popup_idx = *idx;
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_pulldown", "Action Menu")
                                    .subtitle("Contextual pull-down button")
                                    .control(
                                        MacPullDownButton::new("action_pulldown", "Export As…")
                                            .icon(IconName::Share),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_text", "Account")
                                    .subtitle("Standard text field with focus ring")
                                    .control(
                                        div()
                                            .w(px(200.0))
                                            .child(MacTextField::new("acc_field", self.text_state.clone())),
                                    ),
                            ),
                    )
                    // Progress & Spinners Group
                    .child(
                        MacSettingsGroup::new("group_progress")
                            .title("Indicators")
                            .row(
                                MacSettingsRow::new("row_prog", "Index Status")
                                    .subtitle("Determinate progress bar (72%)")
                                    .control(
                                        div()
                                            .w(px(160.0))
                                            .child(MacProgress::new("prog").value(0.72)),
                                    ),
                            )
                            .row(
                                MacSettingsRow::new("row_spinner", "Background Sync")
                                    .subtitle("Circular activity spinner")
                                    .control(MacSpinner::new("spinner")),
                            ),
                    )
                    // Disclosure Section
                    .child(
                        MacDisclosure::new("adv_disclosure", "Advanced Options")
                            .expanded(self.disclosure_expanded)
                            .on_toggle(cx.listener(|this, exp: &bool, _, cx| {
                                this.disclosure_expanded = *exp;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .mac_caption()
                                    .text_color(apple_colors.secondary_label)
                                    .child("Additional developer configuration options displayed in an expanding section."),
                            ),
                    )
                    // Action Buttons
                    .child(
                        MacSettingsGroup::new("group_actions")
                            .title("Buttons & Modals")
                            .row(
                                MacSettingsRow::new("row_buttons", "Action Hierarchy")
                                    .subtitle("Push, prominent accent, and alert modal")
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
                                                MacButton::new("btn_alert")
                                                    .destructive()
                                                    .label("Show Alert Dialog")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.show_dialog = true;
                                                        cx.notify();
                                                    })),
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
            );

        if self.show_dialog {
            root = root.child(
                MacDialog::new("dialog", "Remove Photo Index?")
                    .critical()
                    .message("This removes the local embedding index from disk. You can recreate it at any time.")
                    .primary_label("Remove")
                    .secondary_label(Some("Cancel".into()))
                    .destructive(true)
                    .on_secondary(cx.listener(|this, _, _, cx| {
                        this.show_dialog = false;
                        cx.notify();
                    }))
                    .on_primary(cx.listener(|this, _, _, cx| {
                        this.show_dialog = false;
                        cx.notify();
                    })),
            );
        }

        root
    }
}

fn main() {
    let filter = std::env::args().nth(1).map(|s| s.to_lowercase());
    application().with_assets(assets::AllAssets).run(move |cx| {
        init(cx);
        apply_apple_theme(false, MacAccentColor::Blue, cx);

        let bounds = Bounds::centered(None, size(px(960.0), px(720.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(600.0), px(400.0))),
            window_background: WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            ..TitleBar::window_options()
        };

        let filter_clone = filter.clone();
        open_window(options, cx, move |window, cx| {
            cx.new(|cx| ShowcaseApp::new(window, filter_clone, cx))
        }).expect("failed to open window");
    });
}
