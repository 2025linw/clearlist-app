import { Text, View, StyleSheet } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

type Props = { navigation: AllNavigationProp };
export default function CalendarScreen({}: Props) {
  return (
    <View style={styles.container}>
      <Text>This is the calendar screen</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
