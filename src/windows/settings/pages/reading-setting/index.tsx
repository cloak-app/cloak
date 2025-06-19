import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { HexColorPicker } from 'react-colorful';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import FontSelector from './components/font-selector';
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
import { useFormWatch } from '@/hooks/use-form-watch';
import { Config } from '@/types';

const formSchema = z.object({
  font_size: z.coerce.number(),
  font_family: z.string(),
  line_height: z.coerce.number(),
  font_weight: z.coerce.number(),
  font_color: z.string(),
  letter_spacing: z.coerce.number(),
});

const ReadingSetting: React.FC = () => {
  const { loading } = useRequest(() => invoke<Config>('get_config'), {
    onSuccess: (data) => form.reset(data),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  useFormWatch(form, 'font_size', (fontSize) => {
    if (loading) return;
    invoke('set_font_size', { fontSize: Number(fontSize) });
  });

  useFormWatch(form, 'font_color', (fontColor) => {
    if (loading) return;
    invoke('set_font_color', { fontColor });
  });

  useFormWatch(form, 'letter_spacing', (letterSpacing) => {
    if (loading) return;
    invoke('set_letter_spacing', { letterSpacing: Number(letterSpacing) });
  });

  useFormWatch(form, 'font_weight', (fontWeight) => {
    if (loading) return;
    invoke('set_font_weight', { fontWeight: Number(fontWeight) });
  });

  useFormWatch(form, 'font_family', (fontFamily) => {
    if (loading) return;
    invoke('set_font_family', { fontFamily });
  });

  useFormWatch(form, 'line_height', (lineHeight) => {
    if (loading) return;
    invoke('set_line_height', { lineHeight: Number(lineHeight) });
  });

  if (loading)
    return (
      <div className="space-y-4">
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
      <form className="space-y-4 w-full flex flex-col gap-4 p-4 h-full overflow-y-auto">
        <FormField
          control={form.control}
          name="font_size"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字体大小</FormLabel>
              <FormControl>
                <Input
                  className="w-20"
                  type="number"
                  min={10}
                  max={100}
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="font_color"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字体颜色</FormLabel>
              <FormControl>
                <HexColorPicker color={field.value} onChange={field.onChange} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="letter_spacing"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字间距</FormLabel>
              <FormControl>
                <Input
                  className="w-20"
                  type="number"
                  min={0}
                  max={30}
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="font_weight"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字体粗细</FormLabel>
              <FormControl>
                <Input
                  className="w-20"
                  type="number"
                  min={100}
                  max={500}
                  step={100}
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="font_family"
          render={({ field }) => (
            <FormItem>
              <FormLabel>字体</FormLabel>
              <FormControl>
                <FontSelector {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="line_height"
          render={({ field }) => (
            <FormItem>
              <FormLabel>行高</FormLabel>
              <FormControl>
                <Input
                  className="w-20"
                  type="number"
                  min={0}
                  step={0.1}
                  {...field}
                />
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
