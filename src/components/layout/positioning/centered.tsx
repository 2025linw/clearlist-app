import { PropsWithChildren } from 'react';
import { View } from 'react-native';

import { atoms as a } from '#/alf';

type Props = PropsWithChildren & {};
export function Centered({ children, ...rest }: Props) {
  return (
    <View style={[a.w_full, a.my_auto]} {...rest}>
      {children}
    </View>
  );
}
