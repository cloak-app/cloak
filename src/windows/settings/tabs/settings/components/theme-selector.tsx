import { useControllableValue } from 'ahooks';
import { Monitor, Moon, Sun } from 'lucide-react';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface ThemeSelectorProps {
  value: string;
  onChange: (value: string) => void;
  defaultValue?: string;
}

const ThemeSelector: React.FC<ThemeSelectorProps> = (props) => {
  const [value, onChange] = useControllableValue(props);

  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="w-50">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="light">
          <Sun size={20} />
          浅色
        </SelectItem>
        <SelectItem value="dark">
          <Moon size={20} />
          深色
        </SelectItem>
        <SelectItem value="system">
          <Monitor size={20} />
          跟随系统
        </SelectItem>
      </SelectContent>
    </Select>
  );
};

export default ThemeSelector;
