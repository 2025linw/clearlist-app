import { ReactNode, createContext, useContext } from 'react';
import { useColorScheme } from 'react-native';

import usePersisted from '@/hooks/use-persisted';

import { ColorTheme, Theme, ThemeMode, buildTheme } from './types';

type Context = {
  loaded: boolean;

  theme: Theme;

  themeMode: 'system' | ThemeMode;
  setThemeMode: (_: 'system' | ThemeMode) => void;
  resetThemeMode: () => void;

  colorTheme: ColorTheme;
  setColorTheme: (_: ColorTheme) => void;
  resetColorTheme: () => void;
};
const ThemeContext = createContext<Context>({} as unknown as Context); // TODO: fix this jank?

type Props = {
  children: ReactNode;
  onThemeVariantChange?: (_v: ColorTheme) => void; // TODO: is this needed?
};

export function ThemeProvider({ children, ...props }: Props) {
  const { value: themeMode, setValue: _setThemeMode, loaded: themeLoaded } = usePersisted('systemTheme');
  const { value: colorTheme, setValue: _setColorTheme, loaded: colorLoaded } = usePersisted('colorTheme');

  const systemTheme = useColorScheme();

  const darkMode = themeMode === 'system' ? (systemTheme === 'dark' ? 'dark' : 'light') : themeMode;

  const theme = buildTheme(colorTheme, darkMode);

  function setThemeMode(v: 'system' | ThemeMode) {
    _setThemeMode(v);
  }
  function resetThemeMode() {
    _setThemeMode('system');
  }

  function setColorTheme(v: ColorTheme) {
    _setColorTheme(v);
    props.onThemeVariantChange?.(v);
  }
  function resetColorTheme() {
    _setColorTheme('default');
    props.onThemeVariantChange?.('default');
  }

  const loaded = themeLoaded && colorLoaded;

  return (
    <ThemeContext
      value={{
        loaded,
        theme,
        themeMode,
        setThemeMode,
        resetThemeMode,
        colorTheme,
        setColorTheme,
        resetColorTheme,
      }}
    >
      {children}
    </ThemeContext>
  );
}

export function useThemeContext() {
  const ctx = useContext(ThemeContext);

  // TODO: add this to all contexts
  // if (!ctx) {
  //   throw new Error('useTheme must be used inside ThemeProvider');
  // }

  return ctx;
}

export function useTheme() {
  const { theme } = useThemeContext();

  return theme;
}

export function useThemeMode() {
  const { themeMode, setThemeMode } = useThemeContext();

  return [themeMode, setThemeMode] as const;
}

export function useResetThemeMode() {
  const { resetThemeMode } = useThemeContext();

  return resetThemeMode;
}

export function useColorTheme() {
  const { colorTheme, setColorTheme } = useThemeContext();

  return [colorTheme, setColorTheme] as const;
}

export function useResetColorTheme() {
  const { resetColorTheme } = useThemeContext();

  return resetColorTheme;
}
