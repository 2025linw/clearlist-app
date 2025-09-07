import babelParser from '@babel/eslint-parser';
import js from '@eslint/js';
import parser from '@typescript-eslint/parser';
import prettier from 'eslint-config-prettier';
import react from 'eslint-plugin-react';
import reactNative from 'eslint-plugin-react-native';
import unusedImports from 'eslint-plugin-unused-imports';
import { defineConfig } from 'eslint/config';
import ts from 'typescript-eslint';

export default defineConfig(
  js.configs.recommended,
  ts.configs.recommended,

  // React Configs
  { ...react.configs.flat.recommended },

  // Babel Config
  {
    files: ['./babel.config.js'],
    languageOptions: { parser: babelParser },
    rules: { 'no-undef': 'off' },
  },
  {
    files: ['**/*.{js,mjs,jsx,ts,mts,tsx}'],

    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      parser: parser,
    },
    settings: { react: { version: 'detect' } },

    plugins: { 'unused-imports': unusedImports, 'react-native': reactNative },

    rules: {
      // Javascript
      'prefer-const': 'warn',
      'no-empty-pattern': 'warn',

      // Typescript
      '@typescript-eslint/array-type': ['warn', { default: 'array-simple' }],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': 'warn',

      // React
      'react/react-in-jsx-scope': 'off',

      // React Native
      'react-native/no-inline-styles': 'warn',
    },
  },

  // Prettier - this must be last
  prettier,
);
