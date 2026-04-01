import { useState } from 'react';
import { Button, Text, TextInput, View } from 'react-native';

import { useSessionApi } from '@/context/auth-context';

export default function TaskItem() {
  const { login, createAccount } = useSessionApi();

  const [email, setEmail] = useState('will@email.com');
  const [password, setPassword] = useState('testpass');
  const [name, setName] = useState('');

  const [isFocused, setFocus] = useState(false);

  return (
    <View>
      <Text>Login Here</Text>

      <TextInput
        placeholder="Email"
        value={email}
        onChangeText={setEmail}
        autoCapitalize="none"
        autoComplete="email"
        autoCorrect={false}
      />
      <TextInput
        placeholder="Password"
        value={password}
        onChangeText={setPassword}
        autoCapitalize="none"
        autoComplete="new-password"
        autoCorrect={false}
        secureTextEntry={!isFocused}
        onFocus={() => setFocus(true)}
        onBlur={() => setFocus(false)}
      />
      <TextInput
        placeholder="Name"
        value={name}
        onChangeText={setName}
        autoCapitalize="none"
        autoComplete="name"
        autoCorrect={false}
        onSubmitEditing={() => login({ email, password })}
      />

      <Button
        title="Login"
        onPress={() => login({ email, password })}
      />
      <Button
        title="Signup"
        onPress={() => createAccount({ email, password, name })}
      />
    </View>
  );
}
