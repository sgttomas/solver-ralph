// ESLint configuration for SOLVER-Ralph UI
// Per D-03 CI requirements
module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: './tsconfig.json',
  },
  plugins: ['@typescript-eslint'],
  ignorePatterns: ['dist', 'node_modules', '*.config.js', '*.config.ts', '*.cjs'],
  rules: {
    // Allow unused vars prefixed with underscore
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
  },
};
