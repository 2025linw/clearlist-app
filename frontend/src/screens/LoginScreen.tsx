import { useState } from 'react';
import { StyleSheet, TextInput } from 'react-native';

import { atoms as a } from '#/alf';

import { useSessionApi } from '#/state/session';

import { Button, ButtonText } from '#/components/Button';
import Layout from '#/components/layout';

// TODO: does LoginScreen need Props
// type Props = {};
export default function LoginScreen() {
  // TODO: add theming
  const [email, setEmail] = useState(__DEV__ ? 'testuser1@email.com' : '');
  const [password, setPassword] = useState(__DEV__ ? 'testpassword' : '');

  const { login, createAccount } = useSessionApi();

  return (
    <Layout>
      <Layout.Centered>
        <Layout.Content
          scrollEnabled={false}
          contentContainerStyle={[a.flex_col]}
        >
          <TextInput
            style={[styles.input, a.mt_0]}
            onChangeText={setEmail}
            value={email}
            placeholder="Email address"
            placeholderTextColor="black"
            autoCapitalize="none"
            autoCorrect={false}
            autoComplete="email"
            textContentType="emailAddress"
          />

          <TextInput
            style={styles.input}
            onChangeText={setPassword}
            value={password}
            placeholder="Password"
            placeholderTextColor="black"
            secureTextEntry={true}
            autoCapitalize="none"
            autoCorrect={false}
            autoComplete="current-password"
            textContentType="password"
          />

          <Button
            label="Login"
            color="theme"
            style={[a.mx_sm, a.mt_sm]}
            onPress={() =>
              login({ email: email.toLowerCase(), password: password })
            }
          >
            <ButtonText>Login</ButtonText>
          </Button>

          <Button
            label="Register"
            color="secondary"
            style={[a.mx_sm, a.mt_sm]}
            onPress={() =>
              createAccount({ email: email.toLowerCase(), password: password })
            }
          >
            <ButtonText>Register</ButtonText>
          </Button>
        </Layout.Content>
      </Layout.Centered>
    </Layout>
  );
}

const styles = StyleSheet.create({
  input: {
    ...a.flex_1,
    height: 50,
    ...a.mx_md,
    ...a.mt_sm,
    ...a.border,
    ...a.p_lg,
    color: 'black',
  },
  button: { ...a.mx_sm, ...a.mt_sm },
});
