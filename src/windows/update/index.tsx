import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

const UpdateWindow: React.FC = () => {
  const [finished, setFinished] = useState(false);
  const [progress, setProgress] = useState<number>(0);

  useEffect(() => {
    const finishedListener = listen('update-finished', () => {
      setFinished(true);
    });

    const progressListener = listen<number>('update-progress', (progress) => {
      setProgress(progress.payload);
    });

    return () => {
      progressListener.then((unListen) => unListen());
      finishedListener.then((unListen) => unListen());
    };
  }, []);

  return <section></section>;
};

export default UpdateWindow;
