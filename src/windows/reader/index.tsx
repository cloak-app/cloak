import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { Novel } from '@/types';

export default function ReaderWindow() {
  const { data: reader } = useRequest(() => invoke<Novel>('get_novel_reader'));
  const { data: line, refresh } = useRequest(() => invoke<string>('get_line'));

  const [isFocus, setIsFocus] = useState(false);

  useEffect(() => {
    const listener = listen('reader-line-num-changed', () => refresh());

    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refresh]);

  useEffect(() => {
    const win = getCurrentWindow();

    const focusListener = win.listen('tauri://focus', () => {
      setIsFocus(true);
      win.setDecorations(true);
    });

    const blurListener = win.listen('tauri://blur', () => {
      setIsFocus(false);
      win.setDecorations(false);
    });

    return () => {
      focusListener.then((unListen) => unListen());
      blurListener.then((unListen) => unListen());
    };
  }, []);

  return (
    <div className={clsx('reader-window', { 'is-focus': isFocus })}>
      {reader ? line : <p>请从托盘菜单打开一本小说</p>}
    </div>
  );
}
