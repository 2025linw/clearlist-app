import { StyleSheet, View } from 'react-native';

import { useSessionApi } from '#/state/session';

import { Button, ButtonText } from '#/components/Button';
import { AllNavigationProp } from '#/types/routes';
import { useCallback } from 'react';

type Props = {
  navigation: AllNavigationProp
};
export default function SettingScreen({navigation}: Props) {
  const { logout } = useSessionApi();

  const onPressInner = useCallback(() => {
    navigation.navigate('Debug');
  })

  return (
    <View style={styles.container}>
      <Button label="debug-screen" onPress={onPressInner}>
        <ButtonText>Debug</ButtonText>
      </Button>
      <Button label="logout" onPress={() => logout()}>
        <ButtonText>Logout</ButtonText>
      </Button>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
