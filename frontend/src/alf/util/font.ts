import { TextStyle } from 'react-native';

import { atoms } from '../atoms';

export function normalizeLineHeight({ fontSize, lineHeight }: TextStyle) {
  const size = fontSize || atoms.text_md.fontSize;
  const height = lineHeight || atoms.leading_normal.lineHeight;

  return Math.round(size * height);
}
