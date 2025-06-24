import { platform } from '@tauri-apps/plugin-os';
import { KEY_MAP, KEY_SYMBOLS, MODIFIER_KEYS } from './config';

const isMac = platform() === 'macos';

export const metaOrCtrlKey = (): string => {
  if (isMac) {
    return 'Meta';
  } else {
    return 'Control';
  }
};

export const isMetaOrCtrlKey = (event: KeyboardEvent): boolean => {
  if (isMac) {
    return event.metaKey;
  } else {
    return event.ctrlKey;
  }
};

// Normalize key names
export const normalizeKey = (key: string): string => {
  if (KEY_MAP[key]) {
    return KEY_MAP[key];
  }

  if (key.startsWith('Key')) {
    return key.replace('Key', '');
  }

  if (key.startsWith('Digit')) {
    return key.replace('Digit', '');
  }

  if (key.startsWith('Numpad')) {
    return key.replace('Numpad', '');
  }

  return key;
};

// Format key for display
export const formatKey = (key: string): string => {
  if (KEY_SYMBOLS[key]) {
    return KEY_SYMBOLS[key];
  }

  if (key.startsWith('Key')) {
    return key.replace('Key', '');
  }

  if (key.startsWith('Digit')) {
    return key.replace('Digit', '');
  }

  if (key.startsWith('Numpad')) {
    return key.replace('Numpad', '');
  }

  return key;
};

// Check if key is a modifier
export const isModifierKey = (key: string): boolean => {
  return MODIFIER_KEYS.includes(key);
};

// Sort keys to ensure consistent order (modifiers first)
export const sortKeys = (keys: string[]): string[] => {
  return [...keys].sort((a, b) => {
    const aIndex = MODIFIER_KEYS.indexOf(a);
    const bIndex = MODIFIER_KEYS.indexOf(b);

    if (aIndex === -1 && bIndex === -1) return 0;
    if (aIndex === -1) return 1;
    if (bIndex === -1) return -1;
    return aIndex - bIndex;
  });
};
