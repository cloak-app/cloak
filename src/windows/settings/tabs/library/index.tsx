import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useRequest } from 'ahooks';
import dayjs from 'dayjs';
import { filesize } from 'filesize';
import { MoreHorizontal, Plus, Trash2 } from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { getNovelStatus, NovelStatus, STATUS_BADGE_MAP } from './helper';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Progress } from '@/components/ui/progress';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Novel } from '@/types';

const LibraryTab: React.FC = () => {
  const { data: novelList, refresh } = useRequest(() =>
    invoke<Novel[]>('get_novel_list'),
  );

  const handleDelete = (id: number) => {
    toast.promise(invoke('delete_novel', { id }), {
      loading: '删除中...',
      success: '删除成功！',
      error: '删除失败！',
      finally: () => refresh(),
    });
  };

  const handleOpen = (id: number) => {
    toast.promise(invoke('open_novel', { id }), {
      loading: '打开小说中...',
      success: '打开成功！',
      error: '打开失败！',
      finally: () => refresh(),
    });
  };

  const handleContinue = () => {
    invoke('open_reader_window');
  };

  const handleAdd = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'txt', extensions: ['txt'] }],
    });

    if (!file) return;

    toast.promise(invoke('add_novel', { path: file }), {
      loading: '添加小说中...',
      success: '添加成功！',
      error: '添加失败！',
      finally: () => refresh(),
    });
  };

  const handleOpenFileDirectory = (path: string) => {
    invoke('reveal_item_in_dir', { path });
  };

  const [filter, setFilter] = useState('ALL');

  const filteredNovelList = novelList?.filter((novel) => {
    if (filter === 'ALL') return true;
    if (filter === NovelStatus.READING)
      return novel.read_progress > 0 && novel.read_progress < 100;
    if (filter === NovelStatus.READ) return novel.read_progress === 100;
    if (filter === NovelStatus.UN_READ) return novel.read_progress === 0;
    return false;
  });

  return (
    <>
      <div className="flex justify-between items-center">
        <div>
          <h3 className="text-lg font-semibold text-primary">我的小说库</h3>
          <p className="text-sm text-muted-foreground">
            共 {novelList?.length} 本小说
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="icon" onClick={handleAdd}>
            <Plus className="text-primary" />
          </Button>
          <Select value={filter} onValueChange={setFilter}>
            <SelectTrigger className="w-32 text-primary">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ALL">全部</SelectItem>
              <SelectItem value={NovelStatus.READING}>在读</SelectItem>
              <SelectItem value={NovelStatus.READ}>已完成</SelectItem>
              <SelectItem value={NovelStatus.UN_READ}>未读</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {filteredNovelList?.map((novel) => {
          const novelStatus = getNovelStatus(novel.read_progress);

          return (
            <Card key={novel.id} className="hover:shadow-md">
              <CardHeader>
                <CardTitle className="truncate">{novel.title}</CardTitle>
                <CardDescription>{filesize(novel.file_size)}</CardDescription>
                <CardAction>{STATUS_BADGE_MAP[novelStatus]}</CardAction>
              </CardHeader>
              <CardContent>
                {/* 添加时间和进度 */}
                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <span>
                    添加于 {dayjs(novel.created_at).format('YYYY-MM-DD')}
                  </span>
                  <span>{novel.read_progress.toFixed(2)}% 已读</span>
                </div>
                <Progress className="mt-2" value={novel.read_progress} />
              </CardContent>
              <CardFooter className="gap-2">
                {novel.is_open ? (
                  <Button className="flex-1" onClick={handleContinue}>
                    继续阅读
                  </Button>
                ) : (
                  <Button
                    variant="outline"
                    className="flex-1"
                    onClick={() => handleOpen(novel.id)}
                  >
                    开始阅读
                  </Button>
                )}
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button
                      variant="ghost"
                      className="data-[state=open]:bg-muted"
                    >
                      <MoreHorizontal />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent className="w-[160px]">
                    <DropdownMenuLabel>操作</DropdownMenuLabel>
                    <DropdownMenuItem
                      onClick={() => handleOpenFileDirectory(novel.path)}
                    >
                      打开小说目录
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                      variant="destructive"
                      onClick={() => handleDelete(novel.id)}
                    >
                      <Trash2 />
                      删除
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </CardFooter>
            </Card>
          );
        })}
        {filteredNovelList?.length === 0 && (
          <div className="col-span-2 flex justify-center items-center h-full">
            <p className="text-muted-foreground">暂无小说</p>
          </div>
        )}
      </div>
    </>
  );
};

export default LibraryTab;
