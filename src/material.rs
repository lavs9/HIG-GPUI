//! Apple Human Interface Guidelines (macOS HIG) - Materials & Vibrancy.
//!
//! Materials act like glass in the physical world; they let background elements
//! show through while blurring them to maintain readability and establish visual depth.
//!
//! Conforms to Apple HIG Materials specifications:
//! - 5 Thickness Tiers: Ultra-Thin, Thin, Regular, Thick, Ultra-Thick.
//! - 11 Semantic Context Roles: Titlebar, Selection, Menu, Popover, Sidebar,
//!   HeaderView, Sheet, WindowBackground, HudWindow, FullScreenUI, ToolTip.
//! - Blending Modes: BehindWindow (desktop blur) & WithinWindow (in-app scroll blur).
//! - Window Active/Inactive states with dynamic desaturation.
//! - Accessibility: "Reduce Transparency" fallback to high-contrast opaque surfaces.
//! - Specular Glass Rim Highlight (1px subtle border).

use gpui_kit::component::ActiveTheme;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, StyleRefinement, Styled, Window, div, px, rgb, rgba, Hsla,
};

/// Thickness / Translucency spectrum of Apple HIG Materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacMaterialThickness {
    /// Ultra-Thin: Highest translucency, subtle blur (16px).
    /// Used for subtle overlays, floating badges, light translucent surfaces.
    UltraThin,
    /// Thin: Light blur (24px), moderate translucency.
    /// Used for source list sidebars, auxiliary panels.
    Thin,
    /// Regular: Medium balanced blur (32px), standard translucency.
    /// Default baseline for menus, toolbars, and grouped cards.
    #[default]
    Regular,
    /// Thick: Dense blur (44px), low translucency.
    /// Used for floating inspectors, dialog cards, modal sheets, and popovers.
    Thick,
    /// Ultra-Thick: Heaviest blur (60px), nearly opaque.
    /// Used for window background chrome, dialog backdrops, full-screen sheets.
    UltraThick,
}

impl MacMaterialThickness {
    /// Nominal blur radius in pixels.
    pub fn blur_radius(&self) -> f32 {
        match self {
            Self::UltraThin => 16.0,
            Self::Thin => 24.0,
            Self::Regular => 32.0,
            Self::Thick => 44.0,
            Self::UltraThick => 60.0,
        }
    }

    /// Background opacity (0.0 to 1.0) in light appearance.
    pub fn light_opacity(&self) -> f32 {
        match self {
            Self::UltraThin => 0.35,
            Self::Thin => 0.50,
            Self::Regular => 0.70,
            Self::Thick => 0.85,
            Self::UltraThick => 0.95,
        }
    }

    /// Background opacity (0.0 to 1.0) in dark appearance.
    pub fn dark_opacity(&self) -> f32 {
        match self {
            Self::UltraThin => 0.40,
            Self::Thin => 0.55,
            Self::Regular => 0.70,
            Self::Thick => 0.82,
            Self::UltraThick => 0.92,
        }
    }
}

/// Semantic context roles defined by macOS HIG (NSVisualEffectView / SwiftUI Material).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacMaterialRole {
    /// Window title bar and toolbar chrome (regular blur, within or behind window).
    #[default]
    Titlebar,
    /// Highlighted list/table selection with accent vibrancy.
    Selection,
    /// Context menus and pull-down menus.
    Menu,
    /// Floating anchored popover bubble.
    Popover,
    /// Source list sidebar navigation.
    Sidebar,
    /// Section / list / table headers.
    HeaderView,
    /// Modal document sheets.
    Sheet,
    /// Base window background surface.
    WindowBackground,
    /// Dark translucent Heads-Up Display utility panel.
    HudWindow,
    /// Full-screen modal overlay backdrop.
    FullScreenUI,
    /// Floating contextual tooltips.
    ToolTip,
    /// In-window scrollable content background.
    ContentBackground,
}

impl MacMaterialRole {
    /// Returns the recommended material thickness tier for this semantic role.
    pub fn default_thickness(&self) -> MacMaterialThickness {
        match self {
            Self::Sidebar | Self::ContentBackground => MacMaterialThickness::Thin,
            Self::Titlebar | Self::Selection | Self::Menu | Self::HeaderView | Self::ToolTip => {
                MacMaterialThickness::Regular
            }
            Self::Popover | Self::Sheet | Self::HudWindow => MacMaterialThickness::Thick,
            Self::WindowBackground | Self::FullScreenUI => MacMaterialThickness::UltraThick,
        }
    }

    /// Default blending mode for this role.
    pub fn default_blending_mode(&self) -> MacBlendingMode {
        match self {
            Self::Sidebar | Self::WindowBackground => MacBlendingMode::BehindWindow,
            _ => MacBlendingMode::WithinWindow,
        }
    }
}

