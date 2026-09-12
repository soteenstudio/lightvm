import { sidebarMethodFunctions } from './sidebarMethodFunctions.js';
import { sidebarInstructionSet } from './sidebarInstructionSet.js';
import { sidebarErrorCodes } from './sidebarErrorCodes.js';

export const sidebarAPIReferences = [
  sidebarMethodFunctions,
  { text: 'Primitive Types', link: '/api-reference/primitive-types' },
  { text: 'Capabilities', link: '/api-reference/capabilities' },
  { text: 'Time Budget', link: '/api-reference/time-budget' },
  sidebarInstructionSet,
  sidebarErrorCodes,
];
