import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useRequest } from 'ahooks';
import dayjs from 'dayjs';
import { HelpCircle, History } from 'lucide-react';
import { useEffect, useState } from 'react';
import { SubFormProps } from './types';
import UpdateIntervalSelector from '../components/update-interval-selector';
import { Button } from '@/components/ui/button';
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
import { CheckUpdateStatus, CustomEvent, UpdateCheckResult } from '@/types';

const UpdateSettingsForm: React.FC<SubFormProps> = (props) => {
  const { form } = props;

  const [isChecking, setIsChecking] = useState(false);

  const { data: lastCheckResult, runAsync: refreshLastCheckResult } =
    useRequest(() => invoke<UpdateCheckResult>('get_last_check_result'), {
      onSuccess(result) {
        if (result.status === CheckUpdateStatus.Checking) {
          setIsChecking(true);
        }
      },
    });

  // 监听更新检查状态变化事件
  useEffect(() => {
    const listener = listen<UpdateCheckResult>(
      CustomEvent.UpdateCheckStatusChange,
      (event) => {
        const result = event.payload;

        // 更新本地状态
        if (result.status === CheckUpdateStatus.Checking) {
          setIsChecking(true);
        } else {
          setIsChecking(false);
        }

        // 刷新检查结果
        refreshLastCheckResult();
      },
    );

    return () => {
      listener.then((unListen) => unListen());
    };
  }, [refreshLastCheckResult]);

  const checkUpdateInterval = form.watch('check_update_interval');

  useFormWatch(form, 'check_update_interval', async (interval) => {
    invoke('set_check_update_interval', { interval });
  });

  const handleCheckUpdate = async () => {
    setIsChecking(true);
    await invoke('check_update');
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <History size={20} />
          更新设置
        </CardTitle>
        <CardDescription>自动检查版本更新</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <FormField
          control={form.control}
          name="check_update_interval"
          render={({ field }) => (
            <FormItem className="flex items-center justify-between">
              <div className="space-y-1">
                <FormLabel>
                  自动检查更新
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <HelpCircle size={14} className="text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent>
                      应用使用 GitHub
                      托管版本，如遇检查失败，请检查代理配置，或手动修改 Hosts
                      文件
                    </TooltipContent>
                  </Tooltip>
                </FormLabel>
                <FormDescription>定期检查新版本</FormDescription>
              </div>
              <FormControl>
                <UpdateIntervalSelector {...field} />
              </FormControl>
            </FormItem>
          )}
        />
        {checkUpdateInterval !== 0 && (
          <>
            <Separator />
            <FormItem className="flex items-center justify-between">
              <FormLabel>更新状态</FormLabel>
              <Button
                size="sm"
                variant="outline"
                onClick={handleCheckUpdate}
                disabled={isChecking}
                type="button"
              >
                {isChecking ? '检查中...' : '手动检查'}
              </Button>
            </FormItem>

            <div className="space-y-2">
              <div className="flex items-center gap-2">
                {lastCheckResult?.status === CheckUpdateStatus.Success && (
                  <>
                    <div className="w-2 h-2 rounded-full bg-green-500" />
                    <span className="text-sm text-green-600">检查成功</span>
                  </>
                )}
                {lastCheckResult?.status === CheckUpdateStatus.Failed && (
                  <>
                    <div className="w-2 h-2 rounded-full bg-destructive" />
                    <span className="text-sm text-destructive">检查失败</span>
                  </>
                )}
                {lastCheckResult?.status === CheckUpdateStatus.Checking && (
                  <>
                    <div className="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
                    <span className="text-sm text-blue-600">检查中...</span>
                  </>
                )}
              </div>

              <div className="text-xs text-muted-foreground">
                上次更新：
                {dayjs.unix(lastCheckResult?.timestamp || 0).fromNow()}
              </div>

              {lastCheckResult?.status === CheckUpdateStatus.Failed && (
                <div className="text-xs text-destructive">
                  网络连接失败，请检查网络设置
                </div>
              )}
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
};

export default UpdateSettingsForm;
