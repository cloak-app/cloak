import { Badge } from '@/components/ui/badge';

export enum NovelStatus {
  READING = 'READING',
  READ = 'READ',
  UN_READ = 'UN_READ',
}

export const getNovelStatus = (progress: number) => {
  if (progress === 0) return NovelStatus.UN_READ;
  if (progress === 100) return NovelStatus.READ;
  return NovelStatus.READING;
};

export const STATUS_BADGE_MAP: Record<NovelStatus, React.ReactNode> = {
  [NovelStatus.UN_READ]: <Badge variant="outline">未读</Badge>,
  [NovelStatus.READ]: <Badge variant="secondary">已完成</Badge>,
  [NovelStatus.READING]: <Badge variant="default">在读</Badge>,
};
