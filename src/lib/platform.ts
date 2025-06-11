const navigatorPlatform = navigator.platform.toLowerCase();

export const isWeb = !('__TAURI_OS_PLUGIN_INTERNALS__' in window);
export const isDesktop = true;
export const isMac = navigatorPlatform.includes('mac');
export const isWin = navigatorPlatform.includes('win');
export const isLinux = navigatorPlatform.includes('linux');
export const appScale = 1;

export function platform() {
  if (isWin) {
    return 'windows';
  } else if (isMac) {
    return 'macos';
  } else if (isLinux) {
    return 'linux';
  }

  return void 0;
}

export function family() {
  if (isWeb) {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes('windows')) {
      return 'windows';
    } else {
      return 'unix';
    }
  } else {
    return 'unknown';
  }
}
