import { Children, createContext, isValidElement, useContext } from 'react';
import { View, ViewProps, type StyleProp, type ViewStyle } from 'react-native';

import { atoms as a } from '#/alf';

import { Content } from './content';
import Header from './header';

const LayoutContext = createContext({ hasHeader: false });

type Props = ViewProps & { style?: StyleProp<ViewStyle> };
function Layout({ style, children, ...props }: Props) {
  const hasHeader = Children.toArray(children).some(
    child => isValidElement(child) && child.type === Header,
  );

  return (
    <LayoutContext.Provider value={{ hasHeader }}>
      <View style={[a.util_screen_outer, style]} {...props}>
        {children}
      </View>
    </LayoutContext.Provider>
  );
}

Layout.Header = Header;
Layout.Content = Content;

export default Layout;

export function useLayoutContext() {
  return useContext(LayoutContext);
}
