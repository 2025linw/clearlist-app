const config = {
  plugins: ['@trivago/prettier-plugin-sort-imports'],

  // General
  tabWidth: 2,
  useTabs: false,

  semi: true,
  trailingComma: 'all',
  singleQuote: true,

  arrowParens: 'avoid',

  bracketSpacing: true,
  objectWrap: 'collapse',
  quoteProps: 'as-needed',
  bracketSameLine: false,

  // React JSX Elements
  jsxSingleQuote: false,

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
