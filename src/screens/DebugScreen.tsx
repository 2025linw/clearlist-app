import { AllNavigationProp } from '#/types/routes';

import { atoms as a } from '#/alf';

import { Button, ButtonText } from '#/components/Button';
import Layout from '#/components/layout';

type Props = { navigation: AllNavigationProp };
export default function DebugScreen({ navigation }: Props) {
  return (
    <Layout>
      <Layout.Content>
        <Button label="primary" color="primary">
          <ButtonText>Primary</ButtonText>
        </Button>
        <Button label="primary-disabled" color="primary" disabled>
          <ButtonText>Primary Disabled</ButtonText>
        </Button>
        <Button label="secondary" color="secondary">
          <ButtonText>Secondary</ButtonText>
        </Button>
        <Button label="secondary-disabled" color="secondary" disabled>
          <ButtonText>Secondary Disabled</ButtonText>
        </Button>
        <Button label="theme" color="theme">
          <ButtonText>Theme</ButtonText>
        </Button>
        <Button label="theme-disabled" color="theme" disabled>
          <ButtonText>Theme Disabled</ButtonText>
        </Button>

        <Button
          label="returnHome"
          color="primary"
          onPress={navigation.popToTop}
          style={[a.fixed, a.bottom_0, a.left_0]}
        >
          <ButtonText>Return Home</ButtonText>
        </Button>
      </Layout.Content>
    </Layout>
  );
}
