import { ComponentProps } from 'react';
import { View, Text, StyleSheet, FlatList } from 'react-native';

import { Ionicons } from '@expo/vector-icons';

type Props = {
  headerLabel: string;
  headerIcon?: ComponentProps<typeof Ionicons>['name'];
  headerIconColor?: string;
  // data:
}

type TaskProps = {
  title: string;
}

function Task({ title }: TaskProps) {
  return (
    <View>
      <Text>{title}</Text>
    </View>
  )
}

export default function Content({ headerLabel, headerIcon='add', headerIconColor='#000' }: Props) {
  return (
    <FlatList
      data={DATA}
      renderItem={({ item }) => <Task title={item.title} />}
      keyExtractor={item => item.id}
      ListHeaderComponent={
        <View style={styles.header}>
          <Ionicons name={headerIcon}/>
          <Text>{headerLabel}</Text>
        </View>
      }
      style={styles.container}
      contentContainerStyle={styles.content}
    />
  );
}

const styles = StyleSheet.create({
  container: {
    padding: 50,
  },
  header: {
    flexDirection: 'row',
  },
  headerIcon: {

  },
  content: {

  },
});

// TODO: Remove temp data
const DATA = [
  {
    id: 'bd7acbea-c1b1-46c2-aed5-3ad53abb28ba',
    title: 'First Item',
  },
  {
    id: '3ac68afc-c605-48d3-a4f8-fbd91aa97f63',
    title: 'Second Item',
  },
  {
    id: '58694a0f-3da1-471f-bd96-145571e29d72',
    title: 'Third Item',
  },
];
