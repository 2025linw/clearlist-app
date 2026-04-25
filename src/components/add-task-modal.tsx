import Ionicons from '@expo/vector-icons/Ionicons';
import { useState } from 'react';
import { Modal, Pressable, StyleSheet, View, ViewStyle } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, { useAnimatedStyle, useSharedValue, withSpring } from 'react-native-reanimated';
import { runOnJS } from 'react-native-worklets';

import useTheme from '@/hooks/use-theme';

import FormTextInput from '@/components/forms/form-text-input';

type Props = {
  style?: ViewStyle;
};

export default function AddTaskModal(props: Props) {
  const { currentColor } = useTheme();

  const [showModal, setShowModal] = useState(false);

  const isPressed = useSharedValue(false);
  const offset = useSharedValue({ x: 0, y: 0 });

  const start = useSharedValue({ x: 0, y: 0 });
  const gesture = Gesture.Pan()
    .onBegin(() => {
      isPressed.value = true;

      runOnJS(setShowModal)(true);
    })
    .onUpdate((e) => {
      offset.value = {
        x: e.translationX + start.value.x,
        y: e.translationY + start.value.y,
      };
    })
    .onEnd(() => {
      offset.value = {
        x: withSpring(0),
        y: withSpring(0),
      };
    })
    .onFinalize(() => {
      isPressed.value = false;
    });

  const animatedStyles = useAnimatedStyle(() => {
    return {
      transform: [
        { translateX: offset.value.x },
        { translateY: offset.value.y },
        { scale: withSpring(isPressed.value ? 1.2 : 1) },
      ],
    };
  });

  return (
    <>
      <Modal
        visible={showModal}
        transparent={true}
      >
        <Pressable
          style={styles.container}
          onPress={() => setShowModal(false)}
        >
          <Pressable
            style={styles.modal}
            onPress={(e) => e.stopPropagation()}
          >
            <View>
              <FormTextInput label={'Title'} />
            </View>
          </Pressable>
        </Pressable>
      </Modal>

      <GestureDetector gesture={gesture}>
        <Animated.View style={[styles.button, animatedStyles]}>
          <Ionicons
            name="add-circle"
            size={64}
            color={currentColor.primary}
          />
        </Animated.View>
      </GestureDetector>
    </>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,

    justifyContent: 'center',
    alignItems: 'center',

    backgroundColor: 'red',
  },
  modal: {
    width: '100%',
    height: '30%',

    padding: 25,

    backgroundColor: 'blue',
  },
  button: {
    position: 'absolute',
    right: 25,
    bottom: 25,
  },
});
