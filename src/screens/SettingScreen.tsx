import { useCallback } from 'react';

import { AllNavigationProp } from '#/types/routes';

import { useSessionApi } from '#/state/session';

import { Button, ButtonText } from '#/components/Button';
import Layout from '#/components/layout';

type Props = { navigation: AllNavigationProp };
export default function SettingScreen({ navigation }: Props) {
  const { logout } = useSessionApi();

  const onPressInner = useCallback(() => {
    navigation.navigate('Debug');
  }, []);

  return (
    <Layout>
      <Layout.Content>
        <Button label="debug-screen" onPress={onPressInner}>
          <ButtonText>Debug</ButtonText>
        </Button>
        <Button label="logout" onPress={() => logout()}>
          <ButtonText>Logout</ButtonText>
        </Button>
      </Layout.Content>
    </Layout>
  );
}
