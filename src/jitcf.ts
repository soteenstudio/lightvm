/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import { loadWasm } from "./utils/index.js";

export class JITCF {
  private wasm: any;
  private hitCounter: Record<number, number> = {};
  private HOT_THRESHOLD = 0;
  constructor(hit: Record<number, number>, hot: number) {
    this.hitCounter = hit;
    this.HOT_THRESHOLD = hot;
  }
  add(a: number, b: number): void {
    this.wasm = loadWasm("add.wasm");
  }
}