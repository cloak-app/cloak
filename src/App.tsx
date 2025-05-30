import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useRequest } from 'ahooks';

interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
}

function App() {
  const { data, refresh } = useRequest(() => invoke<Novel[]>('get_novel_list'));

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
    <main className="container">
      <button onClick={chooseFile}>导入小说</button>
      {data?.map((novel) => (
        <div key={novel.id}>
          {novel.id} {novel.title}
        </div>
      ))}
    </main>
  );
}

export default App;
