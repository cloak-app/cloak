import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { open } from '@tauri-apps/plugin-dialog';
import { Progress } from '@/components/ui/progress';
import { safeDivide } from '@/lib/number';

interface Novel {
  id: string;
  title: string;
  path: string;
  last_read_position: number;
  total_characters: number;
}

const NovelList: React.FC = () => {
  const { data, refresh } = useRequest(() => invoke<Novel[]>('get_novel_list'));

  const handleDelete = (id: string) => {
    invoke('delete_novel', { id });
  };

  const handleAdd = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'txt', extensions: ['txt'] }],
    });
    await invoke('add_novel', { path: file });
    refresh();
  };

  console.log(data);

  return (
    <section>
      <div className="flex justify-end">
        <Button onClick={handleAdd}>添加小说</Button>
      </div>
      <Separator className="my-4" />
      <div className="grid grid-cols-2 gap-4">
        {data?.map((novel) => {
          const progress =
            safeDivide(novel.last_read_position, novel.total_characters) * 100;
          return (
            <Card key={novel.id}>
              <CardHeader>
                <CardTitle>{novel.title}</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="flex items-center gap-2">
                  <div className="text-sm shrink-0">阅读进度：</div>
                  <Progress value={progress} />
                  <small>{progress.toFixed(2)}%</small>
                </div>
              </CardContent>
              <CardFooter>
                <Button
                  variant="destructive"
                  onClick={() => handleDelete(novel.id)}
                >
                  删除
                </Button>
              </CardFooter>
            </Card>
          );
        })}
      </div>
    </section>
  );
};

export default NovelList;
