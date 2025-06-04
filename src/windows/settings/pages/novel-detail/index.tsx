import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { Chapter, Novel } from '@/types';

const NovelDetail: React.FC = () => {
  const { data } = useRequest(() => invoke<Novel>('get_current_novel'));
  const { data: chapterList } = useRequest(() =>
    invoke<Chapter[]>('get_chapter_list'),
  );

  console.log(data);
  console.log(chapterList);
  return <section>当前阅读小说：{data?.title}</section>;
};

export default NovelDetail;
