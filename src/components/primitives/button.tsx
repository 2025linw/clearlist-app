import { ReactNode } from 'react';
import { ColorValue, Pressable, PressableProps, StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';
import { Palette } from '@/context/theme/types';

import Typography from '@/components/primitives/typography';

type ButtonShemes = 'default' | keyof Pick<Palette, 'primary' | 'danger'>;

export type Props = PressableProps & {
  text: string;
  scheme?: ButtonShemes;
  leftIcon?: ReactNode;
  disabled?: boolean;
};

function splitStyles({
  backgroundColor,
  borderColor,
  color,
}: {
  backgroundColor: ColorValue;
  borderColor: ColorValue;
  color: ColorValue;
}): [{ backgroundColor: ColorValue; borderColor: ColorValue }, { color: ColorValue }] {
  return [{ backgroundColor, borderColor }, { color }];
}

export default function Button({ text, scheme = 'default', ...props }: Props) {
  const { rounded, spacings, components } = useTheme();

  const paletteStyle = props.disabled ? components.Button.disabled : components.Button[scheme];

  const [buttonStyle, typographicStyle] = splitStyles(paletteStyle);

  return (
    <Pressable {...props}>
      <View
        style={[
          styles.container,
          { borderRadius: rounded.base, paddingVertical: spacings.lg, paddingHorizontal: spacings.xl },
          buttonStyle,
        ]}
      >
        {props.leftIcon && props.leftIcon}

        <Typography
          variant="button"
          style={typographicStyle}
        >
          {text}
        </Typography>
      </View>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',

    alignItems: 'center',
  },
});

export function Demo() {
  return (
    <View>
      <Button text="Default" />
      <Button text="Primary" />
    </View>
  );
}
