import { z } from 'zod';

export const formSchema = z.object({
  check_update_interval: z.number(),
  auto_start: z.boolean(),
  theme: z.string(),
  language: z.string(),
  dock_visibility: z.boolean(),
  always_on_top: z.boolean(),
  transparent: z.boolean(),
  line_size: z.number(),
  font_size: z.number(),
  font_family: z.string(),
  line_height: z.number(),
  font_weight: z.number(),
  font_color: z.string(),
  letter_spacing: z.number(),
  next_line_shortcut: z.string(),
  prev_line_shortcut: z.string(),
  next_chapter_shortcut: z.string(),
  prev_chapter_shortcut: z.string(),
  boss_key_shortcut: z.string(),
  toggle_reading_mode_shortcut: z.string(),
});
