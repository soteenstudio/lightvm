export const CapabilityValue = {
  Control: 0,
  Observe: 1,
  Debug: 2,
  Unsafe: 3,
} as const;

export enum Capability {
  Control = 0,
  Observe = 1,
  Debug = 2,
  Unsafe = 3,
}
