import { getCurrentWindow } from '@tauri-apps/api/window';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import './styles.less';
import { Typography } from 'lessline';

interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
}

export default function ReaderWindow() {
  const [isFocus, setIsFocus] = useState(false);

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
    <div className={clsx('reader-window')}>
      <Typography.Paragraph>请从托盘菜单打开一本小说</Typography.Paragraph>
    </div>
  );
}
