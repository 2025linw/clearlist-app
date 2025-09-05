/*
 * Font
 */
export type FontScale = '-2' | '-1' | '0' | '1' | '2';

export type FontFamily = 'system' | 'theme';

/*
* Color Themes and Palettes
*/
export type ColorMode = 'light' | 'dark';
export type DarkThemes = 'dark';

export type ThemeName = ColorMode | DarkThemes;

// Follows Radix palettes
export type Palette = {
  white: string;
  black: string;

  primary_step1: string;
  primary_step2: string;
  primary_step3: string;
  primary_step4: string;
  primary_step5: string;
  primary_step6: string;
  primary_step7: string;
  primary_step8: string;
  primary_step9: string;
  primary_step10: string;
  primary_step11: string;
  primary_step12: string;

  gray_step1: string;
  gray_step2: string;
  gray_step3: string;
  gray_step4: string;
  gray_step5: string;
  gray_step6: string;
  gray_step7: string;
  gray_step8: string;
  gray_step9: string;
  gray_step10: string;
  gray_step11: string;
  gray_step12: string;
};

export type ThemedAtoms = {
  text: { color: string };
  text_low_contrast: { color: string };
  text_high_contrast: { color: string };

  bg: { backgroundColor: string };

  border_contrast_low: { borderColor: string };
  border_contrast_medium: { borderColor: string };
  border_contrast_high: { borderColor: string };
};

export type Theme = {
  scheme: ColorMode;
  name: ThemeName;
  palette: Palette;
  atoms: ThemedAtoms;
};
