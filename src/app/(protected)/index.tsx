import Ionicons from '@expo/vector-icons/Ionicons';
import { useRouter } from 'expo-router';
import { useEffect, useState } from 'react';
import { FlatList, StyleSheet, Text, View } from 'react-native';

import { Tag } from '@/types';

import useTheme from '@/hooks/use-theme';
import { getTags } from '@/services/api';

import ListButton from '@/components/buttons/list-button';
import Layout from '@/components/layout';
import HorizontalDivider from '@/components/primitives/horizontal-divider';

export default function Index() {
  const router = useRouter();

  const { currentColor } = useTheme();

  const [tags, setTags] = useState<Tag[] | null>(null);

  useEffect(() => {
    getTags().then((tags) => {
      setTags(tags);
    });
  }, []);

  return (
    <View>
      <Layout>
        <View style={styles.navContainer}>
          <ListButton
            leftIcon={
              <Ionicons
                name="file-tray"
                size={18}
                color="skyblue"
              />
            }
            onPress={() => router.navigate('/lists/inbox')}
          >
            Inbox
          </ListButton>
          <ListButton
            leftIcon={
              <Ionicons
                name="sunny-sharp"
                size={18}
                color="yellow"
              />
            }
            onPress={() => router.navigate('/lists/today')}
          >
            Today
          </ListButton>
          <ListButton
            leftIcon={
              <Ionicons
                name="calendar"
                size={18}
                color="red"
              />
            }
            onPress={() => router.navigate('/lists/upcoming')}
          >
            Upcoming
          </ListButton>
          <ListButton
            leftIcon={
              <Ionicons
                name="flag"
                size={18}
                color="red"
              />
            }
            onPress={() => router.navigate('/lists/deadline')}
          >
            Deadline
          </ListButton>
          <ListButton
            leftIcon={
              <Ionicons
                name="checkmark-circle"
                size={18}
                color="green"
              />
            }
            onPress={() => router.navigate('/lists/logbook')}
          >
            Logbook
          </ListButton>
          <ListButton
            leftIcon={
              <Ionicons
                name="trash-bin"
                size={18}
                color="gray"
              />
            }
            onPress={() => router.navigate('/lists/trash')}
          >
            Trash
          </ListButton>

          {process.env.NODE_ENV === 'development' && (
            <ListButton
              leftIcon={
                <Ionicons
                  name="bug"
                  size={18}
                  color="blue"
                />
              }
              onPress={() => router.navigate('/lists/debug')}
            >
              Debug
            </ListButton>
          )}

          <HorizontalDivider />

          <Text style={{ color: currentColor.text }}>Tags</Text>

          <FlatList
            data={tags}
            keyExtractor={(tag) => tag.id}
            renderItem={({ item }) => (
              <View>
                <Text style={{ color: currentColor.text }}>{item.label}</Text>
              </View>
            )}
          />

          <HorizontalDivider />

          <ListButton
            leftIcon={
              <Ionicons
                name="settings"
                size={18}
                color={'gray'}
              />
            }
            onPress={() => router.navigate('/settings')}
          >
            Settings
          </ListButton>
        </View>
      </Layout>
    </View>
  );
}

const styles = StyleSheet.create({
  navContainer: {
    padding: 10,
  },
});
