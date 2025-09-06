import * as SplashScreen from 'expo-splash-screen';
import { useState, useEffect } from 'react';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import { ThemeProvider as Alf } from '#/alf';

import {
  Provider as SessionProvider,
  useSession,
  useSessionApi,
} from '#/state/session';
import { init as initPersistedState } from '#/storage/async-storage';
import { type AccountSchema } from '#/storage/schemas';

import Shell from '#/components/shell';

import Splash from '#/Splash';
import { isJwtExpired } from '#/util/isSessionExpired';
import { useColorTheme } from '#/util/useColorTheme';

export const DEBUG = true;

SplashScreen.preventAutoHideAsync();

function InnerApp() {
  const [isReady, setReady] = useState(false);

  const theme = useColorTheme();

  const { account } = useSession();
  const { resumeSession } = useSessionApi();

  // Init
  useEffect(() => {
    async function onLaunch(account?: AccountSchema) {
      if (!account || !account.refreshJwt) {
        // NOTE: this means that no-one is currently logged in
        // This behavior might change though as more development is done

        setReady(true);

        return undefined;
      }
      if (isJwtExpired(account.refreshJwt)) {
        // TODO: Create menu to show that login has expired
        setReady(true);

        return undefined;
      }

      try {
        await resumeSession(account);
      } catch (e) {
        console.error('initialResumeSession', e);
      } finally {
        setReady(true);
      }
    }

    onLaunch(account);
  }, []);

  return (
    <Alf theme={theme}>
      <Splash isReady={isReady}>
        <Shell />
      </Splash>
    </Alf>
  );
}

export default function App() {
  const [isReady, setReady] = useState(false);

  // Setup
  useEffect(() => {
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
