import { TextStyle } from 'react-native';

import { type FontScale, type FontFamily } from '#/types/font';

import { isAndroid, isWeb } from '#/util/detectPlatform';
import { Device, device } from '#/storage';

const WEB_FONT_FAMILIES = `system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"`;

const factor = 0.0625; // 1 - (15/16)
const fontScaleMultipliers: Record<Device['fontScale'], number> = {
  '-2': 1 - factor * 3,
  '-1': 1 - factor * 2,
  '0': 1 - factor * 1,
  '1': 1,
  '2': 1 + factor * 1,
};

export function computeFontScaleMultiplier(scale: Device['fontScale']): number {
  return fontScaleMultipliers[scale];
}

export function getFontScale(): FontScale {
  return device.get(['fontScale']) ?? '0';
}
export function setFontScale(fontScale: Device['fontScale']) {
  device.set(['fontScale'], fontScale);
}

export function getFontFamily(): FontFamily {
  return device.get(['fontFamily']) || 'theme';
}
export function setFontFamily(fontFamily: Device['fontFamily']) {
  device.set(['fontFamily'], fontFamily);
}

export function applyFonts(style: TextStyle, fontFamily: FontFamily) {
  if (fontFamily === 'theme') {
    if (isAndroid) {
      if (style.fontStyle === 'italic') {
        console.trace("unimplemented");
      }
    } else {
      if (style.fontStyle === 'italic') {
        console.trace("unimplemented");
      }

      // web fallback families
      if (isWeb) {
        style.fontFamily += `, ${WEB_FONT_FAMILIES}`;
      }
    }
    console.trace('unimplemented');
  } else {
    // web fallback families
    if (isWeb) {
      style.fontFamily = style.fontFamily || WEB_FONT_FAMILIES;
    }

    style.letterSpacing = 0.25;
  }
}
