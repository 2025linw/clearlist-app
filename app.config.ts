import { ConfigContext, ExpoConfig } from 'expo/config';

export default ({ config }: ConfigContext): ExpoConfig => ({
  name: 'Clear List',
  slug: 'clearlist-app',
  version: '0.0.1',
  orientation: 'portrait',
  icon: './assets/images/icon.png',
  scheme: 'clearlist',
  userInterfaceStyle: 'automatic',
  owner: '2025linw',
  ios: {
    supportsTablet: true,
    bundleIdentifier: 'com.saphy.clearlist',
    infoPlist: {
      ITSAppUsesNonExemptEncryption: false,
    },
  },
  android: {
    adaptiveIcon: {
      backgroundColor: '#E6F4FE',
      foregroundImage: './assets/images/android-icon-foreground.png',
      backgroundImage: './assets/images/android-icon-background.png',
      monochromeImage: './assets/images/android-icon-monochrome.png',
    },
    predictiveBackGestureEnabled: false,
    package: 'com.saphy.clearlist',
  },
  web: {
    output: 'static',
    favicon: './assets/images/favicon.png',
  },
  plugins: [
    'expo-router',
    [
      'expo-splash-screen',
      {
        image: './assets/images/splash-icon.png',
        imageWidth: 200,
        resizeMode: 'contain',
        backgroundColor: '#ffffff',
        dark: {
          backgroundColor: '#000000',
        },
      },
    ],
    'expo-secure-store',
    [
      'expo-font',
      {
        fonts: ['./assets/fonts/Inter.otf'], //
        android: {
          fonts: [
            {
              fontFamily: 'Inter',
              fontDefinitions: [
                {
                  path: './assets/fonts/Inter-Italic.otf',
                  weight: 500,
                  style: 'italic',
                },
                {
                  path: './assets/fonts/Inter-Bold.otf',
                  weight: 700,
                },
                {
                  path: './assets/fonts/Inter-BoldItalic.otf',
                  weight: 700,
                  style: 'italic',
                },
              ],
            },
          ],
        },
        ios: {
          fonts: ['./assets/'],
        },
      },
    ],
    'expo-image',
    'expo-web-browser',
  ],
  experiments: {
    typedRoutes: true,
    reactCompiler: true,
  },
  extra: {
    router: {},
    eas: {
      projectId: '06a24e08-53fe-4606-af15-b25c353ad340',
    },
  },
});
