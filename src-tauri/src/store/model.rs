#[derive(Clone, Copy)]
pub enum AppStoreKey {
    LastReadNovelId,
    DockVisibility,
    AlwaysOnTop,
    Transparent,
    FontSize,
    FontFamily,
    LineHeight,
    LetterSpacing,
    FontWeight,
    FontColor,
    NextLineShortcut,
    PrevLineShortcut,
    BossKeyShortcut,
}

impl AppStoreKey {
    pub fn as_str(&self) -> &str {
        match self {
            AppStoreKey::LastReadNovelId => "last_read_novel_id",
            AppStoreKey::DockVisibility => "dock_visibility",
            AppStoreKey::AlwaysOnTop => "always_on_top",
            AppStoreKey::Transparent => "transparent",
            AppStoreKey::FontSize => "font_size",
            AppStoreKey::FontFamily => "font_family",
            AppStoreKey::LineHeight => "line_height",
            AppStoreKey::FontWeight => "font_weight",
            AppStoreKey::FontColor => "font_color",
            AppStoreKey::LetterSpacing => "letter_spacing",
            AppStoreKey::NextLineShortcut => "next_line_shortcut",
            AppStoreKey::PrevLineShortcut => "prev_line_shortcut",
            AppStoreKey::BossKeyShortcut => "boss_key_shortcut",
        }
    }
}
