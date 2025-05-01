import { NavigationContainer } from '@react-navigation/native';

import type { BottomTabBarProps } from '@react-navigation/bottom-tabs';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';

import type {
  FlatNavigatorParams,
  BottomTabNavigatorParams,
} from '#/lib/routes/types';

import { MainScreen } from '#/view/screens/Main'

import { createNativeStackNavigatorWithAuth } from '#/view/shell/createNativeStackNavigatorWithAuth';

const MainTab = createNativeStackNavigatorWithAuth<HomeTabNavigatorParams>();
const Flat = createNativeStackNavigatorWithAuth<FlatNavigatorParams>();
const Tab = createBottomTabNavigator<BottomTabNavigatorParams>();

function TabsNavigator() {
  return (
    <Tab.Navigator
      initialRouteName='MainTab'
    >
      <Tab.Screen name='MainTab' getComponent={() => MainTabNavigator} />
    </Tab.Navigator>
  );
}

function MainTabNavigator() {
  return (
    <View>
      <Text>Hi, this is main</Text>
    </View>
  )
}

function FlatNavigator() {
  return (
    <Flat.Navigator>
      <Flat.Screen
        name="Main"
        getComponent={() => MainScreen}
        options={{title: "Main"}}
      />
    </Flat.Navigator>
  );
}

function RoutesContainer({ children }: React.PropsWithChildren<{}>) {
  return (
    <NavigationContainer>
      {children}
    </NavigationContainer>
  );
}

export {
  FlatNavigator,
  TabsNavigator,
  RoutesContainer,
}
