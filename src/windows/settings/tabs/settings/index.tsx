import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';
import { useRequest } from 'ahooks';
import { Trash2, TriangleAlert } from 'lucide-react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';
import { formSchema } from './config';
import PreferenceSettingsForm from './forms/preference-settings';
import ReadingSettingsForm from './forms/reading-settings';
import ShortcutSettingsForm from './forms/shortcut-settings';
import UpdateSettingsForm from './forms/update-settings';
import WindowSettingsForm from './forms/window-settings';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Form,
  FormDescription,
  FormItem,
  FormLabel,
} from '@/components/ui/form';
import { Skeleton } from '@/components/ui/skeleton';
import { Config } from '@/types';

const SettingsTab: React.FC = () => {
  const { loading, refresh } = useRequest(() => invoke<Config>('get_config'), {
    onSuccess: (data) => form.reset(data),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  const handleReset = async () => {
    const confirmation = await confirm(
      '您的所有设置将被重置为默认值，是否继续',
      {
        title: 'Cloak',
        kind: 'warning',
        okLabel: '确定',
        cancelLabel: '取消',
      },
    );

    if (confirmation) {
      await invoke('reset_config');
      refresh();
      toast.success('重置成功', {
        closeButton: true,
        description: '部分设置将于重启后生效',
        duration: Infinity,
        action: {
          label: '立即重启',
          onClick: () => relaunch(),
        },
      });
    }
  };

  if (loading) {
    return <Skeleton className="w-full h-full" />;
  }

  return (
    <Form {...form}>
      <form className="space-y-4">
        {/* 偏好设置 */}
        <PreferenceSettingsForm form={form} />
        {/* 更新设置 */}
        <UpdateSettingsForm form={form} />
        {/* 窗口设置 */}
        <WindowSettingsForm form={form} />
        {/* 快捷键设置 */}
        <ShortcutSettingsForm form={form} />
        {/* 阅读设置 */}
        <ReadingSettingsForm form={form} />
        {/* 危险区域 */}
        <Card className="border-destructive/50">
          <CardHeader>
            <CardTitle className="text-destructive flex items-center gap-2">
              <TriangleAlert size={20} />
              危险区域
            </CardTitle>
            <CardDescription>不可逆且破坏性操作</CardDescription>
          </CardHeader>
          <CardContent>
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>重置所有设定</FormLabel>
                <FormDescription>
                  部分设置需要重新启动阅读器后生效
                </FormDescription>
              </div>
              <Button variant="destructive" onClick={handleReset} type="button">
                <Trash2 className="mr-2 h-4 w-4" />
                重置所有设定
              </Button>
            </FormItem>
          </CardContent>
        </Card>
      </form>
    </Form>
  );
};

export default SettingsTab;
