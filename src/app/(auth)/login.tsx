import { useRouter } from 'expo-router';
import { useState } from 'react';
import { Button, StyleSheet, TextInput, View } from 'react-native';

import { useSessionApi } from '@/context/auth';

import FormField from '@/components/forms/form-field';
import Layout from '@/components/layout';

export default function LoginPage() {
  const router = useRouter();

  const { login } = useSessionApi();

  const [email, setEmail] = useState('will@email.com');
  const [password, setPassword] = useState('testpass');
  const [isPasswordFocused, setPasswordFocused] = useState(false);

  return (
    <Layout
      headerText="Login"
      style={[styles.container]}
    >
      <View style={styles.loginContainer}>
        <FormField
          label={'Email'}
          style={styles.loginField}
        >
          <TextInput
            style={[styles.loginField, { flex: 2 }]}
            placeholder="Email"
            value={email}
            onChangeText={setEmail}
            autoCapitalize="none"
            autoComplete="email"
            autoCorrect={false}
          />
        </FormField>

        <FormField
          label={'Password'}
          style={styles.loginField}
        >
          <TextInput
            style={[styles.loginField, { flex: 2 }]}
            placeholder="Password"
            value={password}
            onChangeText={setPassword}
            autoCapitalize="none"
            autoComplete="new-password"
            autoCorrect={false}
            secureTextEntry={!isPasswordFocused}
            onFocus={() => setPasswordFocused(true)}
            onBlur={() => setPasswordFocused(false)}
          />
        </FormField>

        <View style={styles.loginField}>
          <Button
            title="Login"
            onPress={async () =>
              login({ email, password }).then(
                () => router.replace('/'),
                (e) => console.error(e),
              )
            }
          />
        </View>
      </View>

      <View style={styles.footer}>
        <Button
          title="Don't have an account? Register"
          onPress={() => router.replace('/register')}
        />
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  container: { justifyContent: 'center', alignContent: 'center' },
  loginContainer: {
    backgroundColor: 'pink',
  },
  loginField: {
    padding: 5,
  },
  footer: {
    width: '100%',
    position: 'absolute',
    bottom: 20,
    alignContent: 'center',
  },
});
