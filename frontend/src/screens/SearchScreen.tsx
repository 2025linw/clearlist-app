import { Text, View, StyleSheet } from 'react-native';

type Props = {};
export default function SearchScreen({}: Props) {
  return (
    <View style={styles.container}>
      <Text>This is the search screen</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
});
