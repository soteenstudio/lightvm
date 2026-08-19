import { sidebarMethodFunctions } from './sidebarMethodFunctions.js';
import { sidebarInstructionSet } from './sidebarInstructionSet.js';

export const sidebarAPIReferences = [
  sidebarMethodFunctions,
  { text: 'Tipe Primitif', link: '/id/api-reference/primitive-types' },
  { text: 'Kapabilitas', link: '/id/api-reference/capabilities' },
  { text: 'Batas Waktu Eksekusi', link: '/id/api-reference/time-budget' },
  sidebarInstructionSet,
];
