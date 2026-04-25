import { expoClient } from '@better-auth/expo/client';
import { createAuthClient } from 'better-auth/react';

import * as SecureStore from 'expo-secure-store';

import { BASE_URL } from '@/constants';

export const authClient = createAuthClient({
  baseURL: BASE_URL,
  plugins: [
    expoClient({
      scheme: 'clearlist',
      storagePrefix: 'clearlist',
      storage: SecureStore,
    }),
  ],
  emailAndPassword: {
    enabled: true,
  },
});