/// Blending mode for Apple HIG materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacBlendingMode {
    /// Blends with the desktop wallpaper and windows beneath the app.
    BehindWindow,
    /// Blends with the scrolling or layered content within the same window.
    #[default]
    WithinWindow,
}

/// Apple HIG Material configuration and styling engine.
#[derive(Debug, Clone, Copy)]
pub struct MacMaterial {
    pub thickness: MacMaterialThickness,
    pub role: Option<MacMaterialRole>,
    pub blending_mode: MacBlendingMode,
    pub is_dark: bool,
    pub is_active: bool,
    pub reduce_transparency: bool,
    pub rim_highlight: bool,
    pub tint: Option<Hsla>,
}

impl Default for MacMaterial {
    fn default() -> Self {
        Self {
            thickness: MacMaterialThickness::Regular,
            role: None,
            blending_mode: MacBlendingMode::WithinWindow,
            is_dark: false,
            is_active: true,
            reduce_transparency: false,
            rim_highlight: true,
            tint: None,
        }
    }
}

impl MacMaterial {
    /// Create a material with the specified thickness.
    pub fn new(thickness: MacMaterialThickness) -> Self {
        Self {
            thickness,
            ..Default::default()
        }
    }

    /// Ultra-Thin material preset.
    pub fn ultra_thin() -> Self {
        Self::new(MacMaterialThickness::UltraThin)
    }

    /// Thin material preset (sidebars, secondary panels).
    pub fn thin() -> Self {
        Self::new(MacMaterialThickness::Thin)
    }

    /// Regular material preset (menus, toolbars).
    pub fn regular() -> Self {
        Self::new(MacMaterialThickness::Regular)
    }

    /// Thick material preset (dialogs, sheets, popovers).
    pub fn thick() -> Self {
        Self::new(MacMaterialThickness::Thick)
    }

    /// Ultra-Thick material preset (window backgrounds, full-screen backdrops).
    pub fn ultra_thick() -> Self {
        Self::new(MacMaterialThickness::UltraThick)
    }

    /// Create a material customized for a specific Apple HIG semantic role.
    pub fn from_role(role: MacMaterialRole) -> Self {
        Self {
            thickness: role.default_thickness(),
            role: Some(role),
            blending_mode: role.default_blending_mode(),
            ..Default::default()
        }
    }

    /// Set light or dark appearance.
    pub fn is_dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    /// Set window active/focused state.
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    /// Set Apple Accessibility "Reduce Transparency" mode.
    pub fn reduce_transparency(mut self, reduce: bool) -> Self {
        self.reduce_transparency = reduce;
        self
    }

    /// Set blending mode (BehindWindow or WithinWindow).
    pub fn blending_mode(mut self, mode: MacBlendingMode) -> Self {
        self.blending_mode = mode;
        self
    }

    /// Enable or disable the physical glass specular rim highlight (1px subtle border).
    pub fn rim_highlight(mut self, enabled: bool) -> Self {
        self.rim_highlight = enabled;
        self
    }

    /// Apply an optional color tint (e.g. selection vibrancy).
    pub fn tint(mut self, tint: Hsla) -> Self {
        self.tint = Some(tint);
        self
    }

    /// Computes the exact background color incorporating appearance, thickness, active state, and accessibility.
    pub fn background_color(&self) -> Hsla {
        // Accessibility fallback: Opaque high-contrast surface
        if self.reduce_transparency {
            return if self.is_dark {
                rgb(0x242426).into()
            } else {
                rgb(0xF2F2F7).into()
            };
        }

        // Special case: HUD Window role is always dark translucent
        if self.role == Some(MacMaterialRole::HudWindow) {
            return rgba(0x1C1C1EE6).into();
        }

        // Selection role with custom tint
        if self.role == Some(MacMaterialRole::Selection) {
            if let Some(tint) = self.tint {
                let alpha = if self.is_active { 0.25 } else { 0.12 };
                return tint.opacity(alpha);
            }
        }

        let base_alpha = if self.is_dark {
            self.thickness.dark_opacity()
        } else {
            self.thickness.light_opacity()
        };

        // When window is inactive, increase opacity slightly and desaturate
        let alpha = if self.is_active {
            base_alpha
        } else {
            (base_alpha + 0.08).min(1.0)
        };

        if self.is_dark {
            // macOS dark vibrancy: base is ~ #242427 / #323236 with varying alpha
            let rgb_val = match self.thickness {
                MacMaterialThickness::UltraThin => 0x202023,
                MacMaterialThickness::Thin => 0x242427,
                MacMaterialThickness::Regular => 0x2C2C30,
                MacMaterialThickness::Thick => 0x323236,
                MacMaterialThickness::UltraThick => 0x38383D,
            };
            let mut c: Hsla = rgb(rgb_val).into();
            c.a = alpha;
            c
        } else {
            // macOS light vibrancy: translucent white with subtle gray tint
            let mut c: Hsla = rgb(0xFFFFFF).into();
            c.a = alpha;
            c
        }
    }

