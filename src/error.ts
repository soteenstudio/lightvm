/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import chalk from "chalk";

let filename: string = "";
export function setName(name: string) {
  filename = name;
}

function formatType(type: string) {
  if (type === "SyntaxError") {
    return chalk.redBright.bold(type);
  } else if (type === "TypeError") {
    return chalk.magentaBright.bold(type);
  } else if (type === "ReferenceError") {
    return chalk.yellowBright.bold(type);
  } else if (type === "RuntimeError") {
    return chalk.cyanBright.bold(type);
  } else if (type === "InternalError") {
    return chalk.blueBright.bold(type);
  } else {
    return chalk.bold(type);
  }
}

function highlight(message: string) {
  return message.replace(
    /\b(cannot\s+compare|cannot|name|expected|unexpected\s+token|unexpected|got|for|with)\s+(?:'?\$?Type\.?)?'?([a-zA-Z0-9_$]+)'?([.,:;!?])?/gi,
    (_, word, next, punc) => {
      return `${chalk.white(word)} ${chalk.blue(word.toLowerCase() === "name" ? `'${next}'` : next)}${punc || ""}`;
    }
  );
}

export class CustomError {
  public line: number;
  public column: number;

  constructor(
    name: string,
    message: string,
    line: number | null,
    column: number | null
  ) {
    const stackTrace = line !== null && column !== null ? true : false;
    console.log(`${formatType(name)}: ${highlight(message)}\n  at ${chalk.gray.underline(filename)}${stackTrace ? `\n  at ${chalk.gray(`line ${chalk.underline(line)}, column ${chalk.underline(column)}`)}` : ""}`);
    process.exit(0);
  }
}