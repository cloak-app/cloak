// 应用数据存储路径
pub const APP_STORE_PATH: &str = "app_data.json";

/* ----------------------------------- 默认值 ---------------------------------- */

// 检查更新间隔
pub const DEFAULT_CHECK_UPDATE_INTERVAL: u64 = 28800;

// 自动启动
pub const DEFAULT_AUTO_START: bool = false;

// 语言
pub const DEFAULT_LANGUAGE: &str = "zh-CN";

// 主题
pub const DEFAULT_THEME: &str = "system";

// 是否显示 Dock 图标
pub const DEFAULT_DOCK_VISIBILITY: bool = true;

// 是否总在最前
pub const DEFAULT_ALWAYS_ON_TOP: bool = true;

// 是否透明
pub const DEFAULT_TRANSPARENT: bool = true;

// 每页字数
pub const DEFAULT_LINE_SIZE: u32 = 50;

// 字体大小
pub const DEFAULT_FONT_SIZE: u32 = 16;

// 字体
pub const DEFAULT_FONT_FAMILY: &str = "";

// 行高
pub const DEFAULT_LINE_HEIGHT: f64 = 1.0;

// 字体粗细
pub const DEFAULT_FONT_WEIGHT: u32 = 400;

// 字体颜色
pub const DEFAULT_FONT_COLOR: &str = "#000000";

// 字间距
pub const DEFAULT_LETTER_SPACING: f64 = 0.0;


// 下一行快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_NEXT_LINE_SHORTCUT: &str = "Control+Alt+ArrowRight";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_NEXT_LINE_SHORTCUT: &str = "Control+ArrowRight";

// 上一行快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_PREV_LINE_SHORTCUT: &str = "Control+Alt+ArrowLeft";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_PREV_LINE_SHORTCUT: &str = "Control+ArrowLeft";

// 下一章节快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_NEXT_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowDown";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_NEXT_CHAPTER_SHORTCUT: &str = "Control+ArrowDown";

// 上一章节快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_PREV_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowUp";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_PREV_CHAPTER_SHORTCUT: &str = "Control+ArrowUp";

// 老板键快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_BOSS_KEY_SHORTCUT: &str = "Control+Alt+Enter";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_BOSS_KEY_SHORTCUT: &str = "Control+Enter";

// 切换阅读模式快捷键
#[cfg(target_os = "macos")]
pub const DEFAULT_TOGGLE_READING_MODE_SHORTCUT: &str = "Control+Alt+Backslash";
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub const DEFAULT_TOGGLE_READING_MODE_SHORTCUT: &str = "Control+Backslash";

