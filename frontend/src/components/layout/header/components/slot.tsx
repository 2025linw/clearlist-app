import { ReactNode } from 'react';
import { View } from 'react-native';

import { atoms as a } from '#/alf';

import { SLOT_SIZE } from '#/components/layout/header/const';

type SlotProps = { children?: ReactNode };
export function Slot({ children }: SlotProps) {
  return <View style={[a.z_50, { width: SLOT_SIZE }]}>{children}</View>;
}
