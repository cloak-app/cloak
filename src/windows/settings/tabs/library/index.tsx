import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useRequest } from 'ahooks';
import { filesize } from 'filesize';
import {
  Book,
  File,
  FileText,
  HardDrive,
  MoreHorizontal,
  Plus,
  Trash2,
} from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { getNovelCover, getNovelFileExtension } from './helper';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Progress } from '@/components/ui/progress';
import { Search } from '@/components/ui/search';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Novel } from '@/types';

const LibraryTab: React.FC = () => {
  const { data: novels, refresh } = useRequest(() =>
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

  const handleOpen = (novel: Novel) => {
    if (novel.is_open) {
      invoke('open_reader_window');
    } else {
      toast.promise(invoke('open_novel', { id: novel.id }), {
        loading: '打开小说中...',
        success: '打开成功！',
        error: '打开失败！',
        finally: () => refresh(),
      });
    }
  };

  const handleAdd = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'novel', extensions: ['txt', 'epub'] }],
    });

    if (!file) return;

    toast.promise(invoke('add_novel', { path: file }), {
      loading: '添加小说中...',
      success: '添加成功！',
      error: '添加失败！',
      finally: () => refresh(),
    });
  };

  const handleShowInFolder = (path: string) => {
    invoke('reveal_item_in_dir', { path });
  };

  const [searchQuery, setSearchQuery] = useState('');

  const filteredNovels = novels?.filter((novel) => {
    const query = searchQuery.toLowerCase();
    return (
      novel.title.toLowerCase().includes(query) ||
      novel.author?.toLowerCase().includes(query) ||
      novel.description?.toLowerCase().includes(query)
    );
  });

  return (
    <>
      {/* 页面头部 */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-lg font-bold">小说列表</h1>
          <p className="text-sm text-muted-foreground">
            共 {novels?.length} 本小说
          </p>
        </div>

        <div className="flex items-center gap-4">
          <Search
            placeholder="搜索小说、作者..."
            className="w-64"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <Button onClick={handleAdd}>
            <Plus className="h-4 w-4 mr-2" />
            导入小说
          </Button>
        </div>
      </div>

      {/* 小说表格 */}
      <div className="border rounded-lg">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="min-w-[200px]">小说</TableHead>
              <TableHead>作者</TableHead>
              <TableHead>进度</TableHead>
              <TableHead>大小</TableHead>
              <TableHead>格式</TableHead>
              <TableHead></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredNovels?.map((novel) => (
              <TableRow key={novel.id}>
                <TableCell>
                  <div className="flex items-center gap-3">
                    <Avatar className="h-16 w-12 rounded flex-shrink-0">
                      <AvatarImage
                        src={getNovelCover(novel.cover) || undefined}
                        alt={novel.title}
                        className="object-cover"
                      />
                      <AvatarFallback className="rounded text-xs">
                        <FileText size={16} />
                      </AvatarFallback>
                    </Avatar>

                    <div className="w-0 flex-1 flex flex-col justify-center">
                      <div className="truncate text-sm font-medium space-x-1">
                        {!!novel.is_open && (
                          <Badge
                            variant="outline"
                            className="text-xs px-1 py-0"
                          >
                            在读
                          </Badge>
                        )}
                        <span>{novel.title}</span>
                      </div>
                      <div className="text-xs text-muted-foreground truncate">
                        {novel.description}
                      </div>
                    </div>
                  </div>
                </TableCell>
                <TableCell>
                  {novel.author ? (
                    <span className="text-sm">{novel.author}</span>
                  ) : (
                    <span className="text-sm text-muted-foreground">未知</span>
                  )}
                </TableCell>
                <TableCell>
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs">
                      <span className="text-muted-foreground">
                        {novel.read_progress.toFixed(2)}%
                      </span>
                      {novel.read_progress === 100 && (
                        <Badge variant="outline" className="text-xs px-1 py-0">
                          已读完
                        </Badge>
                      )}
                    </div>
                    <Progress value={novel.read_progress} className="h-1" />
                  </div>
                </TableCell>
                <TableCell>
                  <div className="flex items-center gap-1 text-sm text-muted-foreground">
                    <HardDrive size={12} />
                    {filesize(novel.file_size)}
                  </div>
                </TableCell>
                <TableCell>
                  <div className="flex items-center gap-1 text-sm text-muted-foreground">
                    <File size={12} />
                    {getNovelFileExtension(novel.path)}
                  </div>
                </TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button
                        variant="ghost"
                        className="data-[state=open]:bg-muted"
                      >
                        <MoreHorizontal size={14} />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>操作</DropdownMenuLabel>
                      <DropdownMenuItem onClick={() => handleOpen(novel)}>
                        继续阅读
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        onClick={() => handleShowInFolder(novel.path)}
                      >
                        在文件夹中显示
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem
                        variant="destructive"
                        onClick={() => handleDelete(novel.id)}
                      >
                        <Trash2 size={14} />
                        删除
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      {/* 空状态 */}
      {filteredNovels?.length === 0 && novels && novels?.length > 0 && (
        <div className="text-center py-12">
          <h3 className="text-lg font-medium mb-2">没有找到匹配的小说</h3>
          <p className="text-muted-foreground mb-4">试试其他关键词</p>
        </div>
      )}

      {!novels?.length && (
        <div className="text-center py-12">
          <Book size={48} className="text-muted-foreground mx-auto mb-4" />
          <h3 className="text-lg font-medium mb-2">还没有小说</h3>
          <p className="text-muted-foreground mb-4">
            导入你的第一本小说开始阅读吧
          </p>
        </div>
      )}
    </>
  );
};

export default LibraryTab;
