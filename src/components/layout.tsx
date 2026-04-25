import Ionicons from '@expo/vector-icons/Ionicons';
import { useRouter } from 'expo-router';
import { PropsWithChildren, ReactNode } from 'react';
import { Pressable, StyleProp, StyleSheet, Text, View, ViewStyle } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import useTheme from '@/hooks/use-theme';

import AddTaskModal from '@/components/add-task-modal';

type LayoutProps = PropsWithChildren & {
  showBackButton?: boolean;
  hasOptions?: boolean;
  showAddModal?: boolean;
  headerText?: string;
  headerIcon?: ReactNode; // TODO: create Icon node
  style?: StyleProp<ViewStyle>;
};

export default function Layout({
  children,
  showBackButton = false,
  hasOptions = false,
  showAddModal = false,
  ...props
}: LayoutProps) {
  const router = useRouter();

  const { currentColor } = useTheme();

  const canGoBack = router.canGoBack() && showBackButton;

  return (
    <SafeAreaView
      edges={['top', 'bottom']}
      style={[styles.container, { backgroundColor: currentColor.secondary }]}
    >
      {(canGoBack || props.headerText || props.headerIcon) && (
        <View style={styles.header}>
          <View style={styles.headerEle}>
            {canGoBack && (
              <Pressable onPress={() => router.back()}>
                <Ionicons
                  name="arrow-back-circle"
                  size={40}
                  color={currentColor.primary}
                />
              </Pressable>
            )}
          </View>

          <Text style={{ flex: 1, fontSize: 32, textAlign: 'center', color: currentColor.text }}>
            {props.headerText}
          </Text>

          <View style={styles.headerEle}>
            {hasOptions && (
              <Pressable>
                <Ionicons
                  name="ellipsis-horizontal-circle"
                  size={40}
                  color={currentColor.primary}
                />
              </Pressable>
            )}
          </View>
        </View>
      )}

      <View style={[styles.children, props.style]}>{children}</View>

      {showAddModal && <AddTaskModal />}
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    height: '100%',
    width: '100%',
  },
  header: {
    height: 40,

    paddingHorizontal: 10,

    flexDirection: 'row',
    alignItems: 'center',
  },
  headerEle: {
    width: 40,

    alignItems: 'center',
    justifyContent: 'center',
  },
  children: {
    flex: 1,
  },
  modal: {
    position: 'absolute',

    bottom: 25,
    right: 25,
  },
});
