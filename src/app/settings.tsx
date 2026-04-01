import { useSessionApi } from '@/context/auth-context';

import { Button, Text, View } from 'react-native';

export default function Index() {
  const { logout } = useSessionApi();

  return (
    <View
      style={{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      <Text>This is the Settings!</Text>

      <Button
        title="Logout"
        onPress={() => logout()}
      />
    </View>
  );
}
