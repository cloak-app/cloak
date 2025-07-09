import { invoke } from '@tauri-apps/api/core';
import { Keyboard } from 'lucide-react';
import { SubFormProps } from './types';
import ShortcutRecorder from '../components/shortcut-recorder';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from '@/components/ui/form';
import { Separator } from '@/components/ui/separator';
import { useFormWatch } from '@/hooks/use-form-watch';

const ShortcutSettingsForm: React.FC<SubFormProps> = (props) => {
  const { form } = props;

  useFormWatch(form, 'next_line_shortcut', (shortcut) => {
    invoke('set_next_line_shortcut', { shortcut });
  });

  useFormWatch(form, 'prev_line_shortcut', (shortcut) => {
    invoke('set_prev_line_shortcut', { shortcut });
  });

  useFormWatch(form, 'next_chapter_shortcut', (shortcut) => {
    invoke('set_next_chapter_shortcut', { shortcut });
  });

  useFormWatch(form, 'prev_chapter_shortcut', (shortcut) => {
    invoke('set_prev_chapter_shortcut', { shortcut });
  });

  useFormWatch(form, 'boss_key_shortcut', (shortcut) => {
    invoke('set_boss_key_shortcut', { shortcut });
  });

  useFormWatch(form, 'toggle_reading_mode_shortcut', (shortcut) => {
    invoke('set_toggle_reading_mode_shortcut', { shortcut });
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Keyboard size={20} />
          快捷键设置
        </CardTitle>
        <CardDescription>自定义键盘快捷键</CardDescription>
      </CardHeader>
      <CardContent className="grid grid-cols-2 gap-4">
        <FormField
          control={form.control}
          name="next_line_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>下一页</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="prev_line_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>上一页</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="next_chapter_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>下一章</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="prev_chapter_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>上一章</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />

        <Separator className="col-span-2" />

        <FormField
          control={form.control}
          name="boss_key_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>老板键</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="toggle_reading_mode_shortcut"
          render={({ field: { value, onChange, ...rest } }) => (
            <FormItem className="space-y-2">
              <FormLabel>切换阅读模式</FormLabel>
              <FormControl>
                <ShortcutRecorder value={value} onChange={onChange} {...rest} />
              </FormControl>
            </FormItem>
          )}
        />
      </CardContent>
    </Card>
  );
};

export default ShortcutSettingsForm;
