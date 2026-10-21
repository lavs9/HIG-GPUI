use gpui_apple_hig::*;

#[test]
fn test_switch_dimensions() {
    let (w, h, thumb, inset) = MacSwitchSize::Regular.dimensions();
    assert_eq!(w, 38.0);
    assert_eq!(h, 22.0);
    assert_eq!(thumb, 18.0);
    assert_eq!(inset, 2.0);

    let (mw, mh, mthumb, minset) = MacSwitchSize::Mini.dimensions();
    assert_eq!(mw, 28.0);
    assert_eq!(mh, 16.0);
    assert_eq!(mthumb, 12.0);
    assert_eq!(minset, 2.0);
}

#[test]
fn test_accent_colors() {
    let blue = MacAccentColor::Blue;
    assert_eq!(blue.light_hex(), 0x007AFF);
    assert_eq!(blue.dark_hex(), 0x0A84FF);

    let green = MacAccentColor::Green;
    assert_eq!(green.light_hex(), 0x34C759);
    assert_eq!(green.dark_hex(), 0x30D158);
}

#[test]
fn test_keyboard_shortcut_formatting() {
    assert_eq!(format_mac_shortcut("cmd+f"), "⌘f");
    assert_eq!(format_mac_shortcut("shift+cmd+space"), "⇧⌘space");
    assert_eq!(format_mac_shortcut("cmd+,"), "⌘,");
    assert_eq!(format_mac_shortcut("alt+enter"), "⌥↵");
    assert_eq!(format_mac_shortcut("ctrl+c"), "⌃c");
    assert_eq!(format_mac_shortcut("secondary-f"), "⌘f");
}

#[test]
fn test_checkbox_state_conversion() {
    assert_eq!(MacCheckboxState::from(true), MacCheckboxState::Checked);
    assert_eq!(MacCheckboxState::from(false), MacCheckboxState::Unchecked);
}

#[test]
fn test_button_sizes() {
    assert_eq!(MacButtonSize::Small.height(), 20.0);
    assert_eq!(MacButtonSize::Regular.height(), 24.0);
    assert_eq!(MacButtonSize::Large.height(), 28.0);
}

#[test]
fn test_spinner_sizes() {
    assert_eq!(MacSpinnerSize::Small.size_px(), 16.0);
    assert_eq!(MacSpinnerSize::Regular.size_px(), 24.0);
    assert_eq!(MacSpinnerSize::Large.size_px(), 32.0);
}

#[test]
fn test_text_field_sizes() {
    assert_eq!(MacFieldSize::Small.height(), 20.0);
    assert_eq!(MacFieldSize::Regular.height(), 24.0);
    assert_eq!(MacFieldSize::Large.height(), 28.0);

    assert_eq!(MacFieldSize::Small.font_size(), 11.0);
    assert_eq!(MacFieldSize::Regular.font_size(), 13.0);
    assert_eq!(MacFieldSize::Large.font_size(), 14.0);
}

#[test]
fn test_dialog_variants() {
    assert_eq!(MacDialogVariant::default(), MacDialogVariant::Informational);
}

#[test]
fn test_combo_box_construction() {
    let combo = MacComboBox::new("combo1")
        .placeholder("Select preset")
        .option("Standard")
        .option("High Resolution")
        .selected_index(Some(0));
    let _ = combo;
}

#[test]
fn test_digit_entry_construction() {
    let digit = MacDigitEntry::new("pin1", 6).value("123456").secure(true);
    let _ = digit;
}

#[test]
fn test_image_well_construction() {
    let well = MacImageWell::new("well1")
        .size(140.0, 140.0)
        .caption("Original DNG")
        .has_image(true);
    let _ = well;
}

#[test]
fn test_date_picker_construction() {
    let picker = MacDatePicker::new("date1")
        .date(2026, 10, 21)
        .time(12, 30)
        .show_time(true);
    let _ = picker;
}

#[test]
fn test_context_menu_construction() {
    let menu = MacContextMenu::new("ctx1")
        .header("Actions")
        .item("Open with Loupe")
        .separator()
        .item("Delete");
    let _ = menu;
}

#[test]
fn test_menu_bar_construction() {
    let bar = MacMenuBar::new("menubar1", "Loupe").active_menu(Some(0));
    let _ = bar;
}

