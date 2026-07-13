export const sidebarAPIReferences = [
  {
    text: 'Fungsi Metode',
    collapsed: false,
    items: [
      { text: 'Metode Run', link: '/id/api-references/method-functions/run-method' },
      { text: 'Metode Provide', link: '/id/api-references/method-functions/provide-method' },
      { text: 'Metode Inspect', link: '/id/api-references/method-functions/inspect-method' },
      { text: 'Metode Halt', link: '/id/api-references/method-functions/halt-method' },
      { text: 'Metode On', link: '/id/api-references/method-functions/on-method' },
      { text: 'Metode Export', link: '/id/api-references/method-functions/export-method' },
      {
        text: 'Metode Tools',
        collapsed: true,
        items: [
          { text: 'Metode Optimize Bytecode', link: '/id/api-references/method-functions/tools-method/optimize-bytecode-method' },
          { text: 'Metode Stringify', link: '/id/api-references/method-functions/tools-method/stringify-method' },
          { text: 'Metode Parse', link: '/id/api-references/method-functions/tools-method/parse-method' },
          { text: 'Metode Parse Array', link: '/id/api-references/method-functions/tools-method/parse-array-method' }
        ]
      },
    ]
  },
  { text: 'Tipe Primitif', link: '/id/id/api-references/primitive-types' },
  {
    text: 'Set Instruksi',
    collapsed: false,
    items: [
      { text: 'Manajemen Stack & Variabel', link: '/id/api-references/instructions-set/stack-variable-management' }
    ]
  }
];