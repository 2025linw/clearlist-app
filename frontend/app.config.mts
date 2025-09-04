import { ExpoConfig } from 'expo/config';
import 'ts-node/register';

const config: ExpoConfig = {
  name: 'clear-list',
  slug: 'clear-list',
  version: '0.0.1',

  newArchEnabled: true,

  icon: './assets/ios/AppIcon~ios-marketing.png',

  ios: {
    bundleIdentifier: 'io.saphynet.todo',

    supportsTablet: true,

    infoPlist: { ITSAppUsesNonExemptEncryption: false },
  },
  android: {
    package: 'io.saphynet.todo',

    edgeToEdgeEnabled: true,
  },

  extra: { eas: { projectId: '484daebd-45bc-432f-9e82-2821014b2ac3' } },
};

export default config;
