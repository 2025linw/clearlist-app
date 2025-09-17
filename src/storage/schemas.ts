import {
  type FontScale,
  type FontFamily,
  type DarkThemes,
  type ColorMode,
} from '#/alf/types';

export type AccountSchema = {
  userId: string;

  email: string;

  accessJwt?: string;
  refreshJwt?: string;
};

export type PersistedSchema = { account: AccountSchema | undefined };

export type SystemColorMode = 'system' | ColorMode;

export type Device = {
  colorMode: SystemColorMode;
  darkTheme: DarkThemes;

  fontScale: FontScale;
  fontFamily: FontFamily;
};
