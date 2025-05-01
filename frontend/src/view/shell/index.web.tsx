import { View, Text } from 'react-native';

import { FlatNavigator, RoutesContainer } from '#/Navigation';

function ShellInner() {
  return (
    <>
      <FlatNavigator />
    </>
  )
}

export function Shell() {
  return (
    <View>
      <RoutesContainer>
        <ShellInner />
      </RoutesContainer>
    </View>
  )
}
