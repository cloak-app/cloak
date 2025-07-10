import { platform } from '@tauri-apps/plugin-os';

export const isMac = platform() === 'macos';
export const isWindows = platform() === 'windows';
