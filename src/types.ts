export interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
  total_lines: number;
}

export interface Reader {
  novel: Novel;
  chapters: Chapter[];
  line_num: number;
}

export interface Chapter {
  title: string;
  start_line: number;
}

export interface Config {
  /* ---------------------------------- 系统设置 ---------------------------------- */
  /** 是否显示 Dock 图标 */
  dock_visibility?: boolean;
  /** 是否总在最前 */
  always_on_top?: boolean;
  /** 是否透明 */
  transparent?: boolean;
  /* ---------------------------------- 阅读设置 ---------------------------------- */
  /** 字体大小 */
  font_size?: number;
  /** 字体 */
  font_family?: string;
  /** 行高 */
  line_height?: number;
  /** 字体粗细 */
  font_weight?: number;
  /** 字体颜色 */
  font_color?: string;
  /** 背景颜色 */
  background_color?: string;
}
