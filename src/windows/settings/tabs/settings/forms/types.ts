import { UseFormReturn } from 'react-hook-form';
import { z } from 'zod';
import { formSchema } from '../config';

export interface SubFormProps {
  form: UseFormReturn<z.infer<typeof formSchema>>;
}
