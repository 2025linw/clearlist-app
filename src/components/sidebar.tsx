import { useRouter } from 'expo-router';
import { StyleProp, StyleSheet, View, ViewStyle } from 'react-native';

import Layout from '@/components/layout';
import Button from '@/components/primitives/button';

type Props = {
  style?: StyleProp<ViewStyle>;
};

export default function Sidebar(props: Props) {
  const router = useRouter();

  return (
    <Layout style={[props.style]}>
      <View style={styles.container}>
        <Button
          text="Inbox"
          onPress={() => router.navigate('/lists/inbox')}
        />
        <Button
          text="Today"
          onPress={() => router.navigate('/lists/today')}
        />
        <Button
          text="Upcoming"
          onPress={() => router.navigate('/lists/upcoming')}
        />
        <Button
          text="Deadline"
          onPress={() => router.navigate('/lists/deadline')}
        />

        <Button
          text="Setting"
          onPress={() => router.navigate('/settings')}
        />
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  layout: {
    alignItems: 'center',
  },
  container: {
    width: '100%',
    height: '100%',
  },
  selected: {
    backgroundColor: 'red',
  },
});
