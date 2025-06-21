import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from '@tanstack/react-table';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useRequest } from 'ahooks';
import { MoreHorizontal, Plus, Trash2 } from 'lucide-react';
import { toast } from 'sonner';
import { defaultColumns } from './columns';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Novel } from '@/types';

const NovelList: React.FC = () => {
  const { data, refresh } = useRequest(() => invoke<Novel[]>('get_novel_list'));

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

  const handleAdd = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'txt', extensions: ['txt'] }],
    });
    toast.promise(invoke('add_novel', { path: file }), {
      loading: '添加小说中...',
      success: '添加成功！',
      error: '添加失败！',
      finally: () => refresh(),
    });
  };

  const handleOpenFileDirectory = (path: string) => {
    invoke('show_in_folder', { path });
  };

  const columns: ColumnDef<Novel>[] = [
    ...defaultColumns,
    {
      id: 'actions',
      header: '操作',
      cell: ({ row }) => {
        const novel = row.original;
        return (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="data-[state=open]:bg-muted size-8"
              >
                <MoreHorizontal />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-[160px]">
              <DropdownMenuLabel>操作</DropdownMenuLabel>
              <DropdownMenuItem onClick={() => handleOpen(novel.id)}>
                阅读
              </DropdownMenuItem>
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
        );
      },
    },
  ];

  const table = useReactTable({
    data: data ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });

  return (
    <div className="p-4">
      <div className="flex items-center justify-between py-4">
        <Input
          placeholder="输入小说名称搜索..."
          value={(table.getColumn('title')?.getFilterValue() as string) ?? ''}
          onChange={(event) =>
            table.getColumn('title')?.setFilterValue(event.target.value)
          }
          className="w-50"
        />
        <Button onClick={handleAdd}>
          <Plus />
          添加小说
        </Button>
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext(),
                          )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && 'selected'}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  暂无小说，请点击右上角按钮导入小说
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
};

export default NovelList;
