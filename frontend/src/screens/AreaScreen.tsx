import { NavigationProp } from '@react-navigation/native';
import { Text, View, StyleSheet } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

type Props = { navigation: NavigationProp<TodoListNavigatorParams> };
export default function AreaScreen({ navigation }: Props) {
  return (
    <View style={[styles.container]}>
      <Text>This is the area screen</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
