import { Text, View, StyleSheet } from 'react-native';

import { useTheme } from '#/alf';

type Props = {};
export default function SearchScreen({}: Props) {
  const t = useTheme();

  return (
    <View style={[styles.container, t.atoms.bg]}>
      <Text style={t.atoms.text}>Page was not found</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
