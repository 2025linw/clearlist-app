import { ReactNode } from 'react';
import { Pressable, PressableProps, StyleSheet, Text, View } from 'react-native';

import useTheme from '@/hooks/use-theme';

export type Props = PressableProps & {
  children: string;
  leftIcon?: ReactNode;
};

export default function Button({ children, ...props }: Props) {
  const { currentColor } = useTheme();

  return (
    <Pressable {...props}>
      <View style={[styles.button, { backgroundColor: currentColor.secondary }]}>
        {props.leftIcon && props.leftIcon}

        <Text style={[styles.text, { color: currentColor.text }]}>{children}</Text>
      </View>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  button: {
    flexDirection: 'row',

    alignItems: 'center',
  },
  text: {
    margin: 8,

    fontSize: 18,
    textAlign: 'center',
  },
});
