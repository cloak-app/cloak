import { ColumnDef } from '@tanstack/react-table';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { safeDivide } from '@/lib/number';
import { Novel } from '@/types';

export const defaultColumns: ColumnDef<Novel>[] = [
  {
    accessorKey: 'title',
    header: '小说名称',
    cell: ({ getValue }) => {
      const title = getValue() as string;
      return (
        <Tooltip>
          <TooltipTrigger className="max-w-50 truncate">{title}</TooltipTrigger>
          <TooltipContent>
            <p>{title}</p>
          </TooltipContent>
        </Tooltip>
      );
    },
  },
  {
    id: 'progress',
    header: '阅读进度',
    cell: ({ row }) => {
      const novel = row.original;
      const lastReadPosition = novel.last_read_position;
      const totalCharacters = novel.total_lines;
      const progress = safeDivide(lastReadPosition, totalCharacters) * 100;
      return (
        <div className="flex items-center gap-2">
          <Progress className="w-30" value={progress} />
          <span className="text-sm text-muted-foreground">
            {progress.toFixed(2)}%
          </span>
        </div>
      );
    },
  },
  {
    id: 'status',
    header: '状态',
    cell: ({ row }) => {
      const novel = row.original;
      const lastReadPosition = novel.last_read_position;
      const totalCharacters = novel.total_lines;
      const isFinished = lastReadPosition >= totalCharacters;

      return (
        <Badge variant={isFinished ? 'default' : 'outline'}>
          {isFinished ? '已读完' : '未读完'}
        </Badge>
      );
    },
  },
];
