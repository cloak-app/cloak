import React, { useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ReaderWindow, SettingsWindow } from './windows';
import './index.css';

type Page = 'reader' | 'settings';

const PAGE: Record<Page, React.ReactNode> = {
  reader: <ReaderWindow />,
  settings: <SettingsWindow />,
};

const App = () => {
  // 根据窗口标签决定渲染哪个组件
  const windowLabel = getCurrentWindow().label as Page;

  useEffect(() => {
    document.body.dataset.window = windowLabel;
  }, [windowLabel]);

  return PAGE[windowLabel];
};

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
