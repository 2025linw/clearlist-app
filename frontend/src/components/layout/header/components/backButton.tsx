import { useNavigation } from '@react-navigation/native';
import { useCallback } from 'react';
import { GestureResponderEvent } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

import { atoms as a } from '#/alf';

import { Button, ButtonIcon } from '#/components/Button';
import { Calendar as X } from '#/components/icons/Calendar';

import { Slot } from './slot';

type BackButtonProps = { onPress: () => void };
export function BackButton({}: BackButtonProps) {
  const navigation = useNavigation<AllNavigationProp>();

  const goBack = useCallback((evt: GestureResponderEvent) => {
    if (navigation.canGoBack) {
      navigation.goBack();
    } else {
      navigation.navigate('Todo');
    }
  });

  return (
    <Slot>
      <Button
        size="small"
        color="secondary"
        shape="square"
        onPress={goBack}
        style={[a.bg_transparent]}
      >
        <ButtonIcon icon={X} />
      </Button>
    </Slot>
  );
}
