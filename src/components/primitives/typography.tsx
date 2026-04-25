import { ReactNode } from 'react';
import { Text } from 'react-native';

type Props = {
  children: ReactNode;
};

export default function Typography({ children, ...props }: Props) {
  return <Text>{children}</Text>;
}
