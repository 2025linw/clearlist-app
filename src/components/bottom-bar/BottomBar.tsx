import { BottomTabBarProps } from '@react-navigation/bottom-tabs';
import { StyleSheet } from 'react-native';
import Animated from 'react-native-reanimated';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { atoms as a, useTheme } from '#/alf';

import {
  Button,
  ButtonIcon,
  ButtonProps,
  ButtonText,
} from '#/components/Button';
import { Calendar as X } from '#/components/icons/Calendar';

export function BottomBar({ navigation }: BottomTabBarProps) {
  const inset = useSafeAreaInsets();
  const t = useTheme();

  return (
    <Animated.View
      style={[styles.bottomBar, t.atoms.bg, { paddingBottom: inset.bottom }]}
    >
      <Btn page="Todo" onPress={() => navigation.navigate('TodoTab')} />
      <Btn page="Calendar" onPress={() => navigation.navigate('CalendarTab')} />
      <Btn page="Search" onPress={() => navigation.navigate('SearchTab')} />
    </Animated.View>
  );
}

type BtnProps = Pick<ButtonProps, 'onPress'> & { page: string };
function Btn({ page, onPress }: BtnProps) {
  return (
    <Button
      label={page + 'Nav'}
      size="large"
      color="transparent"
      shape="square"
      onPress={onPress}
      style={[a.ml_auto, a.mr_auto, a.flex_1, a.flex_col]}
    >
      <ButtonIcon icon={X} />
      <ButtonText style={a.text_2xs}>{page}</ButtonText>
    </Button>
  );
}

const styles = StyleSheet.create({
  bottomBar: {
    ...a.absolute,
    ...a.bottom_0,
    ...a.left_0,
    ...a.right_0,
    ...a.flex_row,
    ...a.border_t,
    ...a.px_xl,
  },
});
