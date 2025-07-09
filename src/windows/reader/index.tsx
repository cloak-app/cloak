import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
import { Minus, X } from 'lucide-react';
import { useEffect, useState } from 'react';
import { cn } from '@/lib/utils';
import { Config, CustomEvent, Reader } from '@/types';

export default function ReaderWindow() {
  const {
    data: reader,
    refresh: refreshReader,
    mutate,
  } = useRequest(() => invoke<Reader>('get_novel_reader'), {
    onError: () => mutate(undefined),
  });

  const { data: readingMode, refresh: refreshReadingMode } = useRequest(() =>
    invoke<string>('get_reading_mode'),
  );

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
    const listener = listen(CustomEvent.ReaderChange, () => {
      refreshLine();
      refreshReader();
    });
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshLine, refreshReader]);

  useEffect(() => {
    const listener = listen(CustomEvent.ConfigChange, () => refreshConfig());
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshConfig]);

  useEffect(() => {
    const listener = listen(CustomEvent.ReadingModeChange, () =>
      refreshReadingMode(),
    );
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshReadingMode]);

  useEffect(() => {
    const focusListener = win.listen('tauri://focus', () => setIsFocus(true));
    const blurListener = win.listen('tauri://blur', () => setIsFocus(false));

    return () => {
      focusListener.then((unListen) => unListen());
      blurListener.then((unListen) => unListen());
    };
  }, [win]);

  const computedStyle: React.CSSProperties = {
    fontSize: `${config?.font_size}px`,
    fontFamily: config?.font_family,
    lineHeight: config?.line_height,
    fontWeight: config?.font_weight,
    color: config?.font_color,
    letterSpacing: `${config?.letter_spacing}px`,
  };

  return (
    <>
      <div
        className={cn(
          'fixed h-screen w-screen select-none border border-dashed border-transparent overflow-hidden p-1 rounded',
          isFocus && 'border-black/[0.2] bg-white',
        )}
        style={computedStyle}
        data-tauri-drag-region
      >
        {!reader ? <p>请从托盘菜单打开一本小说</p> : null}
        {reader && line ? line : null}
        {reader && reader.read_progress === 100 && !line ? '（已读完）' : null}
        {isFocus && (
          <div className="text-center text-sm text-muted-foreground mt-1">
            <p>{reader?.current_chapter.title}</p>
            <p>
              {reader?.read_position}/{reader?.total_lines}(
              {reader?.read_progress.toFixed(2)}%)
            </p>
          </div>
        )}
        {!readingMode ? (
          <p className="text-center text-sm text-destructive mt-1">
            阅读模式未开启
          </p>
        ) : null}
      </div>

      <div
        className={cn(
          'transition-opacity fixed top-2 right-2 flex gap-1 p-2 rounded-full bg-white/90 backdrop-blur-md shadow-md border border-gray-200',
          !isFocus && 'opacity-0',
        )}
      >
        <div
          className="w-5 h-5 bg-yellow-500 hover:bg-yellow-600 text-white rounded-full flex items-center justify-center cursor-pointer"
          onClick={() => win.minimize()}
        >
          <Minus size={10} />
        </div>
        <div
          className="w-5 h-5 bg-red-500 hover:bg-red-600 text-white rounded-full flex items-center justify-center cursor-pointer"
          onClick={() => win.close()}
        >
          <X size={10} />
        </div>
      </div>
    </>
  );
}
