import { invoke } from '@tauri-apps/api/core';
import { useControllableValue, useRequest } from 'ahooks';
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

  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="w-full">
        <SelectValue placeholder="请选择字体" />
      </SelectTrigger>
      <SelectContent>
        {data?.map((item) => (
          <SelectItem key={item} value={item}>
            {item}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export default FontSelector;
