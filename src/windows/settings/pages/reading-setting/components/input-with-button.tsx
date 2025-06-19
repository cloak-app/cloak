import { useControllableValue } from 'ahooks';
import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

interface InputWithButtonProps {
  value: number;
  onChange: (value: number) => void;
}

const InputWithButton: React.FC<InputWithButtonProps> = (props) => {
  const [value, onChange] = useControllableValue(props);

  const [inputValue, setInputValue] = useState(value);

  useEffect(() => {
    setInputValue(value);
  }, [value]);

  return (
    <div className="flex items-center gap-2">
      <Input
        className="w-20"
        type="number"
        min={0}
        value={inputValue}
        onChange={(e) => setInputValue(Number(e.target.value))}
      />
      <Button
        type="button"
        variant="outline"
        onClick={() => onChange(inputValue)}
      >
        确认
      </Button>
    </div>
  );
};

export default InputWithButton;
