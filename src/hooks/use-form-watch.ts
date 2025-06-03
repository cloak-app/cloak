import { useEffect } from 'react';
import { FieldValues, Path, UseFormReturn, useWatch } from 'react-hook-form';

export const useFormWatch = <T extends FieldValues>(
  form: UseFormReturn<T>,
  name: Path<T>,
  callback: (value: T[Path<T>]) => void,
): T[Path<T>] => {
  const value = useWatch({ control: form.control, name: name });

  useEffect(() => {
    callback(value);
  }, [value]);

  return value;
};
