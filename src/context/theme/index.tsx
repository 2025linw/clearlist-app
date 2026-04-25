import { PropsWithChildren, createContext, useCallback, useContext, useState } from 'react';

import { Theme, ThemeContextType, ThemeVariant } from './types';
import usePersisted from '@/hooks/use-persisted';

type Context = {
  theme: Theme;
  themeVariant: ThemeVariant;
  setThemeVariant: (_: ThemeVariant) => void;
  resetThemeVariant: () => void;
};
const ThemeContext = createContext<Context>({});

const light: Theme = {
  theme: 'light',
  colors: {
    primary: '#2B396D',
    secondary: '#E4E4E4',

    text: '#0B0B0B',
  },
};

export function ThemeProvider({ children }: PropsWithChildren) {
  const [theme, setTheme] = useState<ThemeVariant>('default');
  const [persistedTheme, setPersistedTheme] = usePersisted('theme');

  // TODO: get persisted

  const setThemeCb = useCallback<ThemeContextType['setTheme']>(async (theme) => {});

  return <ThemeContext value={{ theme, setThemeCb }}>{children}</ThemeContext>;
}

export function useTheme() {
  return useContext(ThemeContext);
}
