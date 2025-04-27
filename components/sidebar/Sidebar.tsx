import { ReactNode } from 'react';
import { ScrollView, StyleProp, StyleSheet, View, ViewStyle } from 'react-native';

type Props = {
  style?: StyleProp<ViewStyle>,
  children: ReactNode;
}

export default function Sidebar({ style, children }: Props) {
  return (
    <View style={[styles.container, style]}>
      <ScrollView style={styles.sidebar}>
        {children}
      </ScrollView>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {

  },
  sidebar: {
    paddingVertical: 20,
    paddingHorizontal: 15,

    backgroundColor: '#eee',

    borderRightWidth: 1,
    borderRightColor: '#aaa',
  },

})
