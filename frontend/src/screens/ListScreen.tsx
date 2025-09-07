import { NavigationProp, RouteProp } from '@react-navigation/native';
import { Text } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

import Layout from '#/components/layout';

type Props = {
  navigation: NavigationProp<TodoListNavigatorParams>;
  route: RouteProp<TodoListNavigatorParams>;
};
export default function ListScreen({ route }: Props) {
  const page = route.params?.page ?? 'index';

  return (
    <Layout>
      <Layout.Header>
        <Layout.Header.BackButton />
      </Layout.Header>

      <Layout.Content>
        <Text>This is the {page} page</Text>
      </Layout.Content>
    </Layout>
  );
}
