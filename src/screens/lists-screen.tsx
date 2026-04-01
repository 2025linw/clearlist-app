import { useRouter } from 'expo-router';
import { Button, View } from 'react-native';

export default function ListScreen() {
  const router = useRouter();

  return (
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
    </View>
  );
}
