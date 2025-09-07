import {
  type BottomTabBarProps,
  createBottomTabNavigator,
} from '@react-navigation/bottom-tabs';
import {
  createNavigationContainerRef,
  NavigationContainer,
} from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { useCallback, JSX, PropsWithChildren } from 'react';

import {
  AllNavigatorParams,
  type BottomTabsNavigatorParams,
  type TodoListNavigatorParams,
} from '#/types/routes';

import { useTheme } from '#/alf';
import { Theme } from '#/alf/types';

import { useSession } from '#/state/session';

import { BottomBar } from '#/components/bottom-bar/BottomBar';
import { Calendar as X } from '#/components/icons/Calendar';

import AreaScreen from '#/screens/AreaScreen';
import CalendarTab from '#/screens/CalendarScreen';
import DebugScreen from '#/screens/DebugScreen';
import ListScreen from '#/screens/ListScreen';
import LoginScreen from '#/screens/LoginScreen';
import NotFound from '#/screens/NotFound';
import ProjectScreen from '#/screens/ProjectScreen';
import SearchTab from '#/screens/SearchScreen';
import SettingScreen from '#/screens/SettingScreen';
import TodoScreen from '#/screens/TodoScreen';

export const navigationRef = createNavigationContainerRef<AllNavigatorParams>();
type AllNavigatorsType = ReturnType<
  typeof createNativeStackNavigator<AllNavigatorParams>
>;

const Tab = createBottomTabNavigator<BottomTabsNavigatorParams>();

const TodoTab = createNativeStackNavigator<TodoListNavigatorParams>();

/*
 * Screens that are accesible from any Screen in any navigator
 *
 * This matches CommonNavigatorParams in `#/types/routes`
 */
function commonScreens(Stack: AllNavigatorsType) {
  return (
    <>
      <Stack.Screen name="NotFound" component={NotFound} />
      <Stack.Screen name="Setting" component={SettingScreen} />

      {/* TODO: add profile screen */}

      <Stack.Screen name="Debug" component={DebugScreen} />
    </>
  );
}

// Main tabs
export function TabsNavigator() {
  const { hasSession } = useSession();

  const tabBar = useCallback(
    (props: JSX.IntrinsicAttributes & BottomTabBarProps) => (
      <BottomBar {...props} />
    ),
    [],
  );

  if (!hasSession) {
    return <LoginScreen />;
  }

  return (
    <Tab.Navigator
      initialRouteName="TodoTab"
      backBehavior="initialRoute"
      screenOptions={{
        headerShown: false,
        // lazy: true, // TODO: consider whether this is necessary
      }}
      tabBar={tabBar}
    >
      <Tab.Screen
        name="TodoTab"
        component={TodoTabNavigator}
        options={{ tabBarIcon: () => <X style={{ color: 'black' }} /> }}
      />
      <Tab.Screen name="CalendarTab" component={CalendarTab} />
      <Tab.Screen name="SearchTab" component={SearchTab} />
    </Tab.Navigator>
  );
}

function screenOptions(t: Theme) {
  return {
    fullScreenGestureEnabled: true,
    headerShown: false,
    contentStyle: t.atoms.bg,
  };
}

// Todo tab
function TodoTabNavigator() {
  const t = useTheme();

  return (
    <TodoTab.Navigator initialRouteName="Todo" screenOptions={screenOptions(t)}>
      <TodoTab.Screen name="Todo" component={TodoScreen} />

      <TodoTab.Screen name="List" component={ListScreen} />
      <TodoTab.Screen name="Project" component={ProjectScreen} />
      <TodoTab.Screen name="Area" component={AreaScreen} />

      {commonScreens(TodoTab as AllNavigatorsType)}
    </TodoTab.Navigator>
  );
}

/*
 * Route container for all screens in app
 */
export function RoutesContainer({ children }: PropsWithChildren<unknown>) {
  return (
    <NavigationContainer ref={navigationRef}>{children}</NavigationContainer>
  );
}
