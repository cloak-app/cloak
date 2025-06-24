import { useControllableValue, useEventListener } from 'ahooks';
import React, { useEffect, useState } from 'react';
import { formatKey, isModifierKey, normalizeKey } from './helper';
import { Input } from '@/components/ui/input';

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

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (!isEditing) return;
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
  };

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (!isEditing) return;
    const key = normalizeKey(e.code);
    setPressedKeys((prev) => prev.filter((k) => k !== key));
  };

  useEffect(() => {
    if (currentKeys.length && pressedKeys.length === 0 && isEditing) {
      setIsEditing(false);
      onChange(currentKeys.join('+'));
      setCurrentKeys([]);
      setPressedKeys([]);
    }
  }, [pressedKeys, currentKeys, isEditing, onChange]);

  useEventListener('blur', () => {
    setIsEditing(false);
    setCurrentKeys([]);
    setPressedKeys([]);
  });

  const handleStart = () => {
    setIsEditing(true);
    // TODO: 在开始设置快捷键时，应当前置取消所有快捷键的监听，并且提供一个已有快捷键的冲突检测
  };

  const renderKeys = isEditing ? currentKeys : value?.split('+');

  const formatPlaceholder = () => {
    if (isEditing && !renderKeys?.length) return '按下按键以开始录制';
    if (!isEditing && !renderKeys?.length) return '暂未设置快捷键';
  };

  const formatValueString = () => {
    return renderKeys?.map((key) => formatKey(key)).join(' + ');
  };

  return (
    <Input
      className="w-full text-center"
      readOnly
      onFocus={handleStart}
      onKeyDown={handleKeyDown}
      onKeyUp={handleKeyUp}
      value={formatValueString()}
      placeholder={formatPlaceholder()}
    />
  );
};

export default ShortcutRecorder;
