import { Text } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

import Layout from '#/components/layout';

type Props = { navigation: AllNavigationProp };
export default function SearchScreen({}: Props) {
  return (
    <Layout>
      <Layout.Content>
        <Text>This is the search screen</Text>
      </Layout.Content>
    </Layout>
  );
}
