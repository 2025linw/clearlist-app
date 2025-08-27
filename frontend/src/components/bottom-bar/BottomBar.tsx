import { BottomTabBarProps } from '@react-navigation/bottom-tabs';
import { useCallback } from 'react';
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

  const onPressTodo = useCallback(() => {
    navigation.navigate('TodoTab');
  });
  const onPressCalendar = useCallback(() => {
    navigation.navigate('CalendarTab');
  });
  const onPressSearch = useCallback(() => {
    navigation.navigate('SearchTab');
  });

  return (
    <Animated.View
      style={[styles.bottomBar, t.atoms.bg, { paddingBottom: inset.bottom }]}
    >
      <Btn page="Todo" onPress={onPressTodo} />
      <Btn page="Calendar" onPress={onPressCalendar} />
      <Btn page="Search" onPress={onPressSearch} />
    </Animated.View>
  );
}

type BtnProps = ButtonProps & { page: string };
function Btn({ page, onPress }: BtnProps) {
  return (
    <Button
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
  bottomBar: [
    a.absolute,
    a.bottom_0,
    a.left_0,
    a.right_0,
    a.flex_row,
    a.border_t,
    a.px_xl,
  ],
});
