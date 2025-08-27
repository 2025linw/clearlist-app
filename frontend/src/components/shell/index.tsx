import { View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { atoms as a, useTheme } from '#/alf';

import { RoutesContainer, TabsNavigator } from '#/Navigation';

function ShellInner() {
  // Make a component like Splash for login in between safe area and tab navigator
  
  return (
    <SafeAreaView style={[a.h_full]} edges={['top']}>
      <TabsNavigator />
    </SafeAreaView>
  );
}

export default function Shell() {
  const t = useTheme();

  return (
    <View style={[a.h_full, t.atoms.bg]}>
      <RoutesContainer>
        <ShellInner />
      </RoutesContainer>
    </View>
  );
}
