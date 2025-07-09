import { useControllableValue } from 'ahooks';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface FontWeightSelectorProps {
  value: number;
  onChange: (value: number) => void;
  defaultValue?: number;
}

const FontWeightSelector: React.FC<FontWeightSelectorProps> = (props) => {
  const [value, onChange] = useControllableValue(props);

  const handleChange = (value: string) => {
    onChange(parseInt(value));
  };

  return (
    <Select value={String(value)} onValueChange={handleChange}>
      <SelectTrigger className="w-full">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="100">极细 (100)</SelectItem>
        <SelectItem value="200">特细 (200)</SelectItem>
        <SelectItem value="300">细 (300)</SelectItem>
        <SelectItem value="400">正常 (400)</SelectItem>
        <SelectItem value="500">中等 (500)</SelectItem>
        <SelectItem value="600">半粗 (600)</SelectItem>
        <SelectItem value="700">粗 (700)</SelectItem>
        <SelectItem value="800">特粗 (800)</SelectItem>
        <SelectItem value="900">极粗 (900)</SelectItem>
      </SelectContent>
    </Select>
  );
};

export default FontWeightSelector;
