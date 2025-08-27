import { View } from 'react-native';

import { atoms as a } from '#/alf';

import { Button, ButtonText } from '#/components/Button';

type Props = {};
export default function DebugScreen({}: Props) {
  return (
    <View style={[a.px_sm]}>
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
    </View>
  );
}
