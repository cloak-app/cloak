import { platform } from '@tauri-apps/plugin-os';
import { WindowControls } from './controls';
import type { WindowTitleBarProps } from './types';
import { cn } from '@/lib/utils';

export function WindowTitleBar({
  children,
  controlsOrder = 'system',
  className,
  windowControlsProps,
  ...props
}: WindowTitleBarProps) {
  const left =
    controlsOrder === 'left' ||
    (controlsOrder === 'platform' &&
      windowControlsProps?.platform === 'macos') ||
    (controlsOrder === 'system' && platform() === 'macos');

  const customProps = (ml: string) => {
    if (windowControlsProps?.justify !== undefined) return windowControlsProps;

    const { className: windowControlsClassName, ...restProps } =
      windowControlsProps || {};
    return {
      ...restProps,
      justify: false,
      className: cn(windowControlsClassName, ml),
    };
  };

  return (
    <div
      className={cn(
        'bg-white h-8 flex select-none flex-row overflow-hidden',
        className,
      )}
      data-tauri-drag-region
      {...props}
    >
      {left ? (
        <>
          <WindowControls {...customProps('ml-0')} />
          {children}
        </>
      ) : (
        <>
          {children}
          <WindowControls {...customProps('ml-auto')} />
        </>
      )}
    </div>
  );
}
