import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRequest } from 'ahooks';
import { useEffect, useState } from 'react';
import { WindowTitleBar } from '@/components/titlebar';
import { cn } from '@/lib/utils';
import { Config, Reader } from '@/types';

const Icon = ({ className, ...rest }: any) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      strokeWidth="1.5"
      stroke="currentColor"
      className={className}
      {...rest}
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v12m6-6H6" />
    </svg>
  );
};

export default function ReaderWindow() {
  const {
    data: reader,
    refresh: refreshReader,
    mutate,
  } = useRequest(() => invoke<Reader>('get_novel_reader'), {
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
    const listener = listen('reader-change', () => {
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
    fontSize: `${config?.font_size}px`,
    fontFamily: config?.font_family,
    lineHeight: config?.line_height,
    fontWeight: config?.font_weight,
    color: config?.font_color,
    letterSpacing: `${config?.letter_spacing}px`,
  };

  return (
    <div
      className={cn(
        'fixed top-3 bottom-3 left-3 right-3 select-none border border-transparent hover:cursor-default',
        isFocus && 'border-black/[0.2] dark:border-white/[0.2] bg-white',
      )}
    >
      <WindowTitleBar
        className={cn(
          'transition-opacity active:shadow-sm',
          !isFocus && 'opacity-0',
        )}
        data-tauri-drag-region
      />
      <div className="p-1" style={computedStyle}>
        {!reader ? <p>请从托盘菜单打开一本小说</p> : null}
        {reader && line ? line : null}
        {reader && reader.read_progress === 100 && !line ? '（已读完）' : null}
      </div>

      <div className={cn('transition-opacity', !isFocus && 'opacity-0')}>
        <Icon className="absolute h-6 w-6 -top-3 -left-3 text-black" />
        <Icon className="absolute h-6 w-6 -bottom-3 -left-3 text-black" />
        <Icon className="absolute h-6 w-6 -top-3 -right-3 text-black" />
        <Icon className="absolute h-6 w-6 -bottom-3 -right-3 text-black" />
      </div>
    </div>
  );
}
