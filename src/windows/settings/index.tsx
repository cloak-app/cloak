import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { Button } from 'lessline';
import { open } from '@tauri-apps/plugin-dialog';

interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
}

export default function SettingsWindow() {
  const { data: novels, refresh } = useRequest(() =>
    invoke<Novel[]>('get_novel_list'),
  );

  async function chooseFile() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'txt', extensions: ['txt'] }],
    });
    await invoke('add_novel', { path: file });
    refresh();
  }

  return (
    <main className="settings-window">
      <h1 className="text-xl font-bold mb-4">小说管理</h1>
      <Button onClick={chooseFile} className="mb-4">
        导入小说
      </Button>

      <div className="space-y-2">
        {novels?.map((novel) => (
          <div
            key={novel.id}
            className="p-2 border rounded hover:bg-gray-50 cursor-pointer"
            onClick={() => invoke('open_novel', { id: novel.id })}
          >
            <div className="font-medium">{novel.title}</div>
            <div className="text-sm text-gray-500">
              阅读进度: {novel.last_read_position}%
            </div>
          </div>
        ))}
      </div>
    </main>
  );
}
