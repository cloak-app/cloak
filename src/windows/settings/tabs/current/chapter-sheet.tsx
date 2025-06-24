import { invoke } from '@tauri-apps/api/core';
import { ChevronRight, LocateFixed } from 'lucide-react';
import { useCallback, useEffect, useRef, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { ScrollArea } from '@/components/ui/scroll-area';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet';
import { cn } from '@/lib/utils';
import { Chapter, Novel, Reader } from '@/types';

interface ChapterListProps {
  chapterList?: Chapter[];
  current_chapter?: Chapter;
  novel_id?: number;
  refresh: () => void;
}

const ChapterList = (props: ChapterListProps) => {
  const { chapterList, current_chapter, novel_id, refresh } = props;

  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const observer = useRef<IntersectionObserver | null>(null);
  const [isLocateButtonVisible, setIsLocateButtonVisible] = useState(false);

  useEffect(() => {
    if (!scrollAreaRef.current || !current_chapter) return;

    observer.current?.disconnect();

    observer.current = new IntersectionObserver(
      (entries) => setIsLocateButtonVisible(!entries[0].isIntersecting),
      { root: scrollAreaRef.current },
    );

    const currentChapterElement = document.getElementById(
      `${novel_id}-${current_chapter.index}`,
    );

    if (!currentChapterElement) return;

    observer.current.observe(currentChapterElement);

    return () => observer.current?.disconnect();
  }, [current_chapter, novel_id]);

  const scrollIntoView = useCallback(() => {
    if (!current_chapter) return;

    const currentChapterElement = document.getElementById(
      `${novel_id}-${current_chapter.index}`,
    );

    if (!currentChapterElement) return;

    currentChapterElement.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
    });
  }, [current_chapter, novel_id]);

  useEffect(() => {
    setTimeout(() => scrollIntoView());
  }, [scrollIntoView]);

  const handleClick = async (chapter: Chapter) => {
    if (current_chapter?.index === chapter.index) return;
    await invoke('set_read_position', { readPosition: chapter.start_line });
    refresh();
  };

  return (
    <>
      <ScrollArea ref={scrollAreaRef} className="flex-1 h-0 relative px-4">
        <div className="space-y-2">
          {chapterList?.map((chapter) => {
            const isRead =
              current_chapter && chapter.index <= current_chapter.index;
            return (
              <div
                key={chapter.index}
                id={`${novel_id}-${chapter.index}`}
                className={cn(
                  'flex items-center justify-between p-3 rounded-lg border cursor-pointer transition-colors hover:bg-accent',
                  chapter.index === current_chapter?.index && 'border-primary',
                )}
                onClick={() => handleClick(chapter)}
              >
                <div className="flex items-center gap-3 flex-1">
                  <div
                    className={cn(
                      'w-2 h-2 rounded-full',
                      isRead ? 'bg-green-500' : 'bg-gray-300',
                    )}
                  />
                  <span
                    className={cn(
                      'text-sm',
                      !isRead && 'text-muted-foreground',
                    )}
                  >
                    {chapter.title}
                  </span>
                </div>

                <div className="flex items-center gap-2">
                  <ChevronRight className="h-4 w-4 text-muted-foreground" />
                </div>
              </div>
            );
          })}
        </div>
        {isLocateButtonVisible && (
          <div className="absolute bottom-6 right-6">
            <Button variant="outline" size="icon" onClick={scrollIntoView}>
              <LocateFixed />
            </Button>
          </div>
        )}
      </ScrollArea>
    </>
  );
};

interface ChapterSheetProps {
  novel?: Novel;
  reader?: Reader;
  refresh: () => void;
}

const ChapterSheet: React.FC<ChapterSheetProps> = (props) => {
  const { novel, reader, refresh } = props;
  const { chapters, current_chapter, novel_id } = reader || {};
  const [chapterSearch, setChapterSearch] = useState('');

  const filteredChapters = chapters?.filter((chapter) =>
    chapter.title.toLowerCase().includes(chapterSearch.toLowerCase()),
  );

  const readChaptersCount = chapters?.filter(
    (c) => current_chapter && c.index <= current_chapter?.index,
  ).length;

  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button variant="outline">章节目录</Button>
      </SheetTrigger>
      <SheetContent className="h-full flex flex-col overflow-hidden">
        <SheetHeader>
          <SheetTitle>章节目录</SheetTitle>
          <SheetDescription>
            {novel?.title} - 共 {chapters?.length} 章
          </SheetDescription>
        </SheetHeader>

        <div className="px-4">
          <Input
            placeholder="搜索章节..."
            value={chapterSearch}
            onChange={(e) => setChapterSearch(e.target.value)}
          />
        </div>

        <ChapterList
          chapterList={filteredChapters}
          current_chapter={current_chapter}
          novel_id={novel_id}
          refresh={refresh}
        />

        <SheetFooter>
          <span className="text-sm text-muted-foreground">
            已读 {readChaptersCount} 章
          </span>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  );
};

export default ChapterSheet;
