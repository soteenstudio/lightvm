import type { ErrorOptions } from "./ErrorOptions";
import type { RuntimeConfig } from "./RuntimeConfig";
export type VMConfig = {
    caps: Array<number>;
    errorOptions: ErrorOptions | null;
    runtimeConfig: RuntimeConfig | null;
};
