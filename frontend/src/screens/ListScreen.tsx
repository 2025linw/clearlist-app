import { NavigationProp, RouteProp } from '@react-navigation/native';
import { useCallback, useEffect, useState } from 'react';
import { Text, View } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

import { queryTask } from '#/services';
import { TaskSchema } from '#/services/schemas';

import Layout from '#/components/layout';

type Props = {
  navigation: NavigationProp<TodoListNavigatorParams>;
  route: RouteProp<TodoListNavigatorParams>;
};
export default function ListScreen({ route }: Props) {
  const [data, setData] = useState<TaskSchema[]>([]);

  const page = route.params?.page ?? 'index';

  useEffect(() => {
    async function getTasks() {
      const res = await queryTask({});

      setData(res.data);
    }

    getTasks();
  }, []);

  return (
    <Layout>
      <Layout.Header>
        <Layout.Header.BackButton />
      </Layout.Header>

      <Layout.DataContent
        data={data}
        renderItem={({ item }) => <Todo title={item.title} />}
        keyExtractor={item => item.id}
      />
    </Layout>
  );
}

// type TodoProps = TaskSchema;
type TodoProps = { title: string };
function Todo({ title }: TodoProps) {
  return (
    <View>
      <Text>{title}</Text>
    </View>
  );
}
