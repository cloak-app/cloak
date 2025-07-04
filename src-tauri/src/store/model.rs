use serde_json::{Number, Value};

const DEFAULT_NEXT_LINE_SHORTCUT: &str = "Control+Alt+ArrowRight";
const DEFAULT_PREV_LINE_SHORTCUT: &str = "Control+Alt+ArrowLeft";
const DEFAULT_NEXT_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowDown";
const DEFAULT_PREV_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowUp";
const DEFAULT_BOSS_KEY_SHORTCUT: &str = "Control+Alt+Enter";
const DEFAULT_TOGGLE_READING_MODE_SHORTCUT: &str = "Control+Alt+Space";

#[derive(Clone, Copy, Debug)]
pub enum AppStoreKey {
    AutoCheckUpdate,
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
    pub fn as_str(&self) -> String {
        match self {
            AppStoreKey::AutoCheckUpdate => "auto_check_update".to_string(),
            AppStoreKey::AutoStart => "auto_start".to_string(),
            AppStoreKey::Language => "language".to_string(),
            AppStoreKey::Theme => "theme".to_string(),
            AppStoreKey::DockVisibility => "dock_visibility".to_string(),
            AppStoreKey::AlwaysOnTop => "always_on_top".to_string(),
            AppStoreKey::Transparent => "transparent".to_string(),
            AppStoreKey::LineSize => "line_size".to_string(),
            AppStoreKey::FontSize => "font_size".to_string(),
            AppStoreKey::FontFamily => "font_family".to_string(),
            AppStoreKey::LineHeight => "line_height".to_string(),
            AppStoreKey::FontWeight => "font_weight".to_string(),
            AppStoreKey::FontColor => "font_color".to_string(),
            AppStoreKey::LetterSpacing => "letter_spacing".to_string(),
            AppStoreKey::NextLineShortcut => "next_line_shortcut".to_string(),
            AppStoreKey::PrevLineShortcut => "prev_line_shortcut".to_string(),
            AppStoreKey::NextChapterShortcut => "next_chapter_shortcut".to_string(),
            AppStoreKey::PrevChapterShortcut => "prev_chapter_shortcut".to_string(),
            AppStoreKey::BossKeyShortcut => "boss_key_shortcut".to_string(),
            AppStoreKey::ToggleReadingModeShortcut => "toggle_reading_mode_shortcut".to_string(),
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            AppStoreKey::AutoCheckUpdate => Value::Bool(true),
            AppStoreKey::AutoStart => Value::Bool(false),
            AppStoreKey::Language => Value::String("zh-CN".to_string()),
            AppStoreKey::Theme => Value::String("system".to_string()),
            AppStoreKey::DockVisibility => Value::Bool(true),
            AppStoreKey::AlwaysOnTop => Value::Bool(true),
            AppStoreKey::Transparent => Value::Bool(true),
            AppStoreKey::LineSize => Value::Number(Number::from(50)),
            AppStoreKey::FontSize => Value::Number(Number::from(16)),
            AppStoreKey::FontFamily => Value::String("".to_string()),
            AppStoreKey::LineHeight => Value::Number(Number::from_f64(1.0).unwrap()),
            AppStoreKey::FontWeight => Value::Number(Number::from(400)),
            AppStoreKey::FontColor => Value::String("#000000".to_string()),
            AppStoreKey::LetterSpacing => Value::Number(Number::from_f64(0.0).unwrap()),
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
            AppStoreKey::AutoCheckUpdate,
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
