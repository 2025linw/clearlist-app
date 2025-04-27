import { ComponentProps } from 'react';
import { StyleSheet, Pressable, View, Text, GestureResponderEvent } from 'react-native';

import { Ionicons } from '@expo/vector-icons';

type Props = {
  label: string;
  iconName?: ComponentProps<typeof Ionicons>['name'];
  iconColor?: string;
  onPress: (event: GestureResponderEvent) => void;
};

export default function NavButton( { label, iconName='add', iconColor='#000', onPress }: Props ) {
  return (
    <View style={style.container}>
      <Pressable style={style.button} onPress={onPress}>
        <Ionicons name={iconName} style={[style.icon, {color: iconColor}]} />
        <Text style={style.label}>{label}</Text>
      </Pressable>
    </View>
  )
}

const style = StyleSheet.create({
  container: {
    width: '100%',
    height: 25,

    justifyContent: 'center', // horizontal alignment
    alignItems: 'center', // vertical alignment
  },
  button: {
    width: '100%',

    flexDirection: 'row',

    justifyContent: 'flex-start', // horizontal alignment
    alignItems: 'center', // vertical alignment
  },
  icon: {
    marginRight: 5,

    fontSize: 18,
  },
  label: {
    fontSize: 16,
  },
});
