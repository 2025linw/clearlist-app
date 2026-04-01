import { Task } from '@/types/resource';

import TaskItem from '@/components/task-item';

import { useEffect, useState } from 'react';
import { FlatList, Text, View } from 'react-native';

type TaskResponse = {
  status: string;
  data: {
    count: number;
    tasks: Task[];
  };
};

export default function Index() {
  const [isLoaded, setLoaded] = useState(false);
  const [data, setData] = useState<Task[]>([]);

  console.log(process.env);

  const getTasks = async () => {
    try {
      const res = await fetch('http://localhost:5000/api/tasks', {
        headers: {},
        credentials:
          process.env.NODE_ENV === 'development' ? 'include' : 'same-origin',
      });
      if (res.status !== 200) {
        console.error(await res.text());

        return;
      }

      const json = (await res.json()) as TaskResponse;

      setData(json.data.tasks);
    } catch (err) {
      console.error(err);
    } finally {
      setLoaded(true);
    }
  };

  useEffect(() => {
    getTasks();
  }, []);

  return (
    <View
      style={{
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      {!isLoaded ? (
        <Text>Loading tasks...</Text>
      ) : data.length === 0 ? (
        <Text>There are no tasks...</Text>
      ) : (
        <FlatList
          data={data}
          keyExtractor={({ id }) => id}
          renderItem={() => <TaskItem />}
        />
      )}
    </View>
  );
}
