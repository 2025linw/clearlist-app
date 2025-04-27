import { Slot, Stack, useRouter } from 'expo-router';
import { Platform, StyleSheet, ScrollView, View, Text, FlatList } from 'react-native';

import NavButton from '@/components/NavButton';
import Sidebar from '@/components/sidebar/Sidebar';

export default function RootLayout() {
  const isWeb = Platform.OS === 'web';

  if (isWeb) {
    let router = useRouter();

    return (
      <View style={styles.webContainer}>
        <Sidebar style={styles.sidebarContainer}>
          <NavButton label='Inbox' iconName='file-tray' iconColor='#1cadf6' onPress={() => router.navigate('/(pages)/inbox')} />
          <hr style={{ borderWidth: 0 }} />
          <NavButton label='Today' iconName='star' iconColor='#ffd400' onPress={() => router.navigate('/(pages)/today')} />
          <NavButton label='Upcoming' iconName='calendar' iconColor='#fa1854' onPress={() => router.navigate('/(pages)/upcoming')} />
          <NavButton label='Deadline' iconName='flag' iconColor='#fc476e' onPress={() => router.navigate('/(pages)/deadline')} />
          <hr style={{ borderWidth: 0 }} />
          <NavButton label='Logbook' iconName='checkbox' iconColor='#4cbf5f' onPress={() => router.navigate('/(pages)/logbook')} />
          <NavButton label='Trash' iconName='trash-bin' iconColor='#c2c7cb' onPress={() => router.navigate('/(pages)/trash')} />
        </Sidebar>
        <View style={styles.contentContainer}>
          <Slot />
        </View>
      </View>
    )
  }

  return (
    <Stack>
      <Stack.Screen />
    </Stack>
  );
}

const styles = StyleSheet.create({
  webContainer: {
    width: '100%',
    height: '100%',

    flex: 1,
    flexDirection: 'row',

    backgroundColor: '#fff',
  },
  sidebarContainer: {
    width: 200,
  },
  contentContainer: {
    flex: 1,
  },
});
