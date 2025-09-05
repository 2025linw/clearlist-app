import { NavigationProp } from '@react-navigation/native';
import { type ComponentType } from 'react';

import { TodoListNavigatorParams } from '#/types/routes';

import { atoms as a } from '#/alf';

import {
  Button as BaseButton,
  type ButtonProps as BaseButtonProps,
  ButtonIcon,
  ButtonText,
} from '#/components/Button';
import { Calendar as X } from '#/components/icons/Calendar';
import { Props as IconProps } from '#/components/icons/common';
import Layout from '#/components/layout';

type Props = { navigation: NavigationProp<TodoListNavigatorParams> };
export default function TodoScreen({ navigation }: Props) {
  return (
    <Layout>
      <Layout.Content>
        <Button
          text="Inbox"
          icon={X}
          label="inbox"
          onPress={() => navigation.navigate('List')}
          style={a.mb_lg}
        />

        <Button
          text="Today"
          icon={X}
          label="today"
          onPress={() => navigation.navigate('List')}
        />
        <Button
          text="Upcoming"
          icon={X}
          label="upcoming"
          onPress={() => navigation.navigate('List')}
        />
        <Button
          text="Deadline"
          icon={X}
          label="deadline"
          onPress={() => navigation.navigate('List')}
        />
        <Button
          text="Anytime"
          icon={X}
          label="anytime"
          onPress={() => navigation.navigate('List')}
        />
        <Button
          text="Someday"
          icon={X}
          label="someday"
          onPress={() => navigation.navigate('List')}
        />
        <Button
          text="Logbook"
          icon={X}
          label="logbook"
          onPress={() => navigation.navigate('List')}
          style={a.mt_lg}
        />
        <Button
          text="Trash"
          icon={X}
          label="trash"
          onPress={() => navigation.navigate('List')}
        />

        <Button
          text="Settings"
          icon={X}
          label="settings"
          onPress={() => navigation.navigate('Setting')}
          style={a.mt_lg}
        />
      </Layout.Content>
    </Layout>
  );
}

type ButtonProps = Omit<BaseButtonProps, 'children'> & {
  icon: ComponentType<IconProps>;
  text: string;
};
function Button({ text, icon, label, onPress, style, ...rest }: ButtonProps) {
  return (
    <BaseButton {...rest} label={label} onPress={onPress} style={style}>
      <ButtonIcon icon={icon} />
      <ButtonText>{text}</ButtonText>
    </BaseButton>
  );
}
