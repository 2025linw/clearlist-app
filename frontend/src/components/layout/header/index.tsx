import { View } from 'react-native';
import { atoms as a } from '#/alf';
import {Slot} from './components/slot';
import {BackButton }from './components/backButton';


type HeaderProps = { children: ReactNode; sticky?: boolean };
function Header({ children }: HeaderProps) {
  return <View style={[a.mb_sm]}>{children}</View>;
}

Header.Slot = Slot;
Header.BackButton = BackButton;

export default Header;
