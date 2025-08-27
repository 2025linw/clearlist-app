import { View } from 'react-native';

import { atoms as a } from '#/alf';

type SlotProps = { children?: ReactNode };
export function Slot({ children }: SlotProps) {
  return <View style={[a.z_50]}>{children}</View>;
}
