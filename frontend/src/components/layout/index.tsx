import { FC } from 'react';
import { View, ViewProps, type StyleProp, type ViewStyle } from 'react-native';

import { atoms as a } from '#/alf';

import Header from './header';
import {Content} from './content';

type Props = ViewProps & { style?: StyleProp<ViewStyle> };
function Layout({ style, ...props }: Props) {
  return <View style={[a.util_screen_outer, style]} {...props} />;
}

Layout.Header = Header;
Layout.Content = Content;

export default Layout;
