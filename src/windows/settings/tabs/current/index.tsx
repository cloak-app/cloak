import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Package } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';
import ChapterDialog from './chapter-dialog';
import { Badge } from '@/components/ui/badge';
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
import { Progress } from '@/components/ui/progress';
import { Novel, Reader } from '@/types';

const Current: React.FC = () => {
  const [novel, setNovel] = useState<Novel>();
  const [reader, setReader] = useState<Reader>();

  const { current_chapter, chapters, read_progress } = reader || {};

  const fetchData = useCallback(async () => {
    const reader = await invoke<Reader>('get_novel_reader');
    setReader(reader);

    if (reader.novel_id) {
      const novel = await invoke<Novel>('get_novel_detail', {
        id: reader.novel_id,
      });
      setNovel(novel);
    }
  }, []);

  useEffect(() => {
    const listener = listen('reader-change', () => {
      fetchData();
    });
    return () => {
      listener.then((unListen) => unListen());
    };
  }, [fetchData]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const handleContinue = () => {
    invoke('open_reader_window');
  };

  if (!reader || !novel) return null;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Package size={16} />
          <span>{novel.title}</span>
          <Badge>当前阅读</Badge>
        </CardTitle>
        <CardDescription>继续你的阅读之旅</CardDescription>
        <CardAction></CardAction>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span>阅读进度</span>
              <span className="font-medium">{read_progress?.toFixed(2)}%</span>
            </div>
            <Progress value={read_progress} className="w-full h-2" />
            <div className="flex justify-between text-sm text-muted-foreground">
              <span>{current_chapter?.title}</span>
              <span>共{chapters?.length}章</span>
            </div>
          </div>
        </div>
      </CardContent>
      <CardFooter className="gap-3">
        <Button className="flex-1" onClick={handleContinue}>
          继续阅读
        </Button>
        <ChapterDialog novel={novel} reader={reader} refresh={fetchData} />
      </CardFooter>
    </Card>
  );
};

export default Current;
