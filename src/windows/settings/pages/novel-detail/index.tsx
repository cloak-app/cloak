import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { ChevronRight, LocateFixed } from 'lucide-react';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Link } from 'react-router';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { ScrollArea } from '@/components/ui/scroll-area';
import { safeDivide } from '@/lib/number';
import { Chapter, Reader } from '@/types';

const NovelDetail: React.FC = () => {
  const { data, refresh } = useRequest(() =>
    invoke<Reader>('get_novel_reader'),
  );

  const { novel, chapters, line_num } = data ?? {};

  const scrollAreaRef = useRef<HTMLDivElement>(null);

  const currentChapter = useMemo(() => {
    if (!chapters || !line_num) return;

    const currentChapterIndex =
      chapters.findIndex((chapter) => chapter.start_line > line_num) - 1;

    const index = Math.max(currentChapterIndex, 0);

    return chapters[index];
  }, [chapters, line_num]);

  const isCurrentChapter = (chapter: Chapter) => {
    return chapter.start_line === currentChapter?.start_line;
  };

  const handleClick = async (chapter: Chapter) => {
    if (isCurrentChapter(chapter)) return;

    await invoke('set_line_num', { lineNum: chapter.start_line });
    refresh();
  };

  const observer = useRef<IntersectionObserver | null>(null);
  const [isLocateButtonVisible, setIsLocateButtonVisible] = useState(false);

  useEffect(() => {
    if (!scrollAreaRef.current || !currentChapter) return;

    observer.current = new IntersectionObserver(
      (entries) => setIsLocateButtonVisible(!entries[0].isIntersecting),
      { root: scrollAreaRef.current },
    );

    const currentChapterElement = document.getElementById(
      `${novel?.id}-${currentChapter.start_line}`,
    );

    if (!currentChapterElement) return;

    observer.current.observe(currentChapterElement);
  }, [currentChapter, novel]);

  const scrollIntoView = useCallback(() => {
    if (!currentChapter) return;

    const currentChapterElement = document.getElementById(
      `${novel?.id}-${currentChapter.start_line}`,
    );

    if (!currentChapterElement) return;

    currentChapterElement.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
    });
  }, [currentChapter, novel]);

  useEffect(() => {
    scrollIntoView();
  }, [scrollIntoView]);

  const progress = safeDivide(line_num, novel?.total_lines) * 100;

  if (!data)
    return (
      <div className="w-full h-full text-sm">
        <div className="mt-20 text-muted-foreground text-center leading-7 ">
          暂无阅读中的小说，前往
          <Link
            className="text-primary underline underline-offset-4 mx-1"
            to="/novel-list"
          >
            小说列表
          </Link>
          打开一本新的小说
        </div>
      </div>
    );

  return (
    <div className="w-full flex flex-col gap-4 p-4 h-full">
      <div className="border-b pb-2 text-lg font-semibold">{novel?.title}</div>
      <div className="text-sm leading-none font-medium">阅读进度</div>
      <div className="flex items-center gap-2">
        <Progress className="w-3/5" value={progress} />
        <span className="text-sm text-muted-foreground">
          {progress.toFixed(2)}%
        </span>
      </div>
      <div className="text-sm leading-none font-medium">章节列表</div>
      <ScrollArea className="max-h-80 rounded-md border" ref={scrollAreaRef}>
        <div className="flex flex-col p-2 gap-2">
          {chapters?.map((chapter) => (
            <div
              key={`${novel?.id}-${chapter.start_line}`}
              id={`${novel?.id}-${chapter.start_line}`}
              className="rounded-md border p-2 flex justify-between items-center hover:bg-muted-foreground/10 cursor-pointer"
              onClick={() => handleClick(chapter)}
            >
              <div>
                <div className="text-sm font-medium flex items-center">
                  <span>{chapter.title}</span>
                  {isCurrentChapter(chapter) && (
                    <Badge className="scale-75">当前章节</Badge>
                  )}
                </div>
                <div className="text-xs text-muted-foreground">
                  {chapter.start_line}
                </div>
              </div>
              <ChevronRight className="w-4 h-4" />
            </div>
          ))}
        </div>
        {isLocateButtonVisible && (
          <div className="absolute bottom-4 right-4">
            <Button variant="outline" size="icon" onClick={scrollIntoView}>
              <LocateFixed />
            </Button>
          </div>
        )}
      </ScrollArea>
    </div>
  );
};

export default NovelDetail;
