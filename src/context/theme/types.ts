import { ColorSchemeName, ColorValue } from 'react-native';

type ColorTheme = 'default' | 'pink';

type Palette = {
  primary: ColorValue;
  navigation: ColorValue;

  text: ColorValue;

  background: ColorValue;
};

const variants: Record<ColorTheme, Pick<Palette, 'primary' | 'navigation'>> = {
  default: {
    primary: '#2B396D',
    navigation: '#2B396D',
  },
  pink: {
    primary: '#ffd1dc',
    navigation: '#ffd1dc',
  },
};

const palette: Record<'light' | 'dark', Omit<Palette, 'primary' | 'navigation'>> = {
  light: {
    text: '#0B0B0B',
    background: '#E4E4E4',
  },
  dark: {
    text: '#E4E4E4',
    background: '#0B0B0B',
  },
};

const typographyVariants = {
  h1: {
    fontFamily: "Inter"
  }
}

const stitch = {
  navigation: '#0084FF',
  primary: '#0084FF',
};
const flamingo = {
  navigation: '#FF8DA1',
  primary: '#FF8DA1',
};
const grinch = {
  navigation: '#009688',
  primary: '#009688',
};
const lorax = {
  navigation: '#FFA500',
  primary: '#FFA500',
};

const dark = {
  background: '#000000',
  border: '#2C2C2E',
  danger: '#EE3333',
  subtle: '#9A9A9E',
  surface: '#1C1C1E',
  text: '#E5E5E7',
};
const light = {
  background: '#F2F2F2',
  border: '#E2E2E2',
  danger: '#EE3333',
  subtle: '#8E8E93',
  surface: '#FFFFFF',
  text: '#1C1C1E',
};

export type Theme = {
  theme: ColorSchemeName;
  colors: Palette;
  fonts: Typography;
};

export type ThemeVariant = 'default';

export type ThemeContextType = {
  theme: Theme;
  setTheme: () => void;
};

const defaultTheme = {};
