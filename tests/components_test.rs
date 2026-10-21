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
}

#[test]
fn test_dialog_variants() {
    assert_eq!(MacDialogVariant::default(), MacDialogVariant::Informational);
}
