/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

export class VMSystemError extends Error {
  public readonly code: string = 'LVM500';
  public readonly ip: number = 0;

  constructor(detail: string) {
    const red = '\x1b[31;1m';
    const yellow = '\x1b[33m';
    const cyan = '\x1b[36m';
    const reset = '\x1b[0m';
    const bold = '\x1b[1m';

    const prefix = `${bold}[LightVM]${reset}`;
    const formattedMessage = `${prefix} ${red}Runtime Error LVM500${reset}: ${detail}\n${yellow}Location: ${cyan}instruction_pointer: 0${reset}`;

    super(formattedMessage);

    this.name = 'SystemError';

    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, VMSystemError);
    }
  }
}
