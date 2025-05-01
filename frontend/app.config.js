import pkg from './package.json';

module.exports = function (_config) {
  const VERSION = pkg.version;

  const PLATFORM = process.env.EAS_BUILD_PLATFORM;

  const IS_TESTFLIGHT = process.env.EXPO_PUBLIC_ENV === 'testflight';
  const IS_PRODUCTION = process.env.EXPO_PUBLIC_ENV === 'production';
  const IS_DEV = !IS_TESTFLIGHT || !IS_PRODUCTION;

  // const ASSOCIATED_DOMAINS = [];

  const UPDATES_CHANNEL = IS_TESTFLIGHT?
    'testflight' : IS_PRODUCTION?
      'production' : undefined;
  const UPDATES_ENABLED = !!UPDATES_CHANNEL;

  return {
    expo: {
      name: 'Clear List',
      slug: 'clearlist',
      scheme: 'clearlist',
      owner: '2025linw',
      version: VERSION,
      runtimeVersion: {
        policy: 'appVersion',
      },
      newArchEnabled: true,
      experiments: {
        typedRoutes: true,
      },

      icon: './assets/app-icons/apple-touch-icon.png',
      userInterfaceStyle: 'automatic',
      primaryColor: '#209cee',

      ios: {
        bundleIdentifier: '2025linw.clearlist.app',
        supportsTablet: true,
        infoPlist: {
          CFBundleSpokenName: 'Clear List',
        },
        // associatedDomains: ASSOCIATED_DOMAINS,
        // entitlements: {
        //   'com.apple.developer.kernel.increased-memory-limit': true,
        //   'com.apple.developer.kernel.extended-virtual-addressing': true,
        //   'com.apple.security.application-groups': 'group.app.clearlist',
        // },
        // privacyManifests: {
          // TODO: add any for calendar access or reminder access, etc.
        // }
      },
      android: {
        package: '2025linw.clearlist.app',
        adaptiveIcon: {
          foregroundImage: './assets/app-icons/android-icon-192x192.png',
          backgroundColor: '#ffffff',
        },
        // googleServicesFile: './google-services.json',
        // intentFilters: []
      },
      web: {
        bundler: 'metro',
        output: 'static',
        favicon: './assets/favicon.ico',
      },

      plugins: [
        'expo-router',
        [
          'expo-splash-screen',
          {
            'image': './assets/app-icons/android-icon-192x192.png',
            'imageWidth': 200,
            'resizeMode': 'contain',
            'backgroundColor': '#25292e',
          }
        ]
      ].filter(Boolean),
    }
  }
}
