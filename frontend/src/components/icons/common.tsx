import { type TextProps, StyleSheet } from 'react-native';
import { type PathProps, type SvgProps } from 'react-native-svg';

export const sizes = {
  xs: 12,
  sm: 16,
  md: 20,
  lg: 24,
  xl: 28,
  '2xl': 32,
} as const;

export type Props = {
  fill?: PathProps['fill'];
  style?: TextProps['style'];
  size?: keyof typeof sizes;
} & Omit<SvgProps, 'style' | 'size'>;

export function useCommonSVGProps(props: Props) {
  const { fill, size, ...rest } = props;
  const style = StyleSheet.flatten(rest.style);
  const _size = Number(size ? sizes[size] : rest.width || sizes.md);
  let _fill = fill || style?.color;

  return {
    fill: _fill,
    size: _size,
    style,
    ...rest,
  };
}
