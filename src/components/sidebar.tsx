import { usePathname, useRouter } from 'expo-router';
import { Button, StyleProp, StyleSheet, View, ViewStyle } from 'react-native';

import Layout from '@/components/layout';

type Props = {
  style?: StyleProp<ViewStyle>;
};

export default function Sidebar(props: Props) {
  const pathname = usePathname();
  const router = useRouter();

  return (
    <Layout style={[props.style]}>
      <View style={styles.container}>
        <Button
          color={pathname === '/list/inbox' ? 'red' : undefined}
          title="Inbox"
          onPress={() => router.navigate('/lists/inbox')}
        />
        <Button
          color={pathname === '/list/today' ? 'red' : undefined}
          title="Today"
          onPress={() => router.navigate('/lists/today')}
        />
        <Button
          color={pathname === '/list/upcoming' ? 'red' : undefined}
          title="Upcoming"
          onPress={() => router.navigate('/lists/upcoming')}
        />
        <Button
          color={pathname === '/list/deadline' ? 'red' : undefined}
          title="Deadline"
          onPress={() => router.navigate('/lists/deadline')}
        />

        <Button
          title="Settings"
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
