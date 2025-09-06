export const presets = ['babel-preset-expo'];
export const plugins = [
  [
    'module-resolver',
    {
      alias: {
        '#': './src',
      },
    },
  ],
];
