import { View, Text } from 'react-native';

import { RoutesContainer, TabsNavigator } from '#/Navigation';

function ShellInner() {
  return (
    <>
      <TabsNavigator />
    </>
  )
}

export function Shell() {
  return (
    <View>
      <RoutesContainer>
        <Text>
          Hi, from the web
        </Text>
      </RoutesContainer>
    </View>
  )
}
