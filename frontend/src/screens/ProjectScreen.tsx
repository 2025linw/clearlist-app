import { NavigationProp } from '@react-navigation/native';
import { StyleSheet, Text, View } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

type Props = { navigation: NavigationProp<TodoListNavigatorParams> };
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
