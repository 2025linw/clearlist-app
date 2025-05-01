import React from 'react';
import { View } from 'react-native';

import {
  createNavigatorFactory,
  type EventArg,
  type NavigatorTypeBagBase,
  type ParamListBase,
  StackActions,
  type StackActionHelpers,
  type StackNavigationState,
  StackRouter,
  type StackRouterOptions,
  type StaticConfig,
  type TypedNavigator,
  useNavigationBuilder,
} from '@react-navigation/native';

import {
  type NativeStackNavigationEventMap,
  type NativeStackNavigationOptions,
  type NativeStackNavigationProp,
  type NativeStackNavigatorProps,
  NativeStackView,
} from '@react-navigation/native-stack';

import { useSession } from '#/state/session';
import { LoggedOut } from '#/view/com/auth/LoggedOut';
import { isNative } from '#/components/detection';

type NativeStackNavigationOptionsWithAuth = NativeStackNavigationOptions & {
  requireAuth?: boolean;
}

function NativeStackNavigator( {
  id,
  initialRouteName,
  children,
  screenListeners,
  screenOptions,
  ...rest
}: NativeStackNavigatorProps ) {
  // This is from the original native stack navigator
  const { state, descriptors, navigation, NavigationContent, describe } =
    useNavigationBuilder<
      StackNavigationState<ParamListBase>,
      StackRouterOptions,
      StackActionHelpers<ParamListBase>,
      NativeStackNavigationOptionsWithAuth,
      NativeStackNavigationEventMap
    >(StackRouter, {
      id,
      initialRouteName,
      children,
      screenListeners,
      screenOptions,
    });

  React.useEffect(
    () =>
      // @ts-expect-error: there may not be a tab navigator in parent
      navigation?.addListener?.('tabPress', (e: any) => {
        const isFocused = navigation.isFocused();

        // Run the operation in the next frame so we're sure all listeners have been run
        // This is necessary to know if preventDefault() has been called
        requestAnimationFrame(() => {
          if (
            state.index > 0 &&
            isFocused &&
            !(e as EventArg<'tabPress', true>).defaultPrevented
          ) {
            // When user taps on already focused tab and we're inside the tab,
            // reset the stack to replicate native behaviour
            navigation.dispatch({
              ...StackActions.popToTop(),
              target: state.key,
            });
          }
        });
      }),
    [navigation, state.index, state.key]
  );

  // My custom logic
  const { hasSession, currentAccount } = useSession();
  const activeRoute = state.routes[state.index];
  const activeDescriptor = descriptors[activeRoute.key]
  const activeRouteRequiresAuth = activeDescriptor.options.requireAuth ?? false;

  if (!hasSession && (activeRouteRequiresAuth || isNative)) {
    return <LoggedOut />
  }

  const newDescriptors: typeof descriptors = {};

  return (
    <NavigationContent>
      <View>
        <NativeStackView
          {...rest}
          state={state}
          navigation={navigation}
          descriptors={newDescriptors}
          describe={describe}
        />
      </View>
    </NavigationContent>
  );
}

export function createNativeStackNavigatorWithAuth<
  const ParamList extends ParamListBase,
  const NavigatorID extends string | undefined = undefined,
  const TypeBag extends NavigatorTypeBagBase = {
    ParamList: ParamList;
    NavigatorID: NavigatorID;
    State: StackNavigationState<ParamList>;
    ScreenOptions: NativeStackNavigationOptionsWithAuth;
    EventMap: NativeStackNavigationEventMap;
    NavigationList: {
      [RouteName in keyof ParamList]: NativeStackNavigationProp<
        ParamList,
        RouteName,
        NavigatorID
      >;
    };
    Navigator: typeof NativeStackNavigator;
  },
  const Config extends StaticConfig<TypeBag> = StaticConfig<TypeBag>,
>(config?: Config): TypedNavigator<TypeBag, Config> {
  return createNavigatorFactory(NativeStackNavigator)(config);
}

// export function createNativeStackNavigatorWithAuth<
//   StackNavigationState<ParamListBase>,
//   NativeStackNavigationOptionsWithAuth,
//   NativeStackNavigationEventMap,
//   typeof NativeStackNavigator
// >

// export const createNativeStackNavigatorWithAuth = createNavigatorFactory<
// >(NativeStackNavigator);
