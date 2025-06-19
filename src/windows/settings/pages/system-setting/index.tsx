import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { useRequest } from 'ahooks';
import { HelpCircle } from 'lucide-react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import ShortcutRecorder from './components/shortcut-recorder';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Skeleton } from '@/components/ui/skeleton';
import { Switch } from '@/components/ui/switch';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useFormWatch } from '@/hooks/use-form-watch';
import { Config } from '@/types';

const formSchema = z.object({
  dock_visibility: z.boolean(),
  always_on_top: z.boolean(),
  transparent: z.boolean(),
  next_line_shortcut: z.string(),
  prev_line_shortcut: z.string(),
  next_chapter_shortcut: z.string(),
  prev_chapter_shortcut: z.string(),
  boss_key_shortcut: z.string(),
});

const SystemSetting: React.FC = () => {
  const { loading } = useRequest(() => invoke<Config>('get_config'), {
    onSuccess: (data) => form.reset(data),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      dock_visibility: false,
      always_on_top: false,
      transparent: true,
    },
  });

  useFormWatch(form, 'dock_visibility', (dockVisibility) => {
    if (loading) return;
    invoke('set_dock_visibility', { dockVisibility });
  });

  useFormWatch(form, 'always_on_top', (alwaysOnTop) => {
    if (loading) return;
    invoke('set_always_on_top', { alwaysOnTop });
  });

  useFormWatch(form, 'transparent', async (transparent) => {
    if (loading) return;
    const forceReopen = await confirm('重新打开阅读器窗口后生效，是否继续？', {
      title: 'Cloak',
      kind: 'warning',
      okLabel: '立即重启',
      cancelLabel: '下次再说',
    });

    invoke('set_transparent', { transparent, forceReopen });
  });

  useFormWatch(form, 'next_line_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_next_line_shortcut', { shortcut });
  });

  useFormWatch(form, 'prev_line_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_prev_line_shortcut', { shortcut });
  });

  useFormWatch(form, 'next_chapter_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_next_chapter_shortcut', { shortcut });
  });

  useFormWatch(form, 'prev_chapter_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_prev_chapter_shortcut', { shortcut });
  });

  useFormWatch(form, 'boss_key_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_boss_key_shortcut', { shortcut });
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
        <div className="border-b pb-2 text-lg font-semibold">基本设置</div>
        <FormField
          control={form.control}
          name="dock_visibility"
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
          name="always_on_top"
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
        <div className="border-b pb-2 text-lg font-semibold">快捷键设置</div>
        <FormField
          control={form.control}
          name="next_line_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>下一页</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="prev_line_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>上一页</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="next_chapter_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>下一章</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="prev_chapter_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>上一章</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="boss_key_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem>
              <FormLabel>老板键</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
              <FormDescription>
                按下快捷键后，将隐藏所有打开的窗口，重新按下快捷键后恢复
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};

export default SystemSetting;
