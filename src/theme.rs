use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::{App, Hsla, px, rgb, rgba};

/// Apple System Accent Color options per macOS System Settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacAccentColor {
    #[default]
    Blue,
    Purple,
    Pink,
    Red,
    Orange,
    Yellow,
    Green,
    Graphite,
}

impl MacAccentColor {
    pub fn light_hex(&self) -> u32 {
        match self {
            Self::Blue => 0x007AFF,
            Self::Purple => 0xAF52DE,
            Self::Pink => 0xFF2D55,
            Self::Red => 0xFF3B30,
            Self::Orange => 0xFF9500,
            Self::Yellow => 0xFFCC00,
            Self::Green => 0x34C759,
            Self::Graphite => 0x8E8E93,
        }
    }

    pub fn dark_hex(&self) -> u32 {
        match self {
            Self::Blue => 0x0A84FF,
            Self::Purple => 0xBF5AF2,
            Self::Pink => 0xFF375F,
            Self::Red => 0xFF453A,
            Self::Orange => 0xFF9F0A,
            Self::Yellow => 0xFFD60A,
            Self::Green => 0x30D158,
            Self::Graphite => 0x98989D,
        }
    }

    pub fn color(&self, is_dark: bool) -> Hsla {
        let hex = if is_dark { self.dark_hex() } else { self.light_hex() };
        rgb(hex).into()
    }
}

/// Appearance mode for Apple HIG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppleAppearance {
    #[default]
    Auto,
    Light,
    Dark,
}

/// Complete palette of Apple Human Interface Guidelines semantic tokens for macOS.
#[derive(Debug, Clone, Copy)]
pub struct AppleColors {
    pub is_dark: bool,
    // Accents
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_active: Hsla,
    pub switch_green: Hsla,

    // Backgrounds
    pub window_background: Hsla,
    pub content_background: Hsla,
    pub grouped_background: Hsla,
    pub secondary_grouped_background: Hsla,
    pub tertiary_grouped_background: Hsla,
    pub sidebar_background: Hsla,
    pub toolbar_background: Hsla,
    pub control_background: Hsla,
    pub control_background_hover: Hsla,
    pub control_hover: Hsla,
    pub control_pressed: Hsla,
    pub popover_background: Hsla,

    // Labels & Text
    pub label: Hsla,
    pub secondary_label: Hsla,
    pub tertiary_label: Hsla,
    pub quaternary_label: Hsla,

    // Separators & Borders
    pub separator: Hsla,
    pub subtle_border: Hsla,
    pub focus_ring: Hsla,

    // System Status
    pub system_red: Hsla,
    pub system_green: Hsla,
    pub system_orange: Hsla,
    pub system_yellow: Hsla,
    pub system_blue: Hsla,
}

impl AppleColors {
    pub fn light(accent: MacAccentColor) -> Self {
        let accent_c = accent.color(false);
        Self {
            is_dark: false,
            accent: accent_c,
            accent_hover: rgb(0x0062CC).into(),
            accent_active: rgb(0x004FAF).into(),
            switch_green: rgb(0x34C759).into(),

            // macOS Light surfaces
            window_background: rgb(0xF2F2F7).into(),
            content_background: rgb(0xFFFFFF).into(),
            grouped_background: rgb(0xF2F2F7).into(),
            secondary_grouped_background: rgb(0xFFFFFF).into(),
            tertiary_grouped_background: rgb(0xE5E5EA).into(),
            sidebar_background: rgb(0xEDECEE).into(),
            toolbar_background: rgb(0xF6F6F6).into(),
            control_background: rgb(0xFFFFFF).into(),
            control_background_hover: rgb(0xF5F5F7).into(),
            control_hover: rgba(0x0000000D).into(),
            control_pressed: rgba(0x0000001A).into(),
            popover_background: rgb(0xFFFFFF).into(),

            // Text
            label: rgba(0x000000DE).into(),
            secondary_label: rgba(0x3C3C4399).into(),
            tertiary_label: rgba(0x3C3C434D).into(),
            quaternary_label: rgba(0x3C3C432E).into(),

            // Separators
            separator: rgba(0x3C3C432E).into(),
            subtle_border: rgba(0x0000001F).into(),
            focus_ring: accent_c.opacity(0.65),

            system_red: rgb(0xFF3B30).into(),
            system_green: rgb(0x34C759).into(),
            system_orange: rgb(0xFF9500).into(),
            system_yellow: rgb(0xFFCC00).into(),
            system_blue: rgb(0x007AFF).into(),
        }
    }

    pub fn dark(accent: MacAccentColor) -> Self {
        let accent_c = accent.color(true);
        Self {
            is_dark: true,
            accent: accent_c,
            accent_hover: rgb(0x3395FF).into(),
            accent_active: rgb(0x0070E0).into(),
            switch_green: rgb(0x30D158).into(),

            // macOS Dark surfaces
            window_background: rgb(0x1E1E1E).into(),
            content_background: rgb(0x1E1E1E).into(),
            grouped_background: rgb(0x18181A).into(),
            secondary_grouped_background: rgba(0xFFFFFF0F).into(),
            tertiary_grouped_background: rgb(0x2C2C2E).into(),
            sidebar_background: rgb(0x1A1A1C).into(),
            toolbar_background: rgb(0x242426).into(),
            control_background: rgba(0xFFFFFF1F).into(),
            control_background_hover: rgba(0xFFFFFF2E).into(),
            control_hover: rgba(0xFFFFFF1F).into(),
            control_pressed: rgba(0xFFFFFF2E).into(),
            popover_background: rgb(0x252528).into(),

            // Text
            label: rgba(0xFFFFFFFF).into(),
            secondary_label: rgba(0xEBEBF599).into(),
            tertiary_label: rgba(0xEBEBF54D).into(),
            quaternary_label: rgba(0xEBEBF52E).into(),

            // Separators
            separator: rgba(0x54545866).into(),
            subtle_border: rgba(0xFFFFFF1F).into(),
            focus_ring: accent_c.opacity(0.75),

            system_red: rgb(0xFF453A).into(),
            system_green: rgb(0x30D158).into(),
            system_orange: rgb(0xFF9F0A).into(),
            system_yellow: rgb(0xFFD60A).into(),
            system_blue: rgb(0x0A84FF).into(),
        }
    }

