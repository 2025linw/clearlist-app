import { isAndroid } from '#/util/detectPlatform';

// TODO check if this is needed
export const TRACKING = isAndroid ? 0.1 : 0;

export const space = {
  '2xs': 2,
  xs: 4,
  sm: 8,
  md: 12,
  lg: 16,
  xl: 20,
  '2xl': 24,
  '3xl': 28,
  '4xl': 32,
  '5xl': 40,
} as const;

export const fontSize = {
  '2xs': 10,
  xs: 12,
  sm: 14,
  md: 16,
  lg: 18,
  xl: 20,
  '2xl': 22,
  '3xl': 26,
  '4xl': 32,
  '5xl': 40,
} as const;

export const lineHeight = {
  none: 1,
  normal: 1.5,
  relaxed: 1.625,
} as const;

export const borderRadius = {
  '2xs': 2,
  xs: 4,
  sm: 8,
  md: 12,
  lg: 16,
  full: 999,
} as const;

export const fontWeight = {
  normal: '400',
  bold: '600',
  heavy: '800',
} as const;
