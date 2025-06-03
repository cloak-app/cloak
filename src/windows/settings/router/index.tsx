import { createBrowserRouter } from 'react-router';
import NovelList from '../pages/novel-list';
import Layout from '../pages/layout';
import Config from '../pages/system-setting';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    handle: { crumb: () => '设置' },
    children: [
      {
        index: true,
        element: <NovelList />,
        handle: { crumb: () => '小说列表' },
      },
      {
        path: '/system-setting',
        element: <Config />,
        handle: { crumb: () => '系统设置' },
      },
      {
        path: '/reading-setting',
        element: <Config />,
        handle: { crumb: () => '阅读设置' },
      },
    ],
  },
]);