#[test]
fn test_path_control_construction() {
    let path = MacPathControl::new("path1")
        .item("Macintosh HD")
        .item("Users")
        .item("mayank")
        .item("Projects");
    let _ = path;
}

#[test]
fn test_tab_bar_construction() {
    let tab_bar = MacTabBar::new("tabs1")
        .tab_item(MacTabItem::new("General").badge(2))
        .tab_item(MacTabItem::new("Appearance"))
        .tab_item(MacTabItem::new("Advanced"))
        .selected_index(0);
    let _ = tab_bar;
}

#[test]
fn test_token_field_construction() {
    let tokens = MacTokenField::new("tokens1")
        .token("RAW")
        .token("Landscape")
        .token("Sony A7IV");
    let _ = tokens;
}

#[test]
fn test_popover_construction() {
    let popover = MacPopover::new("popover1")
        .title("Histogram Inspector")
        .arrow_position(MacPopoverArrowPosition::Top);
    let _ = popover;
}

#[test]
fn test_panel_construction() {
    let panel = MacPanel::new("panel1", "Adjustments").hud().width(280.0);
    let _ = panel;
}

#[test]
fn test_page_control_construction() {
    let page_ctrl = MacPageControl::new("page1", 5).current_page(2);
    let _ = page_ctrl;
}

#[test]
fn test_window_construction() {
    let win = MacWindow::new("win1", "Photo Engine")
        .subtitle("Edited — DSC04921.ARW")
        .size(640.0, 480.0);
    let _ = win;
}

#[test]
fn test_gauge_construction() {
    let gauge = MacGauge::new("gauge1", 0.82)
        .label("Disk Cache")
        .thresholds(0.70, 0.90);
    let _ = gauge;
}

#[test]
fn test_rating_indicator_construction() {
    let rating = MacRatingIndicator::new("rating1", 4).max_rating(5);
    let _ = rating;
}

#[test]
fn test_activity_rings_construction() {
    let rings = MacActivityRings::new("rings1")
        .ring("CPU", 0.65, MacAccentColor::Red)
        .ring("Memory", 0.45, MacAccentColor::Green)
        .ring("GPU", 0.85, MacAccentColor::Blue);
    let _ = rings;
}

#[test]
fn test_table_view_construction() {
    let table = MacTableView::new("table1")
        .column("File Name", 180.0)
        .column("Size", 80.0)
        .column("Type", 80.0)
        .row(vec!["IMG_0001.CR3", "34.2 MB", "Canon RAW"])
        .row(vec!["IMG_0002.CR3", "32.8 MB", "Canon RAW"])
        .selected_row(Some(0));
    let _ = table;
}

#[test]
fn test_split_view_construction() {
    let split = MacSplitView::new("split1").leading_width(240.0);
    let _ = split;
}

#[test]
fn test_outline_view_construction() {
    let outline = MacOutlineView::new("outline1")
        .item(MacOutlineItem::new("item1", "All Photos", 0).has_children(true))
        .item(MacOutlineItem::new("item2", "2026", 1).has_children(true))
        .item(MacOutlineItem::new("item3", "October", 2));
    let _ = outline;
}

#[test]
fn test_column_view_construction() {
    let col_view = MacColumnView::new("col1").column(vec![
        MacColumnEntry {
            title: "Pictures".into(),
            is_folder: true,
            is_selected: true,
        },
        MacColumnEntry {
            title: "Documents".into(),
            is_folder: true,
            is_selected: false,
        },
    ]);
    let _ = col_view;
}

#[test]
fn test_text_view_construction() {
    let text_view = MacTextView::new("tv1")
        .text("Line 1\nLine 2\nLine 3")
        .show_line_numbers(true);
    let _ = text_view;
}

#[test]
fn test_image_view_construction() {
    let img_view = MacImageView::new("img1")
        .size(200.0, 150.0)
        .caption("Yosemite Valley at Sunrise")
        .content_mode(MacImageContentMode::AspectFill);
    let _ = img_view;
}

#[test]
fn test_chart_construction() {
    let chart = MacChart::new("chart1")
        .title("Weekly Ingest Volume")
        .point("Mon", 0.4)
        .point("Tue", 0.7)
        .point("Wed", 0.9)
        .point("Thu", 0.5)
        .point("Fri", 0.85);
    let _ = chart;
}
