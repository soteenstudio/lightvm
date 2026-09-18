import { sidebarConfigurationReference } from './sidebarConfigurationReference.js';
import { sidebarMethodFunctions } from './sidebarMethodFunctions.js';
import { sidebarInstructionSet } from './sidebarInstructionSet.js';
import { sidebarErrorCodes } from './sidebarErrorCodes.js';

export const sidebarAPIReferences = [
  sidebarConfigurationReference,
  sidebarMethodFunctions,
  { text: 'Tipe Primitif', link: '/id/api-reference/primitive-types' },
  { text: 'Kapabilitas', link: '/id/api-reference/capabilities' },
  { text: 'Batas Waktu Eksekusi', link: '/id/api-reference/time-budget' },
  sidebarInstructionSet,
  sidebarErrorCodes,
];
