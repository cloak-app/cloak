import { platform } from '@tauri-apps/plugin-os';

// 系统保留的快捷键
export const RESERVED_SHORTCUTS = [
  ['Command', 'C'],
  ['Command', 'V'],
  ['Command', 'X'],
  ['Command', 'A'],
  ['Command', 'Z'],
  ['Command', 'Q'],
  // Windows/Linux
  ['Control', 'C'],
  ['Control', 'V'],
  ['Control', 'X'],
  ['Control', 'A'],
  ['Control', 'Z'],
];

const isMac = platform() === 'macos';

export const KEY_SYMBOLS: Record<string, string> = {
  Control: isMac ? '⌃' : 'Ctrl',
  control: isMac ? '⌃' : 'Ctrl',
  Ctrl: isMac ? '⌃' : 'Ctrl',
  ctrl: isMac ? '⌃' : 'Ctrl',
  Shift: isMac ? '⇧' : 'Shift',
  shift: isMac ? '⇧' : 'Shift',
  Alt: isMac ? '⌥' : 'Alt',
  alt: isMac ? '⌥' : 'Alt',
  Option: isMac ? '⌥' : 'Alt',
  option: isMac ? '⌥' : 'Alt',
  Meta: isMac ? '⌘' : 'Win',
  meta: isMac ? '⌘' : 'Win',
  Command: isMac ? '⌘' : 'Win',
  command: isMac ? '⌘' : 'Win',
  super: isMac ? '⌘' : 'Win',
  // Special keys
  Space: 'Space',
  space: 'Space',
  Enter: '↵',
  Backspace: '⌫',
  Delete: 'Del',
  Escape: 'Esc',
  ArrowUp: '↑',
  ArrowDown: '↓',
  ArrowLeft: '←',
  ArrowRight: '→',
  Tab: '⇥',
};

// 修饰键
export const MODIFIER_KEYS = ['Control', 'Shift', 'Alt', 'Meta', 'Command'];

export const KEY_MAP: Record<string, string> = {
  ControlLeft: 'Control',
  ControlRight: 'Control',
  ShiftLeft: 'Shift',
  ShiftRight: 'Shift',
  AltLeft: 'Alt',
  AltRight: 'Alt',
  MetaLeft: 'Command',
  MetaRight: 'Command',
  Space: 'Space',
};
