use gpui_kit::{FontWeight, Styled, px};

pub const SYSTEM_FONT_NAME: &str = ".AppleSystemUIFont";
pub const MONO_FONT_NAME: &str = "SF Mono";

pub trait MacTypographyExt: Styled + Sized {
    /// Apple HIG Large Title (26pt bold)
    fn mac_large_title(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(26.0))
            .font_weight(FontWeight::BOLD)
            .line_height(px(32.0))
    }

    /// Apple HIG Title 1 (22pt bold)
    fn mac_title1(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(22.0))
            .font_weight(FontWeight::BOLD)
            .line_height(px(28.0))
    }

    /// Apple HIG Title 2 (17pt bold)
    fn mac_title2(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(17.0))
            .font_weight(FontWeight::BOLD)
            .line_height(px(22.0))
    }

    /// Apple HIG Title 3 (15pt semibold)
    fn mac_title3(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(15.0))
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(px(20.0))
    }

    /// Apple HIG Headline (13pt semibold)
    fn mac_headline(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(13.0))
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(px(18.0))
    }

    /// Apple HIG Body (13pt regular) - Standard macOS UI text
    fn mac_body(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(13.0))
            .font_weight(FontWeight::NORMAL)
            .line_height(px(18.0))
    }

    /// Apple HIG Callout (12pt regular)
    fn mac_callout(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(12.0))
            .font_weight(FontWeight::NORMAL)
            .line_height(px(16.0))
    }

    /// Apple HIG Subheadline (11pt regular)
    fn mac_subheadline(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(11.0))
            .font_weight(FontWeight::NORMAL)
            .line_height(px(15.0))
    }

    /// Apple HIG Footnote (10pt regular)
    fn mac_footnote(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(10.0))
            .font_weight(FontWeight::NORMAL)
            .line_height(px(14.0))
    }

    /// Apple HIG Caption 1 (10pt regular/medium)
    fn mac_caption(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(10.0))
            .font_weight(FontWeight::MEDIUM)
            .line_height(px(13.0))
    }

    /// Apple HIG Caption 2 (9pt regular)
    fn mac_caption2(self) -> Self {
        self.font_family(SYSTEM_FONT_NAME)
            .text_size(px(9.0))
            .font_weight(FontWeight::NORMAL)
            .line_height(px(12.0))
    }

    /// Apple HIG Monospace code (12pt SF Mono)
    fn mac_mono(self) -> Self {
        self.font_family(MONO_FONT_NAME)
            .text_size(px(12.0))
            .line_height(px(16.0))
    }
}

impl<T: Styled> MacTypographyExt for T {}