    pub fn disabled(&self) -> Hsla {
        self.quaternary_label
    }

    pub fn placeholder(&self) -> Hsla {
        self.tertiary_label
    }

    pub fn red(&self) -> Hsla {
        self.system_red
    }

    pub fn orange(&self) -> Hsla {
        self.system_orange
    }

    pub fn yellow(&self) -> Hsla {
        self.system_yellow
    }

    pub fn green(&self) -> Hsla {
        self.system_green
    }

    pub fn blue(&self) -> Hsla {
        self.system_blue
    }

    pub fn traffic_light_close(&self) -> Hsla {
        rgb(0xFF5F56).into()
    }

    pub fn traffic_light_minimize(&self) -> Hsla {
        rgb(0xFFBD2E).into()
    }

    pub fn traffic_light_zoom(&self) -> Hsla {
        rgb(0x27C93F).into()
    }
}

/// Applies the Apple HIG theme (Light or Dark) into GPUI's Theme system.
pub fn apply_apple_theme(is_dark: bool, accent: MacAccentColor, cx: &mut App) {
    let mode = if is_dark { ThemeMode::Dark } else { ThemeMode::Light };
    Theme::change(mode, None, cx);
    let colors = if is_dark {
        AppleColors::dark(accent)
    } else {
        AppleColors::light(accent)
    };

    Theme::update(cx, |theme| {
        theme.colors.background = colors.window_background;
        theme.colors.foreground = colors.label;
        theme.colors.title_bar = colors.toolbar_background;
        theme.colors.title_bar_border = colors.separator;
        theme.colors.status_bar = colors.toolbar_background;
        theme.colors.status_bar_border = colors.separator;
        theme.colors.border = colors.separator;
        theme.colors.window_border = colors.subtle_border;
        theme.colors.muted = colors.secondary_grouped_background;
        theme.colors.muted_foreground = colors.secondary_label;
        theme.colors.primary = colors.accent;
        theme.colors.primary_foreground = rgb(0xFFFFFF).into();
        theme.colors.primary_hover = colors.accent_hover;
        theme.colors.primary_active = colors.accent_active;
        theme.colors.button_primary = colors.accent;
        theme.colors.button_primary_foreground = rgb(0xFFFFFF).into();
        theme.colors.button_primary_hover = colors.accent_hover;
        theme.colors.button_primary_active = colors.accent_active;
        theme.colors.accent = colors.accent.opacity(0.15);
        theme.colors.accent_foreground = colors.accent;
        theme.colors.secondary = colors.control_background;
        theme.colors.secondary_foreground = colors.label;
        theme.colors.secondary_hover = colors.control_background_hover;
        theme.colors.secondary_active = colors.control_background;
        theme.colors.button_secondary = colors.control_background;
        theme.colors.button_secondary_foreground = colors.label;
        theme.colors.button_secondary_hover = colors.control_background_hover;
        theme.colors.button_secondary_active = colors.control_background;
        theme.colors.ring = colors.focus_ring;
        theme.colors.caret = colors.accent;
        theme.colors.selection = colors.accent.opacity(0.25);
        theme.colors.input = colors.subtle_border;
        theme.colors.popover = colors.popover_background;
        theme.colors.popover_foreground = colors.label;
        theme.colors.list = colors.content_background;
        theme.colors.list_hover = colors.control_background_hover;
        theme.colors.list_active = colors.accent.opacity(0.15);
        theme.colors.list_active_border = colors.accent;
        theme.colors.danger = colors.system_red;
        theme.colors.danger_foreground = rgb(0xFFFFFF).into();
        theme.colors.danger_hover = colors.system_red;
        theme.colors.success = colors.system_green;
        theme.colors.success_foreground = rgb(0xFFFFFF).into();
        theme.colors.sidebar = colors.sidebar_background;
        theme.colors.sidebar_foreground = colors.label;
        theme.colors.sidebar_border = colors.separator;
        theme.colors.sidebar_accent = colors.accent.opacity(0.12);
        theme.colors.sidebar_accent_foreground = colors.label;
        theme.colors.sidebar_primary = colors.accent;
        theme.colors.sidebar_primary_foreground = rgb(0xFFFFFF).into();
        theme.colors.group_box = colors.secondary_grouped_background;
        theme.colors.group_box_foreground = colors.label;
        theme.colors.progress_bar = colors.accent;
        theme.colors.scrollbar = colors.window_background.opacity(0.0);
        theme.colors.scrollbar_thumb = if is_dark {
            rgba(0xFFFFFF33).into()
        } else {
            rgba(0x00000033).into()
        };
        theme.colors.scrollbar_thumb_hover = if is_dark {
            rgba(0xFFFFFF55).into()
        } else {
            rgba(0x00000055).into()
        };
        theme.colors.link = colors.accent;
        theme.colors.link_hover = colors.accent_hover;

        // Apple HIG corner radiuses
        theme.radius = px(6.0); // standard macOS controls (6pt)
        theme.radius_lg = px(10.0); // macOS cards/groups (10pt)
        theme.shadow = true;
    });
}
