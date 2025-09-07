import { StyleProp, ViewStyle } from 'react-native';
import Animated, { AnimatedScrollViewProps } from 'react-native-reanimated';

import { atoms as a } from '#/alf';

import { useLayoutContext } from '#/components/layout';

type Props = AnimatedScrollViewProps & {
  style?: StyleProp<ViewStyle>;
  contentContainerStyle?: StyleProp<ViewStyle>;
};
export function Content({
  children,
  style,
  contentContainerStyle,
  ...props
}: Props) {
  const { hasHeader } = useLayoutContext();

  console.log(hasHeader);

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
