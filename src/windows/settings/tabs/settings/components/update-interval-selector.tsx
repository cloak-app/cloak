import { useControllableValue } from 'ahooks';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface UpdateIntervalSelectorProps {
  value: number;
  onChange?: (value: number) => void;
  defaultValue?: number;
}

const UpdateIntervalSelector: React.FC<UpdateIntervalSelectorProps> = (
  props,
) => {
  const [value, onChange] = useControllableValue(props);

  const handleChange = (value: string) => {
    onChange(parseInt(value));
  };

  return (
    <Select value={String(value)} onValueChange={handleChange}>
      <SelectTrigger className="w-50">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="0">永不</SelectItem>
        <SelectItem value="14400">每 4 小时</SelectItem>
        <SelectItem value="28800">每 8 小时</SelectItem>
        <SelectItem value="43200">每 12 小时</SelectItem>
        <SelectItem value="86400">每 24 小时</SelectItem>
      </SelectContent>
    </Select>
  );
};

export default UpdateIntervalSelector;
