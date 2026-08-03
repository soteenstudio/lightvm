export type SecurityConfig = {
    maxIo: number | null;
    maxImport: number | null;
    maxAlloc: number | null;
    maxCall: number | null;
    maxJump: number | null;
    maxTicks: number | null;
    maxStackSize: number | null;
    allowedImports: Array<string> | null;
    unsafeMode: boolean | null;
};
