import {
  createContext,
  type PropsWithChildren,
  useCallback,
  useContext,
  useMemo,
  useState,
} from 'react';

import { type Theme, type ColorMode, type ThemeName } from '#/alf/types';

import { createThemes, defaultTheme } from './themes';
import { type Device } from '#/storage/schemas';
import {
  computeFontScaleMultiplier,
  getFontFamily,
  getFontScale,
  setFontFamily as persistFontFamily,
  setFontScale as persistFontScale,
} from './fonts';

export { atoms } from './atoms';
export * from './fonts';
export * as tokens from './tokens';

export type Alf = {
  themeName: ThemeName;
  theme: Theme;
  themes: ReturnType<typeof createThemes>;
  fonts: {
    scale: Exclude<Device['fontScale'], undefined>;
    scaleMultiplier: number;
    family: Device['fontFamily'];
    setFontScale: (fontScale: Exclude<Device['fontScale'], undefined>) => void;
    setFontFamily: (fontFamily: Device['fontFamily']) => void;
  };
};

export const Context = createContext<Alf>({
  themeName: 'light',
  theme: defaultTheme,
  themes: createThemes(),
  fonts: {
    scale: getFontScale(),
    scaleMultiplier: computeFontScaleMultiplier(getFontScale()),
    family: getFontFamily(),
    setFontScale: () => {},
    setFontFamily: () => {},
  },
});

export function ThemeProvider({
  children,
  theme: themeName,
}: PropsWithChildren<{ theme: ThemeName }>) {
  const [fontScale, setFontScale] = useState<Alf['fonts']['scale']>(() =>
    getFontScale(),
  );

  const [fontScaleMultiplier, setFontScaleMultiplier] = useState(() =>
    computeFontScaleMultiplier(fontScale),
  );
  const setFontScaleAndPersist = useCallback<Alf['fonts']['setFontScale']>(
    (fontScale) => {
      setFontScale(fontScale);
      persistFontScale(fontScale);

      setFontScaleMultiplier(computeFontScaleMultiplier(fontScale));
    },
    [setFontScale],
  );

  const [fontFamily, setFontFamily] = useState<Alf['fonts']['family']>(() =>
    getFontFamily(),
  );
  const setFontFamilyAndPersist = useCallback<Alf['fonts']['setFontFamily']>(
    (fontFamily) => {
      setFontFamily(fontFamily);
      persistFontFamily(fontFamily);
    },
    [setFontFamily],
  );

  const themes = useMemo(() => {
    return createThemes();
  }, []);

  const value = useMemo<Alf>(
    () => ({
      themeName: themeName,
      theme: themes[themeName],
      themes,
      fonts: {
        scale: fontScale,
        scaleMultiplier: fontScaleMultiplier,
        family: fontFamily,
        setFontScale: setFontScaleAndPersist,
        setFontFamily: setFontFamilyAndPersist,
      },
    }),
    [
      themeName,
      themes,
      fontScale,
      fontScaleMultiplier,
      fontFamily,
      setFontScaleAndPersist,
      setFontFamilyAndPersist,
    ],
  );

  return <Context.Provider value={value}>{children}</Context.Provider>;
}

export function useAlf() {
  return useContext(Context);
}

export function useTheme(theme?: ThemeName) {
  const alf = useAlf();

  return useMemo(() => {
    return theme ? alf.themes[theme] : alf.theme;
  }, [theme, alf]);
}
