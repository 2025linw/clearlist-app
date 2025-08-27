
import { ThemeName } from '#/types/theme';

import * as persisted from '#/state';

export function useColorTheme(): ThemeName {
  const theme = useThemeName();

  return theme;
}

export function useThemeName(): ThemeName {
  const colorSchme = useColorScheme();

  const colorMode = persisted.get('colorMode');
  const {colorMode, darkTheme} = useThemePrefs();


}
