import { StyleProp, Text, ViewStyle } from 'react-native';
import Animated, { AnimatedScrollViewProps } from 'react-native-reanimated';

import { atoms as a } from '#/alf';

type Props = AnimatedScrollViewProps & {
  style?: StyleProp<ViewStyle>;
  contentContainerStyle?: StyleProp<ViewStyle>;
  hasHeader?: boolean;
};
export function Content({
  children,
  style,
  contentContainerStyle,
  hasHeader,
  ...props
}: Props) {
  return (
    <Animated.ScrollView
      style={[hasHeader ? a.h_full : {}, style]}
      contentContainerStyle={contentContainerStyle}
      scrollEnabled={true}
      {...props}
    >
      {children}
    </Animated.ScrollView>
  );
}
