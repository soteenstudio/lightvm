/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

export function humanizeVersion(version: string): string {
  return version;
}

export function formatInfoVM(info: any): string {
  const CYAN = '\x1b[36m';
  const BOLD = '\x1b[1m';
  const RESET = '\x1b[0m';
  const DARK_GRAY = '\x1b[90m';
  const YELLOW = '\x1b[33m';

  const formattedVersion = humanizeVersion(info.version);

  let result = `${CYAN}${BOLD}${info.name}${RESET} v${formattedVersion}\n`;
  result += `${DARK_GRAY}modules:${RESET}\n`;
  result += `  ├─ carzy    v${info.modules.carzy}\n`;
  result += `  ├─ gazle    v${info.modules.gazle}\n`;
  result += `  ├─ itme     v${info.modules.itme}\n`;
  result += `  ├─ krates   v${info.modules.krates}\n`;
  result += `  ├─ torja    v${info.modules.torja}\n`;
  result += `  ├─ bluel    v${info.modules.bluel}\n`;
  result += `  ├─ dying    v${info.modules.dying}\n`;
  result += `  └─ vmerror  v${info.modules.vmerror}`;

  if (info.latest_version && info.latest_version !== info.version) {
    const formattedLatest = humanizeVersion(info.latest_version);
    result += `\n\n${RESET}${DARK_GRAY}new update available:\n`;
    result += `  ${RESET}${YELLOW}* ${RESET}v${formattedLatest}`;
  }

  return result;
}
