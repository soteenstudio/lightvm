/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import { execSync } from 'child_process';
function isMusl() {
  try {
    const report = process.report.getReport();
    if (typeof report === 'object' && report.header && !report.header.glibcVersionRuntime) {
      return true;
    }
  } catch (e) {}
  try {
    const output = execSync('ldd --version', { stdio: ['pipe', 'pipe', 'ignore'] }).toString();
    return output.includes('musl');
  } catch (e) {
    return false;
  }
}
