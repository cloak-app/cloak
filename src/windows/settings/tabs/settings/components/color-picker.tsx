import { useControllableValue } from 'ahooks';
import { Input } from '@/components/ui/input';

interface ColorPickerProps {
  value: string;
  onChange: (value: string) => void;
  defaultValue?: string;
}

const ColorPicker: React.FC<ColorPickerProps> = (props) => {
  const [value, onChange] = useControllableValue(props);

  return (
    <div className="flex gap-2">
      <Input
        type="color"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="w-16 h-10 p-1 border rounded"
      />
      <Input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="#000000"
        className="flex-1"
      />
    </div>
  );
};

export default ColorPicker;
