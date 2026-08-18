/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import fs from 'fs';
import path from 'path';

// Variabel khusus nampung list file rust yang mau di-generate
const TARGET_RUST_FILES = [
  'rust/src/types/capability.rs',
  'rust/src/types/vmevent.rs',
  'rust/src/types/target_arch.rs',
  'rust/src/types/file_type.rs',
  'rust/src/types/time_budget.rs',
];

const args = process.argv.slice(2);
const cliFiles = args.filter((arg) => !arg.startsWith('--'));
const isSilent = args.includes('--silent');

// Kalau di-pass lewat CLI, pakai CLI; kalau kosong, fallback ke variabel konstan di atas
const inputFiles = cliFiles.length > 0 ? cliFiles : TARGET_RUST_FILES;

const OUTPUT_DIR = path.join(process.cwd(), 'ts', 'src', 'generated');

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
};

const logger = {
  info: (msg) =>
    !isSilent && console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) =>
    !isSilent && console.log(`${s.bold}${s.green}✔${s.reset} ${msg}\n`),
  step: (msg) =>
    !isSilent && console.log(`${s.bold}${s.cyan}ℹ${s.reset} ${msg}`),
  cleanup: (msg) =>
    !isSilent && console.log(`${s.bold}${s.dim}🧹 ${msg}${s.reset}`),
  warn: (msg) =>
    !isSilent && console.log(`${s.bold}${s.yellow}⚠${s.reset} ${msg}`),
  error: (msg, detail) =>
    console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

if (inputFiles.length === 0) {
  logger.error(
    'No input files specified.',
    'Define files in TARGET_RUST_FILES or provide them as CLI arguments.',
  );
  console.log(
    `  ${s.dim}Usage:${s.reset} node generate-enums.mjs [file1.rs file2.rs...] ${s.dim}[--silent]${s.reset}\n`,
  );
  process.exit(1);
}

if (!fs.existsSync(OUTPUT_DIR)) {
  fs.mkdirSync(OUTPUT_DIR, { recursive: true });
}

try {
  logger.info(
    `${s.bold}Parsing Rust sources & generating enums${s.reset} ${s.dim}(ts/src/generated)...${s.reset}`,
  );

  let totalGenerated = 0;

  for (const filePath of inputFiles) {
    const resolvedPath = path.resolve(filePath);
    if (!fs.existsSync(resolvedPath)) {
      logger.warn(`File not found: ${resolvedPath}, skipping.`);
      continue;
    }

    const content = fs.readFileSync(resolvedPath, 'utf8');

    // Clean comments
    const cleanContent = content
      .replace(/\/\/.*/g, '')
      .replace(/\/\*[\s\S]*?\*\//g, '');

    const enumRegex = /(?:pub\s+)?enum\s+(\w+)\s*\{([^}]*)\}/g;
    let match;

    while ((match = enumRegex.exec(cleanContent)) !== null) {
      const enumName = match[1];
      const body = match[2];

      const rawVariants = body.split(',').map((v) => v.trim()).filter(Boolean);
      let currentValue = 0;
      const variants = [];

      for (const variant of rawVariants) {
        const cleanVariant = variant.replace(/#\[[\s\S]*?\]/g, '').trim();
        if (!cleanVariant) continue;

        const parts = cleanVariant.split('=').map((p) => p.trim());
        const name = parts[0];

        if (parts.length > 1) {
          currentValue = parseInt(parts[1], 10);
        }

        variants.push(`  ${name} = ${currentValue}`);
        currentValue++;
      }

      const tsCode = `export enum ${enumName} {\n${variants.join(',\n')},\n}\n`;
      const outputPath = path.join(OUTPUT_DIR, `${enumName}.ts`);

      fs.writeFileSync(outputPath, tsCode, 'utf8');
      logger.step(
        `${s.dim}Generated:${s.reset} ${s.bold}${enumName}.ts${s.reset}`,
      );
      totalGenerated++;
    }
  }

  if (totalGenerated === 0) {
    logger.warn('No valid Rust enums were found in the provided files.');
  }

  logger.success(
    `${s.bold}Successfully generated ${totalGenerated} TypeScript enum(s)!${s.reset}`,
  );
} catch (err) {
  logger.error('Enum generation failed:', err.message);
  process.exit(1);
}
