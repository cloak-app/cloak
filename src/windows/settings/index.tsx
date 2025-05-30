import { invoke } from '@tauri-apps/api/core';
import { useRequest } from 'ahooks';
import { Button, Flex, Form, Input, Radio, Switch, Typography } from 'lessline';
import { open } from '@tauri-apps/plugin-dialog';
import './styles.less';

interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
  total_characters: number;
}

export default function SettingsWindow() {
  const { data: novels, refresh } = useRequest(() =>
    invoke<Novel[]>('get_novel_list'),
  );

  const [form] = Form.useForm();

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
      <Typography.Title level={4}>设置</Typography.Title>
      <Form form={form} className="settings-window__form">
        <Form.Item
          label="总在最前"
          name="alwaysOnTop"
          extra="始终展示在所有窗口上方"
        >
          <Radio.Group
            options={[
              { label: '是', value: true },
              { label: '否', value: false },
            ]}
          />
        </Form.Item>

        <Form.Item label="添加小说">
          <Flex>
            <Form.Item name="path" noStyle>
              <Input placeholder="请输入小说路径" />
            </Form.Item>
            <Button style={{ flexShrink: 0 }} onClick={chooseFile}>
              选择小说
            </Button>
          </Flex>
        </Form.Item>
      </Form>
    </main>
  );
}
