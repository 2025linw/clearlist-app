import Animated, {
  AnimatedScrollViewProps,
  FlatListPropsWithLayout,
} from 'react-native-reanimated';

import { atoms as a } from '#/alf';

import { useLayoutContext } from './context';

type ContentProps = AnimatedScrollViewProps & {};
export function Content({
  children,
  style,
  contentContainerStyle,
  ...rest
}: ContentProps) {
  const { hasHeader } = useLayoutContext();

  return (
    <Animated.ScrollView
      style={[hasHeader ? a.h_full : {}, style]}
      contentContainerStyle={contentContainerStyle}
      scrollEnabled={true}
      {...rest}
    >
      {children}
    </Animated.ScrollView>
  );
}

type DataContentProps<T> = FlatListPropsWithLayout<T> & {};
export function DataContent<T>({
  data,
  renderItem,
  style,
  contentContainerStyle,
  ...rest
}: DataContentProps<T>) {
  return (
    <Animated.FlatList
      data={data}
      renderItem={renderItem}
      style={style}
      contentContainerStyle={contentContainerStyle}
      {...rest}
    ></Animated.FlatList>
  );
}
