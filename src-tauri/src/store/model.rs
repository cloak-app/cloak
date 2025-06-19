#[derive(Clone, Copy)]
pub enum AppStoreKey {
    LastReadNovelId,
    DockVisibility,
    AlwaysOnTop,
    Transparent,
    LineSize,
    FontSize,
    FontFamily,
    LineHeight,
    LetterSpacing,
    FontWeight,
    FontColor,
    NextLineShortcut,
    PrevLineShortcut,
    NextChapterShortcut,
    PrevChapterShortcut,
    BossKeyShortcut,
}

impl AppStoreKey {
    pub fn as_str(&self) -> &str {
        match self {
            AppStoreKey::LastReadNovelId => "last_read_novel_id",
            AppStoreKey::DockVisibility => "dock_visibility",
            AppStoreKey::AlwaysOnTop => "always_on_top",
            AppStoreKey::Transparent => "transparent",
            AppStoreKey::LineSize => "line_size",
            AppStoreKey::FontSize => "font_size",
            AppStoreKey::FontFamily => "font_family",
            AppStoreKey::LineHeight => "line_height",
            AppStoreKey::FontWeight => "font_weight",
            AppStoreKey::FontColor => "font_color",
            AppStoreKey::LetterSpacing => "letter_spacing",
            AppStoreKey::NextLineShortcut => "next_line_shortcut",
            AppStoreKey::PrevLineShortcut => "prev_line_shortcut",
            AppStoreKey::NextChapterShortcut => "next_chapter_shortcut",
            AppStoreKey::PrevChapterShortcut => "prev_chapter_shortcut",
            AppStoreKey::BossKeyShortcut => "boss_key_shortcut",
        }
    }
}
