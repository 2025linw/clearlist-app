import { View, Text } from "react-native";


enum ScreenState {
  S_LoginOrCreateAccount,
  S_Login,
  S_CreateAccount
}

export function LoggedOut({ onDismiss }: {onDismiss?: () => void}) {
  return (
    <View>
      <Text>
        This should be the view when logged out!
      </Text>
    </View>
  )
}
