import { StyleSheet, Text, View } from 'react-native';

import { Shell } from '#/view/shell/index';

function App() {
  return (
    <Shell />
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});

export default App;
