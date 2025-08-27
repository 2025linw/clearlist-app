import { NavigationProp } from '@react-navigation/native';
import { StyleSheet, Text, View } from 'react-native';

import { TodoTabNavigatorParams } from '#/types/routes';

type Props = { navigation: NavigationProp<TodoTabNavigatorParams> };
export default function ProjectScreen({}: Props) {
  return (
    <View style={styles.container}>
      <Text>This is the project screen</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
