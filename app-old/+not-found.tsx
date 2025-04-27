import { Stack } from 'expo-router';
import { View, Text } from 'react-native';

export default function NotFoundScreen() {
  return (
    <>
      <Stack.Screen options={{ title: 'Oops! Not Found' }} />
        <View>
          <Text>
            Not found...
          </Text>
        </View>
    </>
  )
}
