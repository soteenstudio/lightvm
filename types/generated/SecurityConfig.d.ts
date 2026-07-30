export type SecurityConfig = {
    max_io: number | null;
    max_import: number | null;
    max_alloc: number | null;
    max_call: number | null;
    max_jump: number | null;
    allowed_imports: Array<string> | null;
    unsafe_mode: boolean | null;
};
