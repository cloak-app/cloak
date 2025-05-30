import React from 'react';
import ReactDOM from 'react-dom/client';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ReaderWindow, SettingsWindow } from './windows';
import 'lessline/es/lessline.css';
import './index.less';

type Page = 'reader' | 'settings';

const PAGE: Record<Page, React.ReactNode> = {
  reader: <ReaderWindow />,
  settings: <SettingsWindow />,
};

const App = () => {
  // 根据窗口标签决定渲染哪个组件
  const windowLabel = getCurrentWindow().label as Page;

  return PAGE[windowLabel];
};

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
