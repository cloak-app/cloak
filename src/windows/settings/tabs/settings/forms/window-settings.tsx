import { invoke } from '@tauri-apps/api/core';
import { Eye, HelpCircle } from 'lucide-react';
import { toast } from 'sonner';
import { SubFormProps } from './types';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
} from '@/components/ui/form';
import { Switch } from '@/components/ui/switch';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { isWindows } from '@/constants';
import { useFormWatch } from '@/hooks/use-form-watch';

const WindowSettingsForm: React.FC<SubFormProps> = (props) => {
  const { form } = props;

  useFormWatch(form, 'dock_visibility', async (dockVisibility) => {
    console.log('run');

    await invoke('set_dock_visibility', { dockVisibility });

    if (isWindows) {
      toast.success('设置成功', {
        closeButton: true,
        description: '重新打开阅读器窗口后生效',
        duration: Infinity,
        action: {
          label: '立即重启',
          onClick: () => invoke('reopen_reader_window'),
        },
      });
    }
  });

  useFormWatch(form, 'always_on_top', (alwaysOnTop) => {
    invoke('set_always_on_top', { alwaysOnTop });
  });

  useFormWatch(form, 'transparent', async (transparent) => {
    invoke('set_transparent', { transparent });
    toast.success('设置成功', {
      closeButton: true,
      description: '重新打开阅读器窗口后生效',
      duration: Infinity,
      action: {
        label: '立即重启',
        onClick: () => invoke('reopen_reader_window'),
      },
    });
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Eye size={20} />
          窗口设置
        </CardTitle>
        <CardDescription>自定义应用窗口行为</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <FormField
          control={form.control}
          name="dock_visibility"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>
                  是否显示任务栏/Dock 栏图标
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <HelpCircle size={14} className="text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent>
                      在 Windows 下，需要重新打开窗口才会生效
                    </TooltipContent>
                  </Tooltip>
                </FormLabel>
                <FormDescription>
                  开启后，应用程序图标将始终显示在任务栏/Dock 栏
                </FormDescription>
              </div>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="always_on_top"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>总在最前</FormLabel>
                <FormDescription>
                  开启后，应用程序将始终显示在其他窗口之上
                </FormDescription>
              </div>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="transparent"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
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
                <FormDescription>开启后，阅读器窗口将变为透明</FormDescription>
              </div>
              <FormControl>
                <Switch checked={value} onCheckedChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
      </CardContent>
    </Card>
  );
};

export default WindowSettingsForm;
