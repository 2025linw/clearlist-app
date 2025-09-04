import { Platform } from 'react-native';

import { isAndroid, isIOS, isNative, isWeb } from '#/util/detectPlatform';

export function web(value: any): any | undefined {
  if (isWeb) {
    return value;
  }

  return undefined;
}

export function ios(value: any): any | undefined {
  if (isIOS) {
    return value;
  }

  return undefined;
}

export function android(value: any): any | undefined {
  if (isAndroid) {
    return value;
  }

  return undefined;
}

export function native(value: any): any | undefined {
  if (isNative) {
    return value;
  }

  return undefined;
}

export const platform = Platform.select;
