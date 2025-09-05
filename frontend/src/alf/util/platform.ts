import { Platform } from 'react-native';

import { isAndroid, isIOS, isNative, isWeb } from '#/util/detectPlatform';

export function web(
  value: string | number | object,
): string | number | object | undefined {
  if (isWeb) {
    return value;
  }

  return undefined;
}

export function ios(
  value: string | number | object,
): string | number | object | undefined {
  if (isIOS) {
    return value;
  }

  return undefined;
}

export function android(
  value: string | number | object,
): string | number | object | undefined {
  if (isAndroid) {
    return value;
  }

  return undefined;
}

export function native(
  value: string | number | object,
): string | number | object | undefined {
  if (isNative) {
    return value;
  }

  return undefined;
}

export const platform = Platform.select;
