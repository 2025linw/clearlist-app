import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useSessionApi } from '@/context/auth';
import { useThemeMode } from '@/context/theme';

import FormField from '@/components/forms/form-field';
import Layout from '@/components/layout';
import Button from '@/components/primitives/button';
import Typography from '@/components/primitives/typography';

import { authClient } from '@/lib/auth-client';

export default function Index() {
  const router = useRouter();
  const { logout } = useSessionApi();

  const [themeMode, setThemeMode] = useThemeMode();

  const { data, isPending } = authClient.useSession();

  return !isPending && data ? (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
    >
      <FormField label="Mode">
        <View style={styles.colorMode}>
          <Button
            text="System"
            style={{ flex: 1 }}
            scheme={themeMode === 'system' ? 'primary' : 'default'}
            onPress={() => setThemeMode('system')}
          />
          <Button
            text="Light"
            style={{ flex: 1 }}
            scheme={themeMode === 'light' ? 'primary' : 'default'}
            onPress={() => setThemeMode('light')}
          />
          <Button
            text="Dark"
            style={{ flex: 1 }}
            scheme={themeMode === 'dark' ? 'primary' : 'default'}
            onPress={() => setThemeMode('dark')}
          />
        </View>
      </FormField>

      <Button
        text="Debug"
        onPress={() => router.navigate('/settings/typography-debug')}
      />
      <Button
        text="Logout"
        onPress={() => logout().finally(() => router.navigate('/login'))}
      />
    </Layout>
  ) : (
    <Layout>
      <Typography>Loading...</Typography>
    </Layout>
  );
}

const styles = StyleSheet.create({
  colorMode: {
    flexDirection: 'row',
  },
});
