import { ColorSchemeName, useColorScheme } from 'react-native';

import { DarkThemes, ThemeName } from '#/alf/types';

import * as persisted from '#/storage/async-storage';
import { device } from '#/storage';
import { SystemColorMode } from '#/storage/schemas';

export function useColorTheme(): ThemeName {
  const theme = useThemeName();

  return theme;
}

export function useThemeName(): ThemeName {
  const colorScheme = useColorScheme();
  const colorMode = device.get('colorMode')!;
  const darkTheme = device.get('darkTheme')!;

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
