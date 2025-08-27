import { NavigationProp } from '@react-navigation/native';
import { Text, View, StyleSheet } from 'react-native';

import { TodoTabNavigatorParams } from '#/types/routes';

import Layout from '#/components/layout';

type Props = { navigation: NavigationProp<TodoTabNavigatorParams> };
export default function ListScreen({}: Props) {
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
