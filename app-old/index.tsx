import { Redirect } from 'expo-router';
import { Platform, View, ScrollView, StyleSheet, Text } from 'react-native';

export default function Index() {
  return (
    <Redirect href='/login' />
  );
}
