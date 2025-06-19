import type { HTMLProps } from 'react';

export interface WindowControlsProps extends HTMLProps<HTMLDivElement> {
  platform?: 'windows' | 'macos' | 'linux';
  hide?: boolean;
  hideMethod?: 'display' | 'visibility';
  justify?: boolean;
}

export interface WindowTitleBarProps extends HTMLProps<HTMLDivElement> {
  controlsOrder?: 'right' | 'left' | 'platform' | 'system';
  windowControlsProps?: WindowControlsProps;
}
