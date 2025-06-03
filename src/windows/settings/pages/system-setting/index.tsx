import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Switch } from '@/components/ui/switch';
import { useFormWatch } from '@/hooks/use-form-watch';
import { invoke } from '@tauri-apps/api/core';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { HelpCircle } from 'lucide-react';

const formSchema = z.object({
  dockVisibility: z.boolean(),
  alwaysOnTop: z.boolean(),
});

const Config: React.FC = () => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      dockVisibility: false,
      alwaysOnTop: false,
    },
  });

  useFormWatch(form, 'dockVisibility', async (visible) => {
    await invoke('set_dock_visibility', { visible });
  });

  useFormWatch(form, 'alwaysOnTop', async (alwaysOnTop) => {
    await invoke('set_always_on_top', { alwaysOnTop });
  });

  // 2. Define a submit handler.
  function onSubmit(values: z.infer<typeof formSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    console.log(values);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="dockVisibility"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>
                是否显示 Dock 图标
                <Tooltip>
                  <TooltipTrigger asChild>
                    <HelpCircle size={14} className="text-muted-foreground" />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>仅在 macOS 生效</p>
                  </TooltipContent>
                </Tooltip>
              </FormLabel>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
              <FormDescription>
                开启后，应用程序图标将始终显示在 Dock 栏
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="alwaysOnTop"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>总在最前</FormLabel>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
              <FormDescription>
                勾选后，应用程序将始终显示在其他窗口之上
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};

export default Config;
