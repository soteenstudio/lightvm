/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

const colors = {
  red: '\x1b[31;1m',
  yellow: '\x1b[33m',
  cyan: '\x1b[36m',
  dark_gray: '\x1b[2;37m',
  reset: '\x1b[0m',
  bold: '\x1b[1m',
}
export class VMSystemError extends Error {
  public readonly code: string = 'LVM500';
  public readonly ip: number = 0;
  constructor(public readonly detail: string, public hintDetails: string[]) {
    super("");
    this.name = "";
    this.stack = "";
  }
  
  public print(explain: boolean, hint: boolean) {
    const { red, yellow, cyan, dark_gray, reset, bold } = colors;
    const formattedMessage = `${red}Error[LVM500]${reset}: ${this.detail}\n${hint ? ` ${reset}${cyan}│   ${dark_gray}` : `     ${dark_gray}`}at instruction pointer: 0\n${hint ? ` ${reset}${cyan}│   ${dark_gray}` : `     ${dark_gray}`}error type: SystemError\n${hint ? ` ${reset}${cyan}│  ${dark_gray}` : `    ${dark_gray}`}\n ${hint ? `${reset}${cyan}└─ hint: ${dark_gray}${explain ? `\n${this.hintDetails[0]}` : this.hintDetails[1]}\n\n` : ""}`;
    console.error(formattedMessage);
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, VMSystemError);
    }
  }
}
