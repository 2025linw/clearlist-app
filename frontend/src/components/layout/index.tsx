import { Attributes, Children, cloneElement, isValidElement } from 'react';
import { View, ViewProps, type StyleProp, type ViewStyle } from 'react-native';

import { atoms as a } from '#/alf';

import { Content } from './content';
import Header from './header';

type Props = ViewProps & { style?: StyleProp<ViewStyle> };
function Layout({ style, children, ...props }: Props) {
  let hasHeader = false;
  Children.forEach(children, child => {
    if (isValidElement(child) && child.type === Header) {
      hasHeader = true;
    }
  });

  return (
    <View style={[a.util_screen_outer, style]} {...props}>
      {hasHeader
        ? Children.map(children, child => {
            if (isValidElement(child) && child.type === Content) {
              return cloneElement(child, {
                hasHeader: true,
              } as Attributes)
            }

            return child;
          })
        : children}
    </View>
  );
}

Layout.Header = Header;
Layout.Content = Content;

export default Layout;
