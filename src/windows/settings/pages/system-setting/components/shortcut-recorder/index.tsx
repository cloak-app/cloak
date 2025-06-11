import { useControllableValue, useEventListener } from 'ahooks';
import React, { useEffect, useState } from 'react';
import { useHotkeys } from 'react-hotkeys-hook';
import { formatKey, isModifierKey, normalizeKey } from './helper';
import { Kbd, KbdKey, KbdSeparator } from '@/components/ui/kbd';

interface ShortcutRecorderProps {
  value?: string;
  onChange: (value?: string) => void;
  defaultValue?: string;
}

const ShortcutRecorder: React.FC<ShortcutRecorderProps> = (props) => {
  const [value, onChange] = useControllableValue<string | undefined>(props);
  const [isEditing, setIsEditing] = useState(false);
  const [pressedKeys, setPressedKeys] = useState<string[]>([]);
  const [currentKeys, setCurrentKeys] = useState<string[]>([]);

  useHotkeys(
    '*',
    (e) => {
      e.preventDefault();
      e.stopPropagation();

      const key = normalizeKey(e.code);

      // Update pressed keys
      setPressedKeys((prev) => [...prev, key]);

      setCurrentKeys((prev) => {
        const keys = [...new Set([...prev, key])];
        let modifiers = keys.filter((k) => isModifierKey(k));
        let nonModifiers = keys.filter((k) => !isModifierKey(k));

        if (modifiers.length > 2) {
          modifiers = modifiers.slice(0, 2);
        }

        if (nonModifiers.length > 2) {
          nonModifiers = nonModifiers.slice(0, 2);
        }

        // Combine modifiers and non-modifiers
        return [...modifiers, ...nonModifiers];
      });
    },
    {
      enabled: isEditing,
      keydown: true,
    },
    [isEditing, pressedKeys],
  );

  // Handle key up events
  useHotkeys(
    '*',
    (e) => {
      const key = normalizeKey(e.code);
      setPressedKeys((prev) => prev.filter((k) => k !== key));
    },
    {
      enabled: isEditing,
      keyup: true,
    },
    [isEditing, pressedKeys],
  );

  useEffect(() => {
    if (currentKeys.length && pressedKeys.length === 0 && isEditing) {
      setIsEditing(false);
      onChange(currentKeys.join('+'));
      setCurrentKeys([]);
      setPressedKeys([]);
    }
  }, [pressedKeys, currentKeys, isEditing, onChange]);

  useEventListener('blur', () => {
    console.log('blur');

    setIsEditing(false);
    setCurrentKeys([]);
    setPressedKeys([]);
  });

  const handleStart = () => {
    setIsEditing(true);
  };

  const renderKeys = isEditing ? currentKeys : value?.split('+');

  return (
    <Kbd className="max-w-40" onClick={handleStart}>
      {isEditing && !renderKeys?.length && <KbdKey>按下按键以开始录制</KbdKey>}
      {!isEditing && !renderKeys?.length && <KbdKey>暂未设置快捷键</KbdKey>}

      {renderKeys?.map((key, index) => (
        <React.Fragment key={key}>
          <KbdKey>{formatKey(key)}</KbdKey>
          {index !== renderKeys.length - 1 ? <KbdSeparator /> : null}
        </React.Fragment>
      ))}
    </Kbd>
  );
};

export default ShortcutRecorder;