    /// Computes the specular glass border (rim highlight) color.
    pub fn border_color(&self) -> Hsla {
        if !self.rim_highlight {
            return rgba(0x00000000).into();
        }

        if self.reduce_transparency {
            return if self.is_dark {
                rgba(0xFFFFFF20).into()
            } else {
                rgba(0x00000020).into()
            };
        }

        if self.role == Some(MacMaterialRole::HudWindow) {
            return rgba(0xFFFFFF33).into();
        }

        if self.is_dark {
            // Dark mode glass rim: subtle translucent white hairline
            rgba(0xFFFFFF26).into()
        } else {
            // Light mode glass rim: subtle translucent black hairline
            rgba(0x00000018).into()
        }
    }

    /// Blur radius in pixels.
    pub fn blur_radius(&self) -> f32 {
        if self.reduce_transparency {
            0.0
        } else {
            self.thickness.blur_radius()
        }
    }

    /// Typical drop shadow blur radius.
    pub fn shadow_blur(&self) -> f32 {
        match self.role {
            Some(MacMaterialRole::Sheet) | Some(MacMaterialRole::WindowBackground) => 24.0,
            Some(MacMaterialRole::Popover) | Some(MacMaterialRole::Menu) => 16.0,
            Some(MacMaterialRole::HudWindow) => 20.0,
            _ => 10.0,
        }
    }

    /// Drop shadow color.
    pub fn shadow_color(&self) -> Hsla {
        if self.is_dark {
            rgba(0x00000080).into()
        } else {
            rgba(0x00000026).into()
        }
    }

    /// Generates CSS backdrop-filter string for web documentation and previews.
    pub fn css_backdrop_filter(&self) -> String {
        if self.reduce_transparency {
            "none".to_string()
        } else {
            let blur = self.blur_radius();
            let saturate = if self.is_active { "180%" } else { "100%" };
            format!("blur({:.0}px) saturate({})", blur, saturate)
        }
    }
}

/// A container surface component rendering an Apple HIG Material backdrop.
#[derive(IntoElement)]
pub struct MacMaterialSurface {
    id: ElementId,
    style: StyleRefinement,
    material: MacMaterial,
    corner_radius: f32,
    children: Vec<AnyElement>,
}

impl MacMaterialSurface {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            material: MacMaterial::default(),
            corner_radius: 10.0,
            children: Vec::new(),
        }
    }

    pub fn thickness(mut self, thickness: MacMaterialThickness) -> Self {
        self.material.thickness = thickness;
        self
    }

    pub fn role(mut self, role: MacMaterialRole) -> Self {
        self.material = MacMaterial::from_role(role);
        self
    }

    pub fn material(mut self, material: MacMaterial) -> Self {
        self.material = material;
        self
    }

    pub fn is_dark(mut self, is_dark: bool) -> Self {
        self.material.is_dark = is_dark;
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.material.is_active = is_active;
        self
    }

    pub fn reduce_transparency(mut self, reduce: bool) -> Self {
        self.material.reduce_transparency = reduce;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Styled for MacMaterialSurface {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MacMaterialSurface {
    fn render(mut self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // If dark mode was not explicitly configured, inherit from the active app theme
        let theme_is_dark = cx.theme().mode.is_dark();
        if !self.material.is_dark && theme_is_dark {
            self.material.is_dark = true;
        }

        let bg = self.material.background_color();
        let border = self.material.border_color();

        div()
            .id(self.id)
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(px(self.corner_radius))
            .shadow_lg()
            .children(self.children)
    }
}

/// Extension trait to easily apply Apple HIG material styling directly to any GPUI element.
pub trait MacMaterialExt: Sized {
    /// Apply Apple HIG material styling with the given thickness and appearance.
    fn mac_material(self, thickness: MacMaterialThickness, is_dark: bool) -> Self;

    /// Apply Apple HIG material styling tailored to a semantic role.
    fn mac_material_role(self, role: MacMaterialRole, is_dark: bool) -> Self;
}

impl<E: InteractiveElement + Styled> MacMaterialExt for E {
    fn mac_material(self, thickness: MacMaterialThickness, is_dark: bool) -> Self {
        let mat = MacMaterial::new(thickness).is_dark(is_dark);
        self.bg(mat.background_color())
            .border_1()
            .border_color(mat.border_color())
    }

    fn mac_material_role(self, role: MacMaterialRole, is_dark: bool) -> Self {
        let mat = MacMaterial::from_role(role).is_dark(is_dark);
        self.bg(mat.background_color())
            .border_1()
            .border_color(mat.border_color())
    }
}
