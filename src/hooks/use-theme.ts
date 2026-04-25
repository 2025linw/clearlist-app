import { ColorSchemeName, ColorValue, useColorScheme } from 'react-native';

type ColorTheme = {
  primary: ColorValue;
  secondary: ColorValue;

  text: ColorValue;
};

type ColorMode = {
  light: ColorTheme;
  dark: ColorTheme;
};

type Theme = {
  theme: ColorSchemeName;
  colors: ColorMode;
  currentColor: ColorTheme;
  fonts: string;
};

export default function useTheme(): Theme {
  const theme = useColorScheme();

  const colors: ColorMode = {
    light: {
      primary: '#2B396D',
      secondary: '#E4E4E4',

      text: '#0B0B0B',
    },
    dark: {
      primary: '#2B396D',
      secondary: '#0B0B0B',

      text: '#E4E4E4',
    },
  };

  const currentColor = theme === 'dark' ? colors.dark : colors.light;

  const fonts = 'temp';

  return { theme, colors, currentColor, fonts };
}
