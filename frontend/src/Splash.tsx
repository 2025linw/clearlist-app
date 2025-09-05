import * as SplashScreen from 'expo-splash-screen';
import { useEffect, PropsWithChildren } from 'react';
import { Text, View, StyleSheet } from 'react-native';

type Props = { isReady: boolean };
export default function Splash(props: PropsWithChildren<Props>) {
  const isReady = props.isReady;

  useEffect(() => {
    if (isReady) {
      SplashScreen.hideAsync().catch(e => {
        console.error('error', e);
      });
    }
  }, [isReady]);

  return (
    <>
      {isReady ? (
        <>{props.children}</>
      ) : (
        <View style={styles.container}>
          <Text>This is the Splash</Text>
        </View>
      )}
    </>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
});
