// 配置变化事件，用于通知前端更新配置
pub const CONFIG_CHANGE: &str = "config-change";

// 阅读器变化事件，用于通知前端更新阅读器状态（如当前小说、当前阅读行等）
pub const READER_CHANGE: &str = "reader-change";

// 阅读状态变化事件，用于通知前端阅读状态变更
pub const READING_MODE_CHANGE: &str = "reading-mode-change";

// 更新检查状态变化事件，用于通知前端更新检查状态
pub const UPDATE_CHECK_STATUS_CHANGE: &str = "update-check-status-changed";

// 自动更新进度变化事件，用于通知前端更新进度条
pub const UPDATE_PROGRESS_CHANGE: &str = "update-progress-change";

// 自动更新完成事件，用于通知前端自动更新完成
pub const UPDATE_FINISHED: &str = "update-finished";
