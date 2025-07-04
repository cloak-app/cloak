import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';
import { useRequest } from 'ahooks';
import {
  BookOpen,
  Eye,
  HelpCircle,
  Keyboard,
  Palette,
  Trash2,
  TriangleAlert,
} from 'lucide-react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';
import ColorPicker from './components/color-picker';
import FontSelector from './components/font-selector';
import FontWeightSelector from './components/font-weight-selector';
import InputWithButton from './components/input-with-button';
import LanguageSelector from './components/language-selector';
import ShortcutRecorder from './components/shortcut-recorder';
import SliderWithAxis from './components/slider-with-axis';
import ThemeSelector from './components/theme-selector';
import { formSchema } from './config';
import { Theme, useTheme } from '@/components/theme-provider';
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
  FormControl,
  FormDescription,
  FormField,
  FormLabel,
} from '@/components/ui/form';
import { Separator } from '@/components/ui/separator';
import { Switch } from '@/components/ui/switch';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useFormWatch } from '@/hooks/use-form-watch';
import { Config } from '@/types';

const SettingsTab: React.FC = () => {
  const { loading, refresh } = useRequest(() => invoke<Config>('get_config'), {
    onSuccess: (data) => form.reset(data),
  });

  const { setTheme } = useTheme();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  useFormWatch(form, 'auto_check_update', (autoCheckUpdate) => {
    if (loading) return;
    invoke('set_auto_check_update', { autoCheckUpdate });
  });

  useFormWatch(form, 'auto_start', (autoStart) => {
    if (loading) return;
    invoke('set_auto_start', { autoStart });
  });

  useFormWatch(form, 'language', (language) => {
    if (loading) return;
    invoke('set_language', { language });
  });

  useFormWatch(form, 'theme', (theme) => {
    if (loading) return;
    invoke('set_theme', { theme });
    setTheme(theme as Theme);
  });

  useFormWatch(form, 'dock_visibility', (dockVisibility) => {
    if (loading) return;
    console.log('run');

    invoke('set_dock_visibility', { dockVisibility });
  });

  useFormWatch(form, 'always_on_top', (alwaysOnTop) => {
    if (loading) return;
    invoke('set_always_on_top', { alwaysOnTop });
  });

  useFormWatch(form, 'transparent', async (transparent) => {
    if (loading) return;
    invoke('set_transparent', { transparent });
    toast.success('设置成功', {
      closeButton: true,
      description: '重新打开阅读器窗口后生效',
      duration: Infinity,
      action: {
        label: '立即重启',
        onClick: () => relaunch(),
      },
    });
  });

  useFormWatch(form, 'line_size', async (lineSize) => {
    if (loading) return;

    const confirmation = await confirm(
      '由于每页字数修改，您在当前章节的阅读进度不会被保留，是否继续',
      {
        title: '温馨提示',
        kind: 'warning',
        okLabel: '确定',
        cancelLabel: '取消',
      },
    );

    if (confirmation) {
      invoke('set_line_size', { lineSize });
    }
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

  useFormWatch(form, 'toggle_reading_mode_shortcut', (shortcut) => {
    if (loading) return;
    invoke('set_toggle_reading_mode_shortcut', { shortcut });
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

  const [
    fontSize,
    fontFamily,
    lineHeight,
    fontWeight,
    fontColor,
    letterSpacing,
  ] = form.watch([
    'font_size',
    'font_family',
    'line_height',
    'font_weight',
    'font_color',
    'letter_spacing',
  ]);

  const computedStyle = {
    fontSize: `${fontSize}px`,
    fontFamily,
    lineHeight,
    fontWeight,
    color: fontColor,
    letterSpacing: `${letterSpacing}px`,
  };

  return (
    <Form {...form}>
      <form className="space-y-4">
        {/* 偏好设置 */}
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
              name="auto_check_update"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>自动检查更新</FormLabel>
                    <FormDescription>
                      开启后，阅读器将自动检查更新
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={value}
                      onCheckedChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="auto_start"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>开机自动启动</FormLabel>
                    <FormDescription>
                      开启后，阅读器将自动在系统启动时启动
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={value}
                      onCheckedChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="language"
              render={({ field }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>语言偏好</FormLabel>
                    <FormDescription>选择阅读器默认语言</FormDescription>
                  </div>
                  <FormControl>
                    <LanguageSelector {...field} />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="theme"
              render={({ field }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>主题偏好</FormLabel>
                    <FormDescription>选择阅读器默认主题</FormDescription>
                  </div>
                  <FormControl>
                    <ThemeSelector {...field} />
                  </FormControl>
                </div>
              )}
            />
          </CardContent>
        </Card>

        {/* 窗口设置 */}
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
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>
                      是否显示 Dock 图标
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <HelpCircle
                            size={14}
                            className="text-muted-foreground"
                          />
                        </TooltipTrigger>
                        <TooltipContent>仅在 macOS 生效</TooltipContent>
                      </Tooltip>
                    </FormLabel>
                    <FormDescription>
                      开启后，应用程序图标将始终显示在 Dock 栏
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={value}
                      onCheckedChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="always_on_top"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>总在最前</FormLabel>
                    <FormDescription>
                      开启后，应用程序将始终显示在其他窗口之上
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={value}
                      onCheckedChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="transparent"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="flex items-center justify-between">
                  <div className="space-y-1">
                    <FormLabel>
                      透明窗口
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <HelpCircle
                            size={14}
                            className="text-muted-foreground"
                          />
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>重新打开阅读器窗口后生效</p>
                        </TooltipContent>
                      </Tooltip>
                    </FormLabel>
                    <FormDescription>
                      开启后，阅读器窗口将变为透明
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={value}
                      onCheckedChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
          </CardContent>
        </Card>

        {/* 快捷键设置 */}
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
                <div className="space-y-2">
                  <FormLabel>下一页</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="prev_line_shortcut"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="space-y-2">
                  <FormLabel>上一页</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="next_chapter_shortcut"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="space-y-2">
                  <FormLabel>下一章</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="prev_chapter_shortcut"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="space-y-2">
                  <FormLabel>上一章</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />

            <Separator className="col-span-2" />

            <FormField
              control={form.control}
              name="boss_key_shortcut"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="space-y-2">
                  <FormLabel>老板键</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="toggle_reading_mode_shortcut"
              render={({ field: { value, onChange, ...rest } }) => (
                <div className="space-y-2">
                  <FormLabel>切换阅读模式</FormLabel>
                  <FormControl>
                    <ShortcutRecorder
                      value={value}
                      onChange={onChange}
                      {...rest}
                    />
                  </FormControl>
                </div>
              )}
            />
          </CardContent>
        </Card>

        {/* 阅读设置 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BookOpen size={20} />
              阅读设置
            </CardTitle>
            <CardDescription>自定义你的阅读体验</CardDescription>
          </CardHeader>
          <CardContent className="grid grid-cols-2 gap-4">
            <FormField
              control={form.control}
              name="line_size"
              render={({ field }) => (
                <div className="col-span-2 flex items-center justify-between">
                  <div className="space-y-2">
                    <FormLabel>
                      每页字数
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <HelpCircle
                            size={14}
                            className="text-muted-foreground"
                          />
                        </TooltipTrigger>
                        <TooltipContent>
                          标题行默认不做处理，不排除有部分老六作者写很长的标题
                        </TooltipContent>
                      </Tooltip>
                    </FormLabel>
                    <FormDescription>
                      每页字数变更后，会将阅读位置重置到当前章节的开头
                    </FormDescription>
                  </div>
                  <FormControl>
                    <InputWithButton {...field} />
                  </FormControl>
                </div>
              )}
            />

            <FormField
              control={form.control}
              name="font_size"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>字体大小</FormLabel>
                  <FormControl>
                    <SliderWithAxis
                      {...field}
                      min={10}
                      max={32}
                      step={1}
                      unit="px"
                    />
                  </FormControl>
                </div>
              )}
            />
            <FormField
              control={form.control}
              name="font_color"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>字体颜色</FormLabel>
                  <FormControl>
                    <ColorPicker {...field} />
                  </FormControl>
                </div>
              )}
            />

            <FormField
              control={form.control}
              name="letter_spacing"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>字间距</FormLabel>
                  <FormControl>
                    <SliderWithAxis {...field} max={5} min={-1} step={0.1} />
                  </FormControl>
                </div>
              )}
            />

            <FormField
              control={form.control}
              name="font_weight"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>字体粗细</FormLabel>
                  <FormControl>
                    <FontWeightSelector {...field} />
                  </FormControl>
                </div>
              )}
            />

            <FormField
              control={form.control}
              name="font_family"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>
                    字体
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <HelpCircle
                          size={14}
                          className="text-muted-foreground"
                        />
                      </TooltipTrigger>
                      <TooltipContent>
                        部分中文繁体字体可能无法识别名称，如苹方字体仅识别简体
                        `PingFang SC`，不识别 `PingFang HK`
                      </TooltipContent>
                    </Tooltip>
                  </FormLabel>
                  <FormControl>
                    <FontSelector {...field} />
                  </FormControl>
                </div>
              )}
            />

            <FormField
              control={form.control}
              name="line_height"
              render={({ field }) => (
                <div className="space-y-2">
                  <FormLabel>行高</FormLabel>
                  <FormControl>
                    <SliderWithAxis {...field} max={3} min={1} step={0.1} />
                  </FormControl>
                </div>
              )}
            />

            <Separator className="col-span-2" />

            {/* 预览区域 */}
            <div className="col-span-2 p-4 border rounded-lg bg-accent">
              <h4 className="text-sm font-medium mb-2">预览效果</h4>
              <div className="text-justify" style={computedStyle}>
                这是一段示例文本，用于预览当前的字体设置效果。你可以调整上方的各项参数来查看实时的变化效果。通过这个预览区域，你可以直观地看到字体大小、颜色、间距、粗细等设置对阅读体验的影响。
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="border-destructive/50">
          <CardHeader>
            <CardTitle className="text-destructive flex items-center gap-2">
              <TriangleAlert size={20} />
              危险区域
            </CardTitle>
            <CardDescription>不可逆且破坏性操作</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between">
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
            </div>
          </CardContent>
        </Card>
      </form>
    </Form>
  );
};

export default SettingsTab;
