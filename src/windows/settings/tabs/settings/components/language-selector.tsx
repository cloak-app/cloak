import { useControllableValue } from 'ahooks';
import { Languages } from 'lucide-react';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface LanguageSelectorProps {
  disabled?: boolean;
  value: string;
  onChange: (value: string) => void;
  defaultValue?: string;
}

const LanguageSelector: React.FC<LanguageSelectorProps> = (props) => {
  const { disabled } = props;
  const [value, onChange] = useControllableValue(props);

  return (
    <Select value={value} onValueChange={onChange} disabled={disabled}>
      <SelectTrigger className="relative w-50">
        <div className="flex items-center gap-2">
          <Languages size={20} />
          <SelectValue />
        </div>
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="zh-CN">简体中文</SelectItem>
        <SelectItem value="en-US">English</SelectItem>
      </SelectContent>
    </Select>
  );
};

export default LanguageSelector;
