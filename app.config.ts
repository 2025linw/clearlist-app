import { ExpoConfig } from 'expo/config';
import 'ts-node/register';

import pkg from './package.json';

const config: ExpoConfig = {
  name: 'Clear List',
  slug: 'clear-list',

  version: pkg.version,

  newArchEnabled: true,

  icon: './assets/ios/AppIcon~ios-marketing.png',

  ios: {
    supportsTablet: true,
    bundleIdentifier: 'io.saphynet.todo',
    infoPlist: { ITSAppUsesNonExemptEncryption: false },
  },
  android: { package: 'io.saphynet.todo' },
  web: { favicon: './assets/web/favicon.ico' },

  plugins: ['expo-font'],

  extra: { eas: { projectId: '484daebd-45bc-432f-9e82-2821014b2ac3' } },
};

export default config;
