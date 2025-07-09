import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { BookOpen, HelpCircle } from 'lucide-react';
import { SubFormProps } from './types';
import ColorPicker from '../components/color-picker';
import FontSelector from '../components/font-selector';
import FontWeightSelector from '../components/font-weight-selector';
import InputWithButton from '../components/input-with-button';
import SliderWithAxis from '../components/slider-with-axis';
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
import { Separator } from '@/components/ui/separator';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useFormWatch } from '@/hooks/use-form-watch';

const ReadingSettingsForm: React.FC<SubFormProps> = (props) => {
  const { form } = props;

  useFormWatch(form, 'line_size', async (lineSize) => {
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
    invoke('set_font_size', { fontSize });
  });

  useFormWatch(form, 'font_color', (fontColor) => {
    invoke('set_font_color', { fontColor });
  });

  useFormWatch(form, 'letter_spacing', (letterSpacing) => {
    invoke('set_letter_spacing', { letterSpacing });
  });

  useFormWatch(form, 'font_weight', (fontWeight) => {
    invoke('set_font_weight', { fontWeight });
  });

  useFormWatch(form, 'font_family', (fontFamily) => {
    invoke('set_font_family', { fontFamily });
  });

  useFormWatch(form, 'line_height', (lineHeight) => {
    invoke('set_line_height', { lineHeight });
  });

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
            <FormItem className="col-span-2 flex items-center justify-between">
              <div className="space-y-2">
                <FormLabel>
                  每页字数
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <HelpCircle size={14} className="text-muted-foreground" />
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
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="font_size"
          render={({ field }) => (
            <FormItem className="space-y-2">
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
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="font_color"
          render={({ field }) => (
            <FormItem className="space-y-2">
              <FormLabel>字体颜色</FormLabel>
              <FormControl>
                <ColorPicker {...field} />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="letter_spacing"
          render={({ field }) => (
            <FormItem className="space-y-2">
              <FormLabel>字间距</FormLabel>
              <FormControl>
                <SliderWithAxis {...field} max={5} min={-1} step={0.1} />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="font_weight"
          render={({ field }) => (
            <FormItem className="space-y-2">
              <FormLabel>字体粗细</FormLabel>
              <FormControl>
                <FontWeightSelector {...field} />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="font_family"
          render={({ field }) => (
            <FormItem className="space-y-2">
              <FormLabel>
                字体
                <Tooltip>
                  <TooltipTrigger asChild>
                    <HelpCircle size={14} className="text-muted-foreground" />
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
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="line_height"
          render={({ field }) => (
            <FormItem className="space-y-2">
              <FormLabel>行高</FormLabel>
              <FormControl>
                <SliderWithAxis {...field} max={3} min={1} step={0.1} />
              </FormControl>
            </FormItem>
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
  );
};

export default ReadingSettingsForm;
