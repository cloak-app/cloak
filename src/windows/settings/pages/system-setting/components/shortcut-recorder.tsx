import { useControllableValue, useEventListener } from 'ahooks';
import { useRef, useState } from 'react';
import { Kbd, KbdKey, KbdSeparator } from '@/components/ui/kbd';

interface ShortcutRecorderProps {
  value?: string[];
  onChange: (value?: string[]) => void;
  defaultValue?: string[];
}

const ShortcutRecorder: React.FC<ShortcutRecorderProps> = (props) => {
  const [shortcut, setShortcut] = useControllableValue<string[] | undefined>(
    props,
  );
  const [started, setStarted] = useState(false);

  const [recordedKeys, setRecordedKeys] = useState<string[]>([]);
  const pressedKeysRef = useRef<Set<string>>(new Set());

  useEventListener(
    'keydown',
    (e) => {
      e.preventDefault();
      console.log(e);

      pressedKeysRef.current.add(e.key);

      if (!recordedKeys.includes(e.key)) {
        setRecordedKeys((pre) => [...pre, e.key]);
      }
    },
    { enable: started },
  );

  useEventListener(
    'keyup',
    (e) => {
      e.preventDefault();
      pressedKeysRef.current.delete(e.key);

      if (pressedKeysRef.current.size === 0) {
        setStarted(false);
        setShortcut(recordedKeys);
        setRecordedKeys([]);
      }
    },
    { enable: started },
  );

  const handleStart = () => {
    setStarted(true);
  };

  const renderKeys = started ? recordedKeys : shortcut;

  return (
    <Kbd className="max-w-40" onClick={handleStart}>
      {started && !renderKeys?.length && <KbdKey>按下按键以开始录制</KbdKey>}
      {!started && !renderKeys?.length && <KbdKey>暂未设置快捷键</KbdKey>}

      {renderKeys?.map((key, index) => (
        <>
          <KbdKey>{key}</KbdKey>
          {index !== renderKeys.length - 1 ? <KbdSeparator /> : null}
        </>
      ))}
    </Kbd>
  );
};

export default ShortcutRecorder;
