import { invoke } from '@tauri-apps/api/core';
import { useControllableValue, useRequest } from 'ahooks';
import { FONT_NAME_MAP } from './config';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface FontSelectorProps {
  value: string;
  onChange: (value: string) => void;
  defaultValue?: string;
}

const FontSelector: React.FC<FontSelectorProps> = (props) => {
  const [value, onChange] = useControllableValue(props);

  const { data } = useRequest(() => invoke<string[]>('get_all_font_families'));

  const fontList = data
    ?.map((item) => ({
      label: FONT_NAME_MAP[item] || item,
      value: item,
    }))
    .sort((a, b) => a.label.localeCompare(b.label));

  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="w-full">
        <SelectValue placeholder="请选择字体" />
      </SelectTrigger>
      <SelectContent>
        {fontList?.map((font) => (
          <SelectItem
            key={font.value}
            value={font.value}
            style={{ fontFamily: font.value }}
          >
            {font.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export default FontSelector;
