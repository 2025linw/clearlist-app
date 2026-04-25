import Ionicons from '@expo/vector-icons/Ionicons';
import { Pressable, StyleSheet, Text, View } from 'react-native';

import { Task } from '@/types/resource';

import useTheme from '@/hooks/use-theme';

type TaskItemProp = {
  task: Task;
};

export default function TaskItem(props: TaskItemProp) {
  const { currentColor } = useTheme();

  return (
    <View style={styles.container}>
      <Pressable>
        <View style={styles.checkbox}>
          {props.task.completed_at ? (
            <Ionicons
              name="checkbox"
              size={16}
            />
          ) : (
            <Ionicons
              name="square-outline"
              size={16}
            />
          )}
        </View>
      </Pressable>

      <Text style={{ color: currentColor.text }}>{props.task.title}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '100%',

    margin: 10,

    flexDirection: 'row',

    alignItems: 'center',
    justifyContent: 'flex-start',
  },
  checkbox: {
    marginRight: 5,
  },
});
