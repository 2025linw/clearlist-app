import { NativeStackNavigationProp } from '@react-navigation/native-stack';

export type CommonNavigatorParams = {
  NotFound: undefined;
  Setting: undefined; // TODO: add parameters to adjust what settings screen (if navigating from calendar vs todo vs search)
  Debug: undefined;
};

export type BottomTabsNavigatorParams = CommonNavigatorParams & {
  TodoTab: undefined;
  CalendarTab: undefined;
  SearchTab: undefined;
};

export type TodoTabNavigatorParams = CommonNavigatorParams & {
  Todo: undefined;

  List: undefined;
  Project: undefined;
  Area: undefined;
};

export type AllNavigatorParams = CommonNavigatorParams &
  BottomTabsNavigatorParams &
  TodoTabNavigatorParams;
export type AllNavigationProp = NativeStackNavigationProp<AllNavigatorParams>;
