import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { HelpCircle } from 'lucide-react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
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
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useFormWatch } from '@/hooks/use-form-watch';

const formSchema = z.object({
  dockVisibility: z.boolean(),
  alwaysOnTop: z.boolean(),
  transparent: z.boolean(),
});

const Config: React.FC = () => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      dockVisibility: false,
      alwaysOnTop: false,
      transparent: false,
    },
  });

  useFormWatch(form, 'dockVisibility', (visible) => {
    invoke('set_dock_visibility', { visible });
  });

  useFormWatch(form, 'alwaysOnTop', (alwaysOnTop) => {
    invoke('set_always_on_top', { alwaysOnTop });
  });

  useFormWatch(form, 'transparent', async (transparent) => {
    const forceReopen = await confirm('重新打开阅读器窗口后生效，是否继续？', {
      title: 'Tauri',
      kind: 'warning',
      okLabel: '立即重启',
      cancelLabel: '下次再说',
    });

    invoke('set_transparent', { transparent, forceReopen });
  });

  return (
    <Form {...form}>
      <form className="space-y-8">
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
                开启后，应用程序将始终显示在其他窗口之上
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="transparent"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>
                透明窗口
                <Tooltip>
                  <TooltipTrigger asChild>
                    <HelpCircle size={14} className="text-muted-foreground" />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>重新打开阅读器窗口后生效</p>
                  </TooltipContent>
                </Tooltip>
              </FormLabel>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
              <FormDescription>开启后，阅读器窗口将变为透明</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};

export default Config;
