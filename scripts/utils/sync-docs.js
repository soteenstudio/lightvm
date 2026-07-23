/**
 * Copyright 2026 Clay.
 *
 * Script for syncing i18n documentation content.
 */

import { cp, readdir, stat } from 'node:fs/promises';
import { join } from 'node:path';
import { existsSync, mkdirSync } from 'node:fs';

const SOURCE_LANG = 'en';
const TARGET_LANGS = ['id']; // Add more languages here
const baseDir = 'docs';

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
  info: (msg) => console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) => console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  warn: (msg) => console.log(`${s.bold}${s.yellow}⚠${s.reset} ${msg}`),
  error: (msg, detail) => console.error(`${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
  step: (msg) => console.log(`${s.bold}${s.dim}→ ${msg}${s.reset}`),
};

async function syncFiles(src, dest, langCode) {
  if (!existsSync(dest)) mkdirSync(dest, { recursive: true });
  
  const entries = await readdir(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = join(src, entry.name);
    const destPath = join(dest, entry.name);

    if (entry.isDirectory()) {
      await syncFiles(srcPath, destPath, langCode);
    } else {
      try {
        const srcStat = await stat(srcPath);
        
        try {
          const destStat = await stat(destPath);

          // Check if file in 'en' is newer than the target
          if (srcStat.mtimeMs > destStat.mtimeMs + 1000) {
            logger.warn(`${s.bold}[${langCode.toUpperCase()}]${s.reset} Changes detected in '${entry.name}'. Please update manually.`);
          } else {
            // Keep "Synced" logs dim to reduce visual clutter
            console.log(`${s.dim}  ✔ [${langCode.toUpperCase()}] ${entry.name} - Synced${s.reset}`);
          }
        } catch {
          // File doesn't exist in target, auto-copy
          await cp(srcPath, destPath);
          logger.success(`${s.bold}[${langCode.toUpperCase()}]${s.reset} New file synced: ${entry.name}`);
        }
      } catch (err) {
        logger.error(`Failed to process ${entry.name} for ${langCode}:`, err.message);
      }
    }
  }
}

async function startSync() {
  logger.info(`${s.bold}Starting synchronization from '${SOURCE_LANG}' to: ${TARGET_LANGS.join(', ')}${s.reset}\n`);
  
  for (const lang of TARGET_LANGS) {
    const src = join(baseDir, SOURCE_LANG);
    const dest = join(baseDir, lang);
    await syncFiles(src, dest, lang);
  }
  
  console.log(`\n${s.bold}${s.green}✔ All languages processed successfully!${s.reset}`);
}

startSync();
