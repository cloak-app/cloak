import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';
import { Config, CustomEvent } from '@/types';

export type Theme = 'dark' | 'light' | 'system';

type ThemeProviderProps = {
  children: React.ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
};

type ThemeProviderState = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
};

const initialState: ThemeProviderState = {
  theme: 'system',
  setTheme: () => null,
};

const ThemeProviderContext = createContext<ThemeProviderState>(initialState);

export function ThemeProvider({
  children,
  defaultTheme = 'system',
  storageKey = 'vite-ui-theme',
  ...props
}: ThemeProviderProps) {
  const [theme, setTheme] = useState<Theme>(
    () => (localStorage.getItem(storageKey) as Theme) || defaultTheme,
  );

  const handleThemeChange = useCallback((theme: Theme) => {
    const root = window.document.documentElement;
    const query = window.matchMedia('(prefers-color-scheme: dark)');
    query.onchange = null;

    root.classList.remove('light', 'dark');

    if (theme === 'system') {
      const systemTheme = query.matches ? 'dark' : 'light';
      root.classList.add(systemTheme);
      query.onchange = () => handleThemeChange(theme);
      return;
    }

    root.classList.add(theme);
  }, []);

  useEffect(() => {
    handleThemeChange(theme);
  }, [theme, handleThemeChange]);

  useEffect(() => {
    const listener = listen(CustomEvent.ConfigChange, () => {
      invoke<Config>('get_config').then((config) =>
        setTheme(config.theme as Theme),
      );
    });
    return () => {
      listener.then((unListen) => unListen());
    };
  }, []);

  const value = {
    theme,
    setTheme: (theme: Theme) => {
      localStorage.setItem(storageKey, theme);
      setTheme(theme);
    },
  };

  return (
    <ThemeProviderContext.Provider {...props} value={value}>
      {children}
    </ThemeProviderContext.Provider>
  );
}

export const useTheme = () => {
  const context = useContext(ThemeProviderContext);

  if (context === undefined)
    throw new Error('useTheme must be used within a ThemeProvider');

  return context;
};
