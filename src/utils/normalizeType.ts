/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

export function normalizeType(typeName?: string): string {
  const aliasMap: Record<string, string> = {
    Any: "$Type.Anything",
    Anything: "$Type.Anything",
    Unt: "$Type.Unit",
    Unit: "$Type.Unit",
    Num: "$Type.Number",
    Number: "$Type.Number",
    Int: "$Type.Integer",
    Integer: "$Type.Integer",
    Flt: "$Type.Float",
    Float: "$Type.Float",
    Dbl: "$Type.Double",
    Double: "$Type.Double",
    Lng: "$Type.Long",
    Long: "$Type.Long",
    Str: "$Type.String",
    String: "$Type.String",
    Char: "$Type.Character",
    Character: "$Type.Character",
    Bool: "$Type.Boolean",
    Boolean: "$Type.Boolean",
    Obj: "$Type.Object",
    Object: "$Type.Object",
    Func: "$Type.Function",
    Function: "$Type.Function",
  };
  return aliasMap[typeName as string] || `$Type.${(typeName as string)?.replace("$Type.", "")}`;
}