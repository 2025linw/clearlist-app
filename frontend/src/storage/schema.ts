import { type FontScale, type FontFamily } from '#/types/font';
import { type ColorMode, type ThemeName } from '#/types/theme';

export type Device = {
  colorMode: 'system' | ColorMode;
  darkTheme: Exclude<ThemeName, 'light'>;

  fontScale: FontScale;
  fontFamily: FontFamily;
};

export type Account = {
  userId: string;
  email: string;

  accessJwt: string;
  refreshJwt: string;
};
export type Session = { account: Account };
