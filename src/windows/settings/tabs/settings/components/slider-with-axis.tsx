import { useControllableValue } from 'ahooks';
import { Slider } from '@/components/ui/slider';

interface SliderWithAxisProps {
  min?: number;
  max?: number;
  value?: number;
  defaultValue?: number;
  onChange?: (value: number) => void;
  unit?: string;
  step?: number;
}

const SliderWithAxis: React.FC<SliderWithAxisProps> = (props) => {
  const { min, max, unit, step } = props;
  const [value, onChange] = useControllableValue(props);

  return (
    <div className="px-3">
      <Slider
        value={[value]}
        onValueChange={(v) => onChange(v[0])}
        className="w-full"
        min={min}
        max={max}
        step={step}
      />
      <div className="flex justify-between text-xs text-muted-foreground mt-1">
        <span>
          {min}
          {unit}
        </span>
        <span className="font-medium">
          {value}
          {unit}
        </span>
        <span>
          {max}
          {unit}
        </span>
      </div>
    </div>
  );
};

export default SliderWithAxis;
