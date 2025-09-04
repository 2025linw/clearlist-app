import { ColorSchemeName, useColorScheme } from 'react-native';

import { SystemColorMode } from '#/types/persisted';
import { DarkThemes, ThemeName } from '#/types/theme';

import * as persisted from '#/storage/async-storage';

export function useColorTheme(): ThemeName {
  const theme = useThemeName();

  return theme;
}

export function useThemeName(): ThemeName {
  const colorScheme = useColorScheme();
  const colorMode = persisted.get('colorMode');
  const darkTheme = persisted.get('darkTheme');

  return getThemeName(colorScheme, colorMode, darkTheme);
}

function getThemeName(
  colorScheme: ColorSchemeName,
  colorMode: SystemColorMode,
  darkTheme: DarkThemes,
): ThemeName {
  if (
    (colorMode === 'system' && colorScheme === 'light') ||
    colorMode === 'light'
  ) {
    return 'light';
  } else {
    return darkTheme;
  }
}
