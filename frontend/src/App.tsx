import * as SplashScreen from 'expo-splash-screen';
import React from 'react';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import { ThemeProvider as Alf } from '#/alf';

import { init as initPersistedState } from '#/storage/async-storage';
import {
  Provider as SessionProvider,
  useSession,
  useSessionApi,
} from '#/state/session';
import { type AccountSchema } from '#/state/session/types';

import Shell from '#/components/shell';

import Splash from '#/Splash';

SplashScreen.preventAutoHideAsync();

function InnerApp() {
  const [isReady, setReady] = React.useState(false);

  // const theme = useColorTheme();
  const theme = 'light';

  const { account } = useSession();
  const { resumeSession } = useSessionApi();

  // Init
  React.useEffect(() => {
    async function onLaunch(account: AccountSchema) {
      try {
        await resumeSession(account);
      } catch (e) {
        console.error('resume session:', e);
      } finally {
        setReady(true);
      }
    }

    onLaunch(account);
  }, []);

  // TODO: check here for when session is dropped (i.e. Refresh JWT has expired)

  return (
    <Alf theme={theme}>
      <Splash isReady={isReady}>
        <Shell />
      </Splash>
    </Alf>
  );
}

export default function App() {
  const [isReady, setReady] = React.useState(false);

  // Setup
  React.useEffect(() => {
    Promise.all([
      initPersistedState(), // Initialize persisted data
    ]).then(() => setReady(true));
  }, []);

  if (!isReady) {
    return null;
  }

  return (
    <SessionProvider>
      <SafeAreaProvider>
        <InnerApp />
      </SafeAreaProvider>
    </SessionProvider>
  );
}
