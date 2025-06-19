import { platform as tauriPlatform } from '@tauri-apps/plugin-os';
import { useContext, useState, type HTMLProps } from 'react';
import { Button } from './button';
import TauriAppWindowContext, { TauriAppWindowProvider } from './context';
import { Icons } from './icons';
import { WindowControlsProps } from './types';
import { cn } from '@/lib/utils';

function MacOS({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { minimizeWindow, maximizeWindow, closeWindow } = useContext(
    TauriAppWindowContext,
  );
  const [isHovering, setIsHovering] = useState(false);

  const handleMouseEnter = () => {
    setIsHovering(true);
  };
  const handleMouseLeave = () => {
    setIsHovering(false);
  };

  return (
    <div
      className={cn(
        'space-x-2 px-3 text-black active:text-black dark:text-black',
        className,
      )}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      {...props}
    >
      <Button
        onClick={closeWindow}
        className="aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center rounded-full border border-black/[.12] bg-[#ff544d] text-center text-black/60 hover:bg-[#ff544d] active:bg-[#bf403a] active:text-black/60 dark:border-none"
      >
        {isHovering && <Icons.closeMac />}
      </Button>
      <Button
        onClick={minimizeWindow}
        className="aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center rounded-full border border-black/[.12]  bg-[#ffbd2e] text-center text-black/60 hover:bg-[#ffbd2e] active:bg-[#bf9122] active:text-black/60 dark:border-none"
      >
        {isHovering && <Icons.minMac />}
      </Button>
      <Button
        onClick={maximizeWindow}
        className="aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center rounded-full border border-black/[.12] bg-[#28c93f] text-center text-black/60 hover:bg-[#28c93f] active:bg-[#1e9930] active:text-black/60 dark:border-none"
      >
        {isHovering && <Icons.plusMac />}
      </Button>
    </div>
  );
}

function Windows({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } =
    useContext(TauriAppWindowContext);

  return (
    <div className={cn('h-8', className)} {...props}>
      <Button
        onClick={minimizeWindow}
        className="max-h-8 w-[46px] cursor-default rounded-none bg-transparent text-black/90 hover:bg-black/[.05] active:bg-black/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
      >
        <Icons.minimizeWin />
      </Button>
      <Button
        onClick={maximizeWindow}
        className={cn(
          'max-h-8 w-[46px] cursor-default rounded-none bg-transparent',
          'text-black/90 hover:bg-black/[.05] active:bg-black/[.03] dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]',
          // !isMaximizable && "text-white/[.36]",
        )}
      >
        {!isWindowMaximized ? (
          <Icons.maximizeWin />
        ) : (
          <Icons.maximizeRestoreWin />
        )}
      </Button>
      <Button
        onClick={closeWindow}
        className="max-h-8 w-[46px] cursor-default rounded-none bg-transparent text-black/90 hover:bg-[#c42b1c] hover:text-white active:bg-[#c42b1c]/90 dark:text-white"
      >
        <Icons.closeWin />
      </Button>
    </div>
  );
}

function Linux({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } =
    useContext(TauriAppWindowContext);

  return (
    <div
      className={cn('mr-[10px] h-auto items-center space-x-[13px]', className)}
      {...props}
    >
      <Button
        onClick={minimizeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.minimizeWin className="h-[9px] w-[9px]" />
      </Button>
      <Button
        onClick={maximizeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        {!isWindowMaximized ? (
          <Icons.maximizeWin className="h-2 w-2" />
        ) : (
          <Icons.maximizeRestoreWin className="h-[9px] w-[9px]" />
        )}
      </Button>
      <Button
        onClick={closeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-[#dadada] p-0 text-[#3d3d3d] hover:bg-[#d1d1d1] active:bg-[#bfbfbf] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.closeWin className="h-2 w-2" />
      </Button>
    </div>
  );
}

export function WindowControls({
  platform = tauriPlatform() as 'windows' | 'macos' | 'linux',
  justify = false,
  hide = false,
  hideMethod = 'display',
  // linuxDesktop = "gnome",
  className,
  ...props
}: WindowControlsProps) {
  const customClass = cn(
    'flex',
    className,
    hide && (hideMethod === 'display' ? 'hidden' : 'invisible'),
  );

  const ControlsComponent = () => {
    switch (platform) {
      case 'windows':
        return (
          <Windows
            className={cn(customClass, justify && 'ml-auto')}
            {...props}
          />
        );
      case 'macos':
        return (
          <MacOS className={cn(customClass, justify && 'ml-0')} {...props} />
        );
      case 'linux':
        return (
          <Linux className={cn(customClass, justify && 'ml-auto')} {...props} />
        );
      default:
        return (
          <Windows
            className={cn(customClass, justify && 'ml-auto')}
            {...props}
          />
        );
    }
  };

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  );
}
