import { Text } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

import Layout from '#/components/layout';

type Props = { navigation: AllNavigationProp };
export default function CalendarScreen({}: Props) {
  return (
    <Layout>
      <Layout.Content>
        <Text>This is the calendar screen</Text>
      </Layout.Content>
    </Layout>
  );
}
