import { StyleProp, Text, ViewStyle } from 'react-native';
import Animated, { AnimatedScrollViewProps } from 'react-native-reanimated';

type Props = AnimatedScrollViewProps & {
  style?: StyleProp<ViewStyle>;
  contentContainerStyle?: StyleProp<ViewStyle>;
};
export function Content({ children, style, contentContainerStyle, ...props }: Props) {
  return (
    <Animated.ScrollView
      style={style}
      contentContainerStyle={contentContainerStyle}
      scrollEnabled={true}
      {...props}
    >
      {children}
    </Animated.ScrollView>
  );
}
