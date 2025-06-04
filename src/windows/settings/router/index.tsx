import { createBrowserRouter } from 'react-router';
import Layout from '../pages/layout';
import NovelDetail from '../pages/novel-detail';
import NovelList from '../pages/novel-list';
import Config from '../pages/system-setting';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    handle: { crumb: () => '设置' },
    children: [
      {
        index: true,
        element: <NovelDetail />,
        handle: { crumb: () => '当前小说' },
      },
      {
        path: '/novel-list',
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
