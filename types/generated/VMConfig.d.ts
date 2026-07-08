import type { ErrorOptions } from './ErrorOptions.js';
import type { RuntimeConfig } from './RuntimeConfig.js';
export type VMConfig = {
    caps: Array<number>;
    errorOptions: ErrorOptions | null;
    runtimeConfig: RuntimeConfig | null;
};
