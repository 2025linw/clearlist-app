import { Text } from 'react-native';

import { AllNavigationProp } from '#/types/routes';

import { useTheme } from '#/alf';

import Layout from '#/components/layout';

type Props = { navigation: AllNavigationProp };
export default function SearchScreen({}: Props) {
  const t = useTheme();

  return (
    <Layout>
      <Layout.Content>
        <Text style={t.atoms.text}>Page was not found</Text>
      </Layout.Content>
    </Layout>
  );
}
