import { type Palette, type Theme } from '#/types/theme';

const themes = createThemes();

export const defaultTheme = themes.light;

export function createThemes(): {
  lightPalette: Palette;
  darkPalette: Palette;
  dimPalette: Palette;
  light: Theme;
  dark: Theme;
  dim: Theme;
} {
  const color = {
    white: '#ffffff',
    black: '#111111',
    trueBlack: '#000000',
  } as const;

  const lightPalette: Palette = {
    white: color.white,
    black: color.black,

    primary_step1: '#fbfdff',
    primary_step2: '#f4fafe',
    primary_step3: '#e5f4ff',
    primary_step4: '#d5edff',
    primary_step5: '#c3e4fe',
    primary_step6: '#aed7f8',
    primary_step7: '#93c6ef',
    primary_step8: '#67b0e6',
    primary_step9: '#3093d4',
    primary_step10: '#1e86c7',
    primary_step11: '#0077b6',
    primary_step12: '#123853',

    gray_step1: '#fcfcfd',
    gray_step2: '#f9f9fb',
    gray_step3: '#eff0f3',
    gray_step4: '#e7e8ec',
    gray_step5: '#e0e1e6',
    gray_step6: '#d8d9e0',
    gray_step7: '#cdced7',
    gray_step8: '#b9bbc6',
    gray_step9: '#8b8d98',
    gray_step10: '#80828d',
    gray_step11: '#62636c',
    gray_step12: '#1e1f24',
  } as const;

  const darkPalette: Palette = {
    white: color.white,
    black: color.black,

    primary_step1: '#000000',
    primary_step2: '#0a131a',
    primary_step3: '#0c2638',
    primary_step4: '#05334f',
    primary_step5: '#0e4061',
    primary_step6: '#1c4f72',
    primary_step7: '#296189',
    primary_step8: '#3478a8',
    primary_step9: '#3093d4',
    primary_step10: '#1e86c7',
    primary_step11: '#75bff7',
    primary_step12: '#c5e6ff',

    gray_step1: '#000000',
    gray_step2: '#121315',
    gray_step3: '#1f1f22',
    gray_step4: '#27282c',
    gray_step5: '#2f3035',
    gray_step6: '#393a3f',
    gray_step7: '#46474e',
    gray_step8: '#5e606a',
    gray_step9: '#6c6e79',
    gray_step10: '#797b86',
    gray_step11: '#b2b3bd',
    gray_step12: '#eeeef0',
  };

  // const dimPalette: Palette = {
  //   white: color.white,
  //   black: color.black,

  //   primary_step1: '#0a1218',
  //   primary_step2: '#101a21',
  //   primary_step3: '#0f293c',
  //   primary_step4: '#073551',
  //   primary_step5: '#104262',
  //   primary_step6: '#1d5073',
  //   primary_step7: '#2a6289',
  //   primary_step8: '#3478a8',
  //   primary_step9: '#3093d4',
  //   primary_step10: '#1e86c7',
  //   primary_step11: '#75bff7',
  //   primary_step12: '#c5e6ff',

  //   gray_step1: '#111113',
  //   gray_step2: '#19191b',
  //   gray_step3: '#222325',
  //   gray_step4: '#292a2e',
  //   gray_step5: '#303136',
  //   gray_step6: '#393a40',
  //   gray_step7: '#46484f',
  //   gray_step8: '#5f606a',
  //   gray_step9: '#6c6e79',
  //   gray_step10: '#797b86',
  //   gray_step11: '#b2b3bd',
  //   gray_step12: '#eeeef0',
  // } as const;
  const dimPalette: Palette = {
    ...darkPalette,
  };

  const light: Theme = {
    scheme: 'light',
    name: 'light',
    palette: lightPalette,
    atoms: {
      text: {
        color: lightPalette.black,
      },
      text_low_contrast: {
        color: lightPalette.gray_step11,
      },
      text_high_contrast: {
        color: lightPalette.gray_step12,
      },
      bg: {
        backgroundColor: lightPalette.white,
      },
      border_contrast_low: {
        borderColor: lightPalette.gray_step6,
      },
      border_contrast_medium: {
        borderColor: lightPalette.gray_step7,
      },
      border_contrast_high: {
        borderColor: lightPalette.gray_step8,
      },
    },
  } as const;

  const dark: Theme = {
    scheme: 'dark',
    name: 'dark',
    palette: darkPalette,
    atoms: {
      text: {
        color: darkPalette.white,
      },
      text_low_contrast: {
        color: darkPalette.gray_step11,
      },
      text_high_contrast: {
        color: darkPalette.gray_step12,
      },
      bg: {
        backgroundColor: darkPalette.black,
      },
      border_contrast_low: {
        borderColor: darkPalette.gray_step6,
      },
      border_contrast_medium: {
        borderColor: darkPalette.gray_step7,
      },
      border_contrast_high: {
        borderColor: darkPalette.gray_step8,
      },
    },
  } as const;

  const dim: Theme = {
    scheme: 'dark',
    name: 'dim',
    palette: dimPalette,
    atoms: {
      text: {
        color: dimPalette.white,
      },
      text_low_contrast: {
        color: dimPalette.gray_step11,
      },
      text_high_contrast: {
        color: dimPalette.gray_step12,
      },
      bg: {
        backgroundColor: dimPalette.black,
      },
      border_contrast_low: {
        borderColor: dimPalette.gray_step6,
      },
      border_contrast_medium: {
        borderColor: dimPalette.gray_step7,
      },
      border_contrast_high: {
        borderColor: dimPalette.gray_step8,
      },
    },
  } as const;

  return {
    lightPalette,
    darkPalette,
    dimPalette,
    light,
    dark,
    dim,
  };
}
