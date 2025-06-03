import { RouterProvider } from 'react-router';
import { router } from './router';

export default function SettingsWindow() {
  return <RouterProvider router={router} />;
}
