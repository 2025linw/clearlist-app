import { useState } from 'react';
import { Text, View, StyleSheet, TextInput } from 'react-native';

import { useSessionApi } from '#/state/session';

import { Button, ButtonText } from '#/components/Button';

// TODO: does LoginScreen need Props
// type Props = {};
export default function LoginScreen() {
  const [email, setEmail] = useState(__DEV__ ? 'testuser1@email.com' : '');
  const [password, setPassword] = useState(__DEV__ ? 'testpassword' : '');

  const { login, createAccount } = useSessionApi();

  return (
    <View style={styles.container}>
      <Text>Login</Text>
      <TextInput
        style={styles.input}
        onChangeText={setEmail}
        value={email}
        placeholder="Email"
        placeholderTextColor="black"
        autoCapitalize="none"
        autoCorrect={false}
        autoComplete="username"
        textContentType="username"
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
        autoComplete="password"
        textContentType="password"
      />
      <Button
        label="Login"
        color="theme"
        onPress={() =>
          login({ email: email.toLowerCase(), password: password })
        }
      >
        <ButtonText>Login</ButtonText>
      </Button>
      <Button
        label="Register"
        color="secondary"
        onPress={() =>
          createAccount({ email: email.toLowerCase(), password: password })
        }
      >
        <ButtonText>Register</ButtonText>
      </Button>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
  input: {
    height: 40,
    width: 250,
    borderWidth: 1,
    paddingHorizontal: 10,
    color: 'black',
  },
});
