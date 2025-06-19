import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { Config, Novel } from '@/types';

export default function ReaderWindow() {
  const { data: reader, refresh: refreshReader } = useRequest(() =>
    invoke<Novel>('get_novel_reader'),
  );
  const { data: line, refresh: refreshLine } = useRequest(() =>
    invoke<string>('get_line'),
  );

  const { data: config, refresh: refreshConfig } = useRequest(() =>
    invoke<Config>('get_config'),
  );

  // 打开后默认聚焦
  const [isFocus, setIsFocus] = useState(true);

  useEffect(() => {
    const listener = listen('reader-line-num-changed', () => {
      refreshLine();
      refreshReader();
    });
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshLine, refreshReader]);

  useEffect(() => {
    const listener = listen('config-change', () => refreshConfig());
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshConfig]);

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

  const computedStyle: React.CSSProperties = {
    fontSize: config?.font_size,
    fontFamily: config?.font_family,
    lineHeight: config?.line_height,
    fontWeight: config?.font_weight,
    color: config?.font_color,
    letterSpacing: config?.letter_spacing,
  };

  return (
    <div
      className={clsx('fixed h-screen w-screen', {
        'h-full bg-white': isFocus,
      })}
      style={computedStyle}
    >
      {reader ? line : <p>请从托盘菜单打开一本小说</p>}
    </div>
  );
}
