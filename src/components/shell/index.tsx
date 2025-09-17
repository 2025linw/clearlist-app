import { SafeAreaView } from 'react-native-safe-area-context';

import { atoms as a, useTheme } from '#/alf';

import { RoutesContainer, TabsNavigator } from '#/Navigation';

function ShellInner() {
  return <TabsNavigator />;
}

export default function Shell() {
  const t = useTheme();

  return (
    <SafeAreaView style={[a.h_full, t.atoms.bg]} edges={['top']}>
      <RoutesContainer>
        <ShellInner />
      </RoutesContainer>
    </SafeAreaView>
  );
}
