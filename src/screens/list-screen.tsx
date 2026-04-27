import { FlatList, StyleSheet, View } from 'react-native';

import { Task } from '@/types';

import Layout from '@/components/layout';
import Typography from '@/components/primitives/typography';
import TaskItem from '@/components/task-component';

export type Props = {
  listName: string;

  tasks?: Task[] | null;
};

export default function ListScreen(props: Props) {
  return (
    <Layout
      headerText={props.listName}
      showBackButton
      showAddModal
    >
      <FlatList
        data={props.tasks}
        keyExtractor={(task) => task.id}
        renderItem={({ item }) => <TaskItem task={item} />}
        style={styles.list}
        contentContainerStyle={styles.listContainer}
        ListEmptyComponent={
          <View style={styles.empty}>
            <Typography>{props.tasks === null ? 'Loading tasks...' : 'Create a new task!'}</Typography>
          </View>
        }
      />
    </Layout>
  );
}

const styles = StyleSheet.create({
  list: {
    flex: 1,
  },
  listContainer: {
    flexGrow: 1,
  },
  empty: {
    flex: 1,

    alignItems: 'center',
    justifyContent: 'center',
  },
});
