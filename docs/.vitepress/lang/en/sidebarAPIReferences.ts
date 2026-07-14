export const sidebarAPIReferences = [
  {
    text: 'Method Functions',
    collapsed: false,
    items: [
      {
        text: 'Run Method',
        link: '/api-references/method-functions/run-method',
      },
      {
        text: 'Provide Method',
        link: '/api-references/method-functions/provide-method',
      },
      {
        text: 'Inspect Method',
        link: '/api-references/method-functions/inspect-method',
      },
      {
        text: 'Halt Method',
        link: '/api-references/method-functions/halt-method',
      },
      { text: 'On Method', link: '/api-references/method-functions/on-method' },
      {
        text: 'Export Method',
        link: '/api-references/method-functions/export-method',
      },
      {
        text: 'Tools Method',
        collapsed: true,
        items: [
          {
            text: 'Optimize Bytecode Method',
            link: '/api-references/method-functions/tools-method/optimize-bytecode-method',
          },
          {
            text: 'Stringify Method',
            link: '/api-references/method-functions/tools-method/stringify-method',
          },
          {
            text: 'Parse Method',
            link: '/api-references/method-functions/tools-method/parse-method',
          },
          {
            text: 'Parse Array Method',
            link: '/api-references/method-functions/tools-method/parse-array-method',
          },
        ],
      },
    ],
  },
  { text: 'Primitive Types', link: '/api-references/primitive-types' },
  {
    text: 'Instructions Set',
    collapsed: false,
    items: [
      {
        text: 'Stack & Variable Management',
        link: '/api-references/instruction-set/stack-variable-management',
      },
      {
        text: 'Arithmetic & Logic',
        link: '/api-references/instruction-set/arithmetic-logic'
      },
      {
        text: 'Control Flow & Function',
        link: '/api-references/instruction-set/control-flow-function'
      },
      {
        text: 'Data Structures & Metadata',
        link: '/api-references/instruction-set/data-structures-metadata'
      }
    ],
  },
];
