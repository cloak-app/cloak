import { invoke } from '@tauri-apps/api/core';
import { platform } from '@tauri-apps/plugin-os';
import {
  KEY_MAP,
  KEY_SYMBOLS,
  MODIFIER_KEYS,
  RESERVED_SHORTCUTS,
} from './config';
import { Config } from '@/types';

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

export const isModifierKey = (key: string): boolean => {
  return MODIFIER_KEYS.includes(key);
};

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

export const validateShortcut = async (keys: string[]): Promise<void> => {
  if (!keys.length) {
    return Promise.reject('快捷键不能为空');
  }

  // 是否是保留快捷键
  const isReserved = RESERVED_SHORTCUTS.some(
    (reserved) =>
      reserved.length === keys.length &&
      reserved.every(
        (key, index) => key.toLowerCase() === keys[index].toLowerCase(),
      ),
  );

  if (isReserved) {
    return Promise.reject('该快捷键为系统保留快捷键');
  }

  // 是否全为修饰键或非修饰键
  const isAllModifier = keys.every((key) => isModifierKey(key));
  const isAllNonModifier = keys.every((key) => !isModifierKey(key));

  if (isAllModifier || isAllNonModifier) {
    return Promise.reject('快捷键不能全为修饰键或非修饰键');
  }

  const config = await invoke<Config>('get_config');

  const existingShortcuts = Object.entries(config)
    .filter(([key]) => key.endsWith('shortcut'))
    .map(([, value]) => value.split('+') as string[]);

  const isConflict = existingShortcuts.some((shortcut) =>
    shortcut.every(
      (key, index) => key.toLowerCase() === keys[index].toLowerCase(),
    ),
  );

  if (isConflict) {
    return Promise.reject('快捷键冲突');
  }

  return Promise.resolve();
};
