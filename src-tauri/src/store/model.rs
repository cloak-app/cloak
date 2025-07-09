use crate::constants::store::*;
use serde_json::{Number, Value};

#[derive(Clone, Copy, Debug)]
pub enum AppStoreKey {
    CheckUpdateInterval,
    AutoStart,
    Language,
    Theme,
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
    ToggleReadingModeShortcut,
}

impl AppStoreKey {
    pub fn as_str(&self) -> &str {
        match self {
            AppStoreKey::CheckUpdateInterval => "check_update_interval",
            AppStoreKey::AutoStart => "auto_start",
            AppStoreKey::Language => "language",
            AppStoreKey::Theme => "theme",
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
            AppStoreKey::ToggleReadingModeShortcut => "toggle_reading_mode_shortcut",
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            AppStoreKey::CheckUpdateInterval => {
                Value::Number(Number::from(DEFAULT_CHECK_UPDATE_INTERVAL))
            }
            AppStoreKey::AutoStart => Value::Bool(DEFAULT_AUTO_START),
            AppStoreKey::Language => Value::String(DEFAULT_LANGUAGE.to_string()),
            AppStoreKey::Theme => Value::String(DEFAULT_THEME.to_string()),
            AppStoreKey::DockVisibility => Value::Bool(DEFAULT_DOCK_VISIBILITY),
            AppStoreKey::AlwaysOnTop => Value::Bool(DEFAULT_ALWAYS_ON_TOP),
            AppStoreKey::Transparent => Value::Bool(DEFAULT_TRANSPARENT),
            AppStoreKey::LineSize => Value::Number(Number::from(DEFAULT_LINE_SIZE)),
            AppStoreKey::FontSize => Value::Number(Number::from(DEFAULT_FONT_SIZE)),
            AppStoreKey::FontFamily => Value::String(DEFAULT_FONT_FAMILY.to_string()),
            AppStoreKey::LineHeight => {
                Value::Number(Number::from_f64(DEFAULT_LINE_HEIGHT).unwrap())
            }
            AppStoreKey::FontWeight => Value::Number(Number::from(DEFAULT_FONT_WEIGHT)),
            AppStoreKey::FontColor => Value::String(DEFAULT_FONT_COLOR.to_string()),
            AppStoreKey::LetterSpacing => {
                Value::Number(Number::from_f64(DEFAULT_LETTER_SPACING).unwrap())
            }
            AppStoreKey::NextLineShortcut => Value::String(DEFAULT_NEXT_LINE_SHORTCUT.to_string()),
            AppStoreKey::PrevLineShortcut => Value::String(DEFAULT_PREV_LINE_SHORTCUT.to_string()),
            AppStoreKey::NextChapterShortcut => {
                Value::String(DEFAULT_NEXT_CHAPTER_SHORTCUT.to_string())
            }
            AppStoreKey::PrevChapterShortcut => {
                Value::String(DEFAULT_PREV_CHAPTER_SHORTCUT.to_string())
            }
            AppStoreKey::BossKeyShortcut => Value::String(DEFAULT_BOSS_KEY_SHORTCUT.to_string()),
            AppStoreKey::ToggleReadingModeShortcut => {
                Value::String(DEFAULT_TOGGLE_READING_MODE_SHORTCUT.to_string())
            }
        }
    }

    pub fn keys() -> Vec<AppStoreKey> {
        vec![
            AppStoreKey::CheckUpdateInterval,
            AppStoreKey::AutoStart,
            AppStoreKey::Language,
            AppStoreKey::Theme,
            AppStoreKey::DockVisibility,
            AppStoreKey::AlwaysOnTop,
            AppStoreKey::Transparent,
            AppStoreKey::LineSize,
            AppStoreKey::FontSize,
            AppStoreKey::FontFamily,
            AppStoreKey::LineHeight,
            AppStoreKey::LetterSpacing,
            AppStoreKey::FontWeight,
            AppStoreKey::FontColor,
            AppStoreKey::NextLineShortcut,
            AppStoreKey::PrevLineShortcut,
            AppStoreKey::NextChapterShortcut,
            AppStoreKey::PrevChapterShortcut,
            AppStoreKey::BossKeyShortcut,
            AppStoreKey::ToggleReadingModeShortcut,
        ]
    }
}
