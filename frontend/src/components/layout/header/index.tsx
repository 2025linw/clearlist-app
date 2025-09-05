import { ReactNode } from 'react';
import { View } from 'react-native';

import { atoms as a } from '#/alf';

import { BackButton } from './components/backButton';
import { Slot } from './components/slot';

type HeaderProps = { children: ReactNode; sticky?: boolean };
function Header({ children }: HeaderProps) {
  return <View style={[a.z_50, a.mb_sm]}>{children}</View>;
}

Header.Slot = Slot;
Header.BackButton = BackButton;

export default Header;
