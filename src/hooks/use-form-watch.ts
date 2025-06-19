import { useLatest } from 'ahooks';
import { useEffect } from 'react';
import { FieldValues, Path, UseFormReturn } from 'react-hook-form';

type WatchValue<
  T extends FieldValues,
  P extends Path<T> | Path<T>[],
> = P extends Path<T> ? T[P] : { [K in P[number]]: T[K] };

export const useFormWatch = <
  T extends FieldValues,
  P extends Path<T> | Path<T>[],
>(
  form: UseFormReturn<T>,
  name: P,
  callback: (value: WatchValue<T, P>) => void,
): void => {
  const callbackRef = useLatest(callback);

  useEffect(() => {
    const subscription = form.subscribe({
      name,
      formState: { values: true },
      callback: ({ values }) => {
        if (Array.isArray(name)) {
          const result = name.reduce((acc, key) => {
            acc[key] = values[key];
            return acc;
          }, {} as WatchValue<T, P>);
          callbackRef.current(result);
        } else {
          callbackRef.current(values[name as Path<T>] as WatchValue<T, P>);
        }
      },
    });

    return () => subscription();
  }, [form, name, callbackRef]);
};
