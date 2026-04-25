import { useRouter } from 'expo-router';
import { useState } from 'react';
import { Button, StyleSheet, Text, View } from 'react-native';

import { useSessionApi } from '@/context/auth';
import usePersisted from '@/hooks/use-persisted';

import FormField from '@/components/forms/form-field';
import Layout from '@/components/layout';

import { authClient } from '@/lib/auth-client';
import { useTheme } from '@/context/theme';

export default function Index() {
  const router = useRouter();
  const { logout } = useSessionApi();
  const { setThemeVariant } = useTheme();
  const { data, isPending } = authClient.useSession();

  // console.log(theme);

  return !isPending && data ? (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
    >
      <FormField label="Mode">
        <View style={styles.colorMode}>
          <Button
            title="Auto"
            onPress={() => {}}
          />
          <Button
            title="Light"
            onPress={() => {}}
          />
          <Button
            title="Dark"
            onPress={() => {}}
          />
        </View>
      </FormField>
      <Button
        title="Logout"
        onPress={() => logout().finally(() => router.navigate('/login'))}
      />
    </Layout>
  ) : (
    <Layout>
      <Text>Loading...</Text>
    </Layout>
  );
}

const styles = StyleSheet.create({
  colorMode: {
    flexDirection: 'row',
  },
});
