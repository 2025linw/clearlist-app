import { NavigationProp } from '@react-navigation/native';
import { Text } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

import Layout from '#/components/layout';

type Props = { navigation: NavigationProp<TodoListNavigatorParams> };
export default function ListScreen({ navigation }: Props) {
  return (
    <Layout>
      <Layout.Header>
        <Layout.Header.BackButton />
      </Layout.Header>

      <Layout.Content>
        <Text>This is the list screen</Text>
      </Layout.Content>
    </Layout>
  );
}
