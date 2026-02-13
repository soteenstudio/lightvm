/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

export type Instruction =
  | ["push", number | string | boolean | null | undefined]
  | ["val", string]
  | ["set", string]
  | ["get", string]
  | ["add", string]
  | ["sub", string]
  | ["mul"]
  | ["div"]
  | ["mod"]
  | ["gt"]
  | ["lt"]
  | ["ge"]
  | ["le"]
  | ["eq"]
  | ["neq"]
  | ["and"]
  | ["or"]
  | ["print"]
  | ["println"]
  | ["error"]
  | ["errorln"]
  | ["if_false", number]
  | ["jump", number]
  | ["inc", string]
  | ["dec", string]
  | ["call", string, number]
  | ["func", string, number, number, number, ...(string[] | [])]
  | ["stop"]
  | ["return"]
  | ["break"]
  | ["access", string]
  | ["access_index"]
  | ["string"]
  | ["number"]
  | ["make_obj", number]
  | ["make_array", number]
  | ["typeof"]
  | ["length"]
  | ["concat"]
  | ["dup"]
  | ["set_prop", string]
  | ["import", string, string]
  | ["export", string]
  | ["instantiate", string, number]