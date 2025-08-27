const config = {
  plugins: ['@trivago/prettier-plugin-sort-imports'],

  tabWidth: 2,
  useTabs: false,

  semi: true,
  trailingComma: 'all',
  singleQuote: true,

  arrowParens: 'avoid',

  bracketSpacing: true,
  objectWrap: 'collapse',
  quoteProps: 'as-needed',

  // React JSX Elements
  jsxSingleQuote: false,

  bracketSameLine: false,

  // @trivago/prettier-plugin-organize-imports
  importOrder: [
    '^#/types/(.*)$',
    '^#/alf(.*)$',
    '^#/(state|storage|services)(.*)$',
    '^#/components/(.*)$',
    '^#/screens/(.*)$',
    '^#/(.*)$',
    '^[./]',
  ],
  importOrderSeparation: true,
};

export default config;
