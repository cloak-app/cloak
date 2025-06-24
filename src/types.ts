export interface Novel {
  /** 小说 ID */
  id: number;
  /** 小说标题 */
  title: string;
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
}

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
}

export interface Chapter {
  index: number;
  title: string;
  start_line: number;
}

export interface Config {
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
  font_weight: string;
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
}
