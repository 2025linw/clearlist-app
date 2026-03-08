import { useRouter } from 'expo-router';
import { Button, View } from 'react-native';

import LogInScreen from '#/log-in-screen';

import { useSession, useSessionApi } from '@/context/AuthContext';

export default function Index() {
  const router = useRouter();

  const { hasSession } = useSession();
  const { logout } = useSessionApi();

  return hasSession ? (
    <View>
      <Button
        title="Inbox"
        onPress={() => router.navigate('/inbox')}
      />
      <Button
        title="Today"
        onPress={() => router.navigate('/today')}
      />
      <Button
        title="Settings"
        onPress={() => router.navigate('/settings')}
      />
      <Button
        title="Logout"
        onPress={() => logout()}
      />
    </View>
  ) : (
    <LogInScreen />
  );
}
