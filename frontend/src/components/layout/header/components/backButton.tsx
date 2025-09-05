import { useNavigation } from '@react-navigation/native';
import { useCallback } from 'react';
import { GestureResponderEvent } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

import { atoms as a } from '#/alf';

import { Button, ButtonIcon, ButtonProps } from '#/components/Button';
import { Calendar as X } from '#/components/icons/Calendar';
import { SPACE_FROM_EDGE } from '#/components/layout/header/const';

import { Slot } from './slot';

// TODO: document that the onPress is only used for any functions to run prior to returning
// This is due to the fact that onPress automatically calls navigation.goBack()
// Consider if this functionality should change
type BackButtonProps = Pick<ButtonProps, 'onPress' | 'style'> & {};
export function BackButton({ onPress }: BackButtonProps) {
  const navigation = useNavigation<AllNavigationProp>();

  const goBack = useCallback((evt: GestureResponderEvent) => {
    onPress?.(evt);

    if (navigation.canGoBack()) {
      navigation.goBack();
    } else {
      navigation.navigate('Todo');
    }
  }, []);

  return (
    <Slot>
      <Button
        label="returnButton"
        size="small"
        color="secondary"
        shape="square"
        onPress={goBack}
        style={[{ marginLeft: SPACE_FROM_EDGE }, a.bg_transparent]}
      >
        <ButtonIcon icon={X} />
      </Button>
    </Slot>
  );
}
