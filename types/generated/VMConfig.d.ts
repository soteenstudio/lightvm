export type VMConfig = {
    caps: Array<number>;
    nightly: boolean | null;
    backtrace: boolean | null;
    explain: boolean | null;
    hint: boolean | null;
};
