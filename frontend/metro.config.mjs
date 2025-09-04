import  { fileURLToPath } from 'url';
import { dirname } from 'path';

// Learn more https://docs.expo.io/guides/customizing-metro
import { getDefaultConfig } from 'expo/metro-config';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('expo/metro-config').MetroConfig} */
const config = getDefaultConfig(__dirname);

// export default config;
