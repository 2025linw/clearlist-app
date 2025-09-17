import { Children, isValidElement } from 'react';
import { View, ViewProps, type StyleProp, type ViewStyle } from 'react-native';

import { atoms as a } from '#/alf';

import { Content, DataContent } from './content';
import { LayoutContext } from './context';
import Header from './header';
import { Centered } from './positioning/centered';

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

// Components
Layout.Header = Header;
Layout.Content = Content;
Layout.DataContent = DataContent;

// Layout Positioning
Layout.Centered = Centered;

export default Layout;
