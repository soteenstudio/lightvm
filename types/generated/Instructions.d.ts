import type { PrimitiveTypes } from './PrimitiveTypes.js';
import type { Value } from './Value.js';
export type Instructions = {
    "push_int16": number;
} | {
    "push_int32": number;
} | {
    "push_int64": number;
} | {
    "push_int128": number;
} | {
    "push_float16": number;
} | {
    "push_float32": number;
} | {
    "push_float64": number;
} | {
    "push_string": string;
} | {
    "push_array": any[];
} | {
    "push_object": Record<string, any>;
} | {
    "push_bool": boolean;
} | "push_null" | "push_undefined" | "push_na_n" | {
    "push": Value;
} | {
    "val": string;
} | {
    "val_idx": number;
} | {
    "set": string;
} | {
    "set_idx": number;
} | {
    "get": string;
} | {
    "get_idx": number;
} | {
    "add": PrimitiveTypes;
} | {
    "addv": PrimitiveTypes;
} | {
    "sub": PrimitiveTypes;
} | {
    "subv": PrimitiveTypes;
} | {
    "mul": PrimitiveTypes;
} | {
    "mulv": PrimitiveTypes;
} | {
    "div": PrimitiveTypes;
} | {
    "divv": PrimitiveTypes;
} | {
    "mod": PrimitiveTypes;
} | {
    "modv": PrimitiveTypes;
} | {
    "shl": PrimitiveTypes;
} | {
    "shlv": PrimitiveTypes;
} | {
    "shr": PrimitiveTypes;
} | {
    "shrv": PrimitiveTypes;
} | {
    "ror": PrimitiveTypes;
} | {
    "rorv": PrimitiveTypes;
} | {
    "rol": PrimitiveTypes;
} | {
    "rolv": PrimitiveTypes;
} | {
    "sin": PrimitiveTypes;
} | {
    "cos": PrimitiveTypes;
} | {
    "tan": PrimitiveTypes;
} | {
    "sinv": PrimitiveTypes;
} | {
    "cosv": PrimitiveTypes;
} | {
    "tanv": PrimitiveTypes;
} | {
    "asin": PrimitiveTypes;
} | {
    "acos": PrimitiveTypes;
} | {
    "atan": PrimitiveTypes;
} | {
    "atan2": PrimitiveTypes;
} | {
    "asinv": PrimitiveTypes;
} | {
    "acosv": PrimitiveTypes;
} | {
    "atanv": PrimitiveTypes;
} | {
    "atan2v": PrimitiveTypes;
} | {
    "sinh": PrimitiveTypes;
} | {
    "cosh": PrimitiveTypes;
} | {
    "tanh": PrimitiveTypes;
} | {
    "sinhv": PrimitiveTypes;
} | {
    "coshv": PrimitiveTypes;
} | {
    "tanhv": PrimitiveTypes;
} | {
    "asinh": PrimitiveTypes;
} | {
    "acosh": PrimitiveTypes;
} | {
    "atanh": PrimitiveTypes;
} | {
    "asinhv": PrimitiveTypes;
} | {
    "acoshv": PrimitiveTypes;
} | {
    "atanhv": PrimitiveTypes;
} | {
    "sqrt": PrimitiveTypes;
} | {
    "sqrtv": PrimitiveTypes;
} | {
    "cbrt": PrimitiveTypes;
} | {
    "cbrtv": PrimitiveTypes;
} | {
    "neg": PrimitiveTypes;
} | {
    "negv": PrimitiveTypes;
} | {
    "ln": PrimitiveTypes;
} | {
    "lnv": PrimitiveTypes;
} | {
    "exp": PrimitiveTypes;
} | {
    "expv": PrimitiveTypes;
} | {
    "log2": PrimitiveTypes;
} | {
    "log2v": PrimitiveTypes;
} | {
    "log10": PrimitiveTypes;
} | {
    "log10v": PrimitiveTypes;
} | {
    "pow": PrimitiveTypes;
} | {
    "powi": PrimitiveTypes;
} | {
    "powf": PrimitiveTypes;
} | {
    "powv": PrimitiveTypes;
} | {
    "powiv": PrimitiveTypes;
} | {
    "powfv": PrimitiveTypes;
} | {
    "gt": PrimitiveTypes;
} | {
    "lt": PrimitiveTypes;
} | {
    "ge": PrimitiveTypes;
} | {
    "le": PrimitiveTypes;
} | {
    "eq": PrimitiveTypes;
} | {
    "neq": PrimitiveTypes;
} | {
    "dot": PrimitiveTypes;
} | {
    "cross": PrimitiveTypes;
} | {
    "normalize": PrimitiveTypes;
} | "and" | "or" | "xor" | "not" | "print" | "println" | "stdout" | "stdoutln" | "stdin" | "clear_screen" | {
    "if_false": number;
} | {
    "jump": number;
} | {
    "inc": [string, PrimitiveTypes];
} | {
    "inc_idx": [number, PrimitiveTypes];
} | {
    "dec": [string, PrimitiveTypes];
} | {
    "dec_idx": [number, PrimitiveTypes];
} | {
    "call": [string, PrimitiveTypes];
} | {
    "func": [string, number, number, number, string[]];
} | "stop" | "return" | {
    "break": number;
} | {
    "access": string;
} | "access_index" | "to_string" | "to_short" | "to_integer" | "to_long" | "to_octa" | "to_half" | "to_float" | "to_double" | {
    "make_obj": number;
} | {
    "make_array": number;
} | "type_of" | "inspect_obj" | "inspect_arr" | "length" | "concat" | "dup" | "swap" | {
    "set_prop": string;
} | {
    "import": [string, number];
} | {
    "export": string;
} | {
    "instantiate": [string, number];
} | "nop" | "truncate" | "shrink";
