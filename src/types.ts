/* ----------------------------------- 小说 ----------------------------------- */
export interface Novel {
  /** 小说 ID */
  id: number;
  /** 小说标题 */
  title: string;
  /** 小说封面 */
  cover?: ArrayBuffer;
  /** 小说作者 */
  author?: string;
  /** 小说描述 */
  description?: string;
  /** 小说路径 */
  path: string;
  /** 上次阅读位置 */
  read_position: number;
  /** 阅读进度 */
  read_progress: number;
  /** 文件大小 */
  file_size: number;
  /** 添加时间 */
  created_at: string;
  /** 更新时间 */
  updated_at: string;
  /** 是否打开 */
  is_open: 0 | 1;
}

/* ----------------------------------- 阅读器 ---------------------------------- */
export interface Reader {
  /** 小说 ID */
  novel_id: number;
  /** 小说路径 */
  novel_path: string;
  /** 章节列表 */
  chapters: Chapter[];
  /** 当前章节 */
  current_chapter: Chapter;
  /** 当前行号 */
  read_position: number;
  /** 阅读进度 */
  read_progress: number;
  /** 总行数 */
  total_lines: number;
}

export interface Chapter {
  index: number;
  title: string;
  start_line: number;
}

/* ----------------------------------- 配置 ----------------------------------- */
export interface Config {
  /* ---------------------------------- 偏好设置 ---------------------------------- */
  /** 自动检查更新 */
  check_update_interval: number;
  /** 自动启动 */
  auto_start: boolean;
  /** 语言 */
  language: string;
  /** 主题 */
  theme: string;
  /* ---------------------------------- 系统设置 ---------------------------------- */
  /** 是否显示 Dock 图标 */
  dock_visibility: boolean;
  /** 是否总在最前 */
  always_on_top: boolean;
  /** 是否透明 */
  transparent: boolean;
  /* ---------------------------------- 阅读设置 ---------------------------------- */
  /** 每页字数 */
  line_size: number;
  /** 字体大小 */
  font_size: number;
  /** 字体 */
  font_family: string;
  /** 行高 */
  line_height: number;
  /** 字体粗细 */
  font_weight: number;
  /** 字体颜色 */
  font_color: string;
  /** 字间距 */
  letter_spacing: number;
  /* ---------------------------------- 快捷键设置 --------------------------------- */
  /** 下一行快捷键 */
  next_line_shortcut: string;
  /** 上一行快捷键 */
  prev_line_shortcut: string;
  /** 下一章节快捷键 */
  next_chapter_shortcut: string;
  /** 上一章节快捷键 */
  prev_chapter_shortcut: string;
  /** 老板键快捷键 */
  boss_key_shortcut: string;
  /** 切换阅读模式快捷键 */
  toggle_reading_mode_shortcut: string;
}

/* ----------------------------------- 更新 ----------------------------------- */
export enum CheckUpdateStatus {
  Checking = 'checking',
  Success = 'success',
  Failed = 'failed',
}

export interface UpdateCheckResult {
  /** 检查时间 */
  timestamp: number;
  /** 检查结果 */
  status: CheckUpdateStatus;
}

/* ----------------------------------- 事件 ----------------------------------- */
export enum CustomEvent {
  ConfigChange = 'config-change',
  ReaderChange = 'reader-change',
  ReadingModeChange = 'reading-mode-change',
  UpdateCheckStatusChange = 'update-check-status-changed',
  UpdateProgressChange = 'update-progress-change',
  UpdateFinished = 'update-finished',
}
