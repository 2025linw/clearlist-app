import { ReactNode } from 'react';
import { StyleSheet, TextStyle, View, ViewStyle } from 'react-native';

import Typography from '@/components/primitives/typography';

export type Props = {
  label?: string;
  labelStyle?: TextStyle;
  style?: ViewStyle;
  children: ReactNode;
};

export default function FormField({ children, ...props }: Props) {
  return (
    <View style={[styles.field, props.style]}>
      {props.label && (
        <Typography
          variant="h2"
          style={styles.label}
        >
          {props.label}
        </Typography>
      )}

      <View style={styles.inputContainer}>{children}</View>
    </View>
  );
}

const styles = StyleSheet.create({
  field: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  label: {
    width: 90,
    fontSize: 16, // TODO: change this into a global reusable font size
  },
  inputContainer: {
    flex: 1,
  },
});
