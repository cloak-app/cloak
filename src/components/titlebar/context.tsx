import { getCurrentWindow, type Window } from '@tauri-apps/api/window';
import { platform } from '@tauri-apps/plugin-os';
import React, { createContext, useCallback, useEffect, useState } from 'react';

interface TauriAppWindowContextType {
  appWindow: Window | null;
  isWindowMaximized: boolean;
  minimizeWindow: () => Promise<void>;
  maximizeWindow: () => Promise<void>;
  closeWindow: () => Promise<void>;
}

const TauriAppWindowContext = createContext<TauriAppWindowContextType>({
  appWindow: null,
  isWindowMaximized: false,
  minimizeWindow: () => Promise.resolve(),
  maximizeWindow: () => Promise.resolve(),
  closeWindow: () => Promise.resolve(),
});

interface TauriAppWindowProviderProps {
  children: React.ReactNode;
}

export const TauriAppWindowProvider: React.FC<TauriAppWindowProviderProps> = ({
  children,
}: any) => {
  const [appWindow] = useState<Window | null>(getCurrentWindow());
  const [isWindowMaximized, setIsWindowMaximized] = useState(false);

  // Update the isWindowMaximized state when the window is resized
  const updateIsWindowMaximized = useCallback(async () => {
    if (appWindow) {
      const _isWindowMaximized = await appWindow.isMaximized();
      setIsWindowMaximized(_isWindowMaximized);
    }
  }, [appWindow]);

  useEffect(() => {
    // temporary: https://github.com/agmmnn/tauri-controls/issues/10#issuecomment-1675884962
    if (platform() !== 'macos') {
      updateIsWindowMaximized();
      let unlisten: () => void = () => {};

      const listen = async () => {
        if (appWindow) {
          unlisten = await appWindow.onResized(() => {
            updateIsWindowMaximized();
          });
        }
      };
      listen();

      // Cleanup the listener when the component unmounts
      return () => unlisten && unlisten();
    }
  }, [appWindow, updateIsWindowMaximized]);

  const minimizeWindow = async () => {
    if (appWindow) {
      await appWindow.minimize();
    }
  };

  const maximizeWindow = async () => {
    if (appWindow) {
      await appWindow.toggleMaximize();
    }
  };

  const closeWindow = async () => {
    if (appWindow) {
      await appWindow.close();
    }
  };

  // Provide the context values to the children components
  return (
    <TauriAppWindowContext.Provider
      value={{
        appWindow,
        isWindowMaximized,
        minimizeWindow,
        maximizeWindow,
        closeWindow,
      }}
    >
      {children}
    </TauriAppWindowContext.Provider>
  );
};

export default TauriAppWindowContext;
