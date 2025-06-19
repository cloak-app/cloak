import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
import { useEffect, useState } from 'react';
import { WindowTitleBar } from '@/components/titlebar';
import { Config, Novel } from '@/types';

export default function ReaderWindow() {
  const {
    data: reader,
    refresh: refreshReader,
    mutate,
  } = useRequest(() => invoke<Novel>('get_novel_reader'), {
    onError: () => mutate(undefined),
  });

  const { data: line, refresh: refreshLine } = useRequest(() =>
    invoke<string>('get_line'),
  );

  const { data: config, refresh: refreshConfig } = useRequest(() =>
    invoke<Config>('get_config'),
  );

  const [win] = useState(getCurrentWindow());

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
  }, [win]);

  const computedStyle: React.CSSProperties = {
    fontSize: config?.font_size,
    fontFamily: config?.font_family,
    lineHeight: config?.line_height,
    fontWeight: config?.font_weight,
    color: config?.font_color,
    letterSpacing: config?.letter_spacing,
  };

  return (
    <div className="fixed h-screen w-screen p-1 select-none hover:cursor-default">
      <WindowTitleBar hidden={!isFocus} />
      <div style={computedStyle}>
        {reader ? line : <p>请从托盘菜单打开一本小说</p>}
      </div>

      {isFocus && (
        <div>
          <div className="absolute top-0.5 left-0.5">
            <div className="w-3 h-0.5 bg-primary" />
            <div className="w-0.5 h-3 bg-primary" />
          </div>
          <div className="absolute top-0.5 right-0.5">
            <div className="w-3 h-0.5 bg-primary ml-auto" />
            <div className="w-0.5 h-3 bg-primary ml-auto" />
          </div>
          <div className="absolute bottom-0.5 left-0.5">
            <div className="w-0.5 h-3 bg-primary" />
            <div className="w-3 h-0.5 bg-primary" />
          </div>
          <div className="absolute bottom-0.5 right-0.5">
            <div className="w-0.5 h-3 bg-primary ml-auto" />
            <div className="w-3 h-0.5 bg-primary ml-auto" />
          </div>
        </div>
      )}
    </div>
  );
}
