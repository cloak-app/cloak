import { invoke } from '@tauri-apps/api/core';
import { Palette } from 'lucide-react';
import { SubFormProps } from './types';
import LanguageSelector from '../components/language-selector';
import ThemeSelector from '../components/theme-selector';
import { Theme, useTheme } from '@/components/theme-provider';
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
import { useFormWatch } from '@/hooks/use-form-watch';

const PreferenceSettingsForm: React.FC<SubFormProps> = (props) => {
  const { form } = props;

  const { setTheme } = useTheme();

  useFormWatch(form, 'auto_start', (autoStart) => {
    invoke('set_auto_start', { autoStart });
  });

  useFormWatch(form, 'language', (language) => {
    invoke('set_language', { language });
  });

  useFormWatch(form, 'theme', (theme) => {
    invoke('set_theme', { theme });
    setTheme(theme as Theme);
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Palette size={20} />
          偏好设置
        </CardTitle>
        <CardDescription>自定义应用偏好</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <FormField
          control={form.control}
          name="auto_start"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>开机自动启动</FormLabel>
                <FormDescription>
                  开启后，阅读器将自动在系统启动时启动
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
          name="language"
          render={({ field }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>语言偏好</FormLabel>
                <FormDescription>选择阅读器默认语言</FormDescription>
              </div>
              <FormControl>
                <LanguageSelector disabled {...field} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="theme"
          render={({ field }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>主题偏好</FormLabel>
                <FormDescription>选择阅读器默认主题</FormDescription>
              </div>
              <FormControl>
                <ThemeSelector {...field} />
              </FormControl>
            </FormItem>
          )}
        />
      </CardContent>
    </Card>
  );
};

export default PreferenceSettingsForm;
