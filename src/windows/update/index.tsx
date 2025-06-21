import { listen } from '@tauri-apps/api/event';
import { WandSparkles } from 'lucide-react';
import { useEffect, useState } from 'react';

const UpdateWindow: React.FC = () => {
  const [finished, setFinished] = useState(false);
  const [progress, setProgress] = useState<number>(0);

  useEffect(() => {
    const finishedListener = listen('update-finished', () => {
      setFinished(true);
    });

    const progressListener = listen<number>(
      'update-progress-change',
      (progress) => {
        setProgress(progress.payload);
      },
    );

    return () => {
      progressListener.then((unListen) => unListen());
      finishedListener.then((unListen) => unListen());
    };
  }, []);

  const finalProgress = finished ? 100 : Math.min(progress, 95);

  return (
    <>
      <div className="h-svh">
        <div className="m-auto flex h-full w-full flex-col items-center justify-center gap-2">
          <WandSparkles size={72} />
          <h1 className="text-4xl leading-tight font-bold">
            {finished ? 'Done ✨' : 'Updating...✨'}
          </h1>
          <p className="text-muted-foreground text-center">
            {finished
              ? 'Cloak 已完成更新，重启应用即可开始摸鱼～'
              : 'Cloak 正在自动更新中，稍等片刻即可继续摸鱼～'}
          </p>
        </div>
      </div>
      <div
        className="loading h-1 bg-foreground transition-all duration-200 absolute z-40 bottom-0"
        style={{ width: `${finalProgress}%` }}
      />
    </>
  );
};

export default UpdateWindow;
