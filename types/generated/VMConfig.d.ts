import type { ErrorOptions } from './ErrorOptions.js';
import type { RuntimeConfig } from './RuntimeConfig.js';
import type { SecurityConfig } from './SecurityConfig.js';
export type VMConfig = {
    caps: Array<number>;
    errorOptions: ErrorOptions | null;
    runtimeConfig: RuntimeConfig | null;
    securityConfig: SecurityConfig | null;
};
