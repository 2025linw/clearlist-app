import { NavigationProp } from '@react-navigation/native';
import { Text } from 'react-native';

import { TodoListNavigatorParams } from '#/types/routes';

import Layout from '#/components/layout';

type Props = { navigation: NavigationProp<TodoListNavigatorParams> };
export default function AreaScreen({ navigation }: Props) {
  return (
    <Layout>
      <Layout.Content>
        <Text>This is the area screen</Text>
      </Layout.Content>
    </Layout>
  );
}
