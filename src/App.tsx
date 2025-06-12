import { getCurrentWindow } from '@tauri-apps/api/window';
import React, { useEffect } from 'react';
import ReaderWindow from './windows/reader';
import SettingsWindow from './windows/settings';
import UpdateWindow from './windows/update';

type Page = 'reader' | 'settings' | 'update';

const PAGE: Record<Page, React.ReactNode> = {
  reader: <ReaderWindow />,
  settings: <SettingsWindow />,
  update: <UpdateWindow />,
};

const App = () => {
  // 根据窗口标签决定渲染哪个组件
  const windowLabel = getCurrentWindow().label as Page;

  useEffect(() => {
    document.body.dataset.window = windowLabel;
  }, [windowLabel]);

  return PAGE[windowLabel];
};

export default App;
