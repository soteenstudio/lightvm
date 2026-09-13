import tsParser from '@typescript-eslint/parser';
import headers from 'eslint-plugin-headers';

const year = '2025-2026';

const customNoCommentRule = {
  meta: {
    type: 'layout',
    docs: { description: 'Remove prohibited comments' },
    fixable: 'code',
  },
  create(context) {
    const sourceCode = context.sourceCode;
    return {
      Program(node) {
        const comments = sourceCode.getAllComments();
        comments.forEach((comment) => {
          if (!(comment.type === 'Block' && comment.value.startsWith('*'))) {
            context.report({
              node: comment,
              message: 'Comments are prohibited!',
              fix(fixer) {
                return fixer.remove(comment);
              },
            });
          }
        });
      },
    };
  },
};

export default [
  {
    files: ['ts/src/**/*.{js,ts}', 'ts/tests/**/*.{js,ts}', 'scripts/**/*.{js,ts}'],
    ignores: ['ts/src/generated/**/*.ts'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    plugins: {
      cleaner: {
        rules: {
          'no-comment': customNoCommentRule,
        },
      },
      headers,
    },
    rules: {
      'cleaner/no-comment': 'error',
      'no-multiple-empty-lines': [
        'error',
        {
          max: 1,
          maxEOF: 1,
          maxBOF: 0,
        },
      ],
      'headers/header-format': [
        'error',
        {
          source: 'string',
          content: `Copyright ${year} SoTeen Studio\n\nLicensed under the Apache License, Version 2.0 (the "License");\nyou may not use this file except in compliance with the License.\nYou may obtain a copy of the License at\n\n    http://www.apache.org/licenses/LICENSE-2.0`,
        },
      ],
    },
  },
  {
    files: ['ts/src/generated/**/*.ts'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    plugins: {
      cleaner: {
        rules: {
          'no-comment': customNoCommentRule,
        },
      },
      headers,
    },
    rules: {
      'cleaner/no-comment': 'off',
      'headers/header-format': 'off',
      'no-multiple-empty-lines': [
        'error',
        {
          max: 1,
          maxEOF: 1,
          maxBOF: 0,
        },
      ],
    },
  },
];
