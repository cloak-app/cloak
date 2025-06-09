import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Skeleton } from '@/components/ui/skeleton';

import { Config } from '@/types';

const formSchema = z.object({
  font_size: z.number(),
  font_family: z.string(),
  line_height: z.number(),
  font_weight: z.number(),
  font_style: z.string(),
  font_color: z.string(),
  background_color: z.string(),
});

const ReadingSetting: React.FC = () => {
  const { loading } = useRequest(() => invoke<Config>('get_config'), {
    onSuccess: (data) => form.reset(data),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  if (loading)
    return (
      <div className="space-y-8">
        {Array.from({ length: 3 }).map((_, index) => (
          <div key={index} className="space-y-2">
            <div className="flex items-center gap-2">
              <Skeleton className="h-5 w-32" />
              <Skeleton className="h-4 w-4" />
            </div>
            <Skeleton className="h-6 w-12" />
            <Skeleton className="h-4 w-64" />
          </div>
        ))}
      </div>
    );

  return (
    <Form {...form}>
      <form className="space-y-8 p-4">
        <FormField
          control={form.control}
          name="font_size"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字体大小</FormLabel>
              <FormControl>
                <Input className="w-20" type="number" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};

export default ReadingSetting;
