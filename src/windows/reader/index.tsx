import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
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
    });

    const blurListener = win.listen('tauri://blur', () => {
      setIsFocus(false);
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
    <>
      <div
        className="fixed h-screen w-screen p-1 select-none hover:cursor-default"
        style={computedStyle}
        data-tauri-drag-region
      >
        {reader ? line : <p>请从托盘菜单打开一本小说</p>}
      </div>
      {isFocus && (
        <>
          <div className="fixed top-0.5 left-0.5">
            <div className="w-3 h-0.5 bg-primary" />
            <div className="w-0.5 h-3 bg-primary" />
          </div>
          <div className="fixed top-0.5 right-0.5">
            <div className="w-3 h-0.5 bg-primary ml-auto" />
            <div className="w-0.5 h-3 bg-primary ml-auto" />
          </div>
          <div className="fixed bottom-0.5 left-0.5">
            <div className="w-0.5 h-3 bg-primary" />
            <div className="w-3 h-0.5 bg-primary" />
          </div>
          <div className="fixed bottom-0.5 right-0.5">
            <div className="w-0.5 h-3 bg-primary ml-auto" />
            <div className="w-3 h-0.5 bg-primary ml-auto" />
          </div>
        </>
      )}
    </>
  );
}
