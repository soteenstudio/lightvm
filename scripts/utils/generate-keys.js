/**
 * Copyright 2026 SoTeen Studio
 * 
 * Key Generator Utility: Ed25519 Signing Setup
 */

import { generateKeyPairSync } from 'crypto';
import os from 'os';

const args = process.argv.slice(2);
const isSilent = args.includes('--silent');

const s = { 
  reset: '\x1b[0m', 
  bold: '\x1b[1m', 
  dim: '\x1b[2m', 
  cyan: '\x1b[36m', 
  green: '\x1b[32m', 
  yellow: '\x1b[33m', 
  red: '\x1b[31m',
  magenta: '\x1b[35m'
};

const logger = {
  info: (msg) => !isSilent && console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) => !isSilent && console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  step: (msg) => !isSilent && console.log(`${s.bold}${s.cyan}ℹ${s.reset} ${msg}`),
  highlight: (msg) => !isSilent && console.log(`${s.bold}${s.magenta}🔑${s.reset} ${s.bold}${msg}${s.reset}`),
  error: (msg, detail) => console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

try {
  logger.info('Generating Ed25519 key pair for binary signatures...');

  const { publicKey, privateKey } = generateKeyPairSync('ed25519', {
    publicKeyEncoding: { type: 'spki', format: 'jwk' },
    privateKeyEncoding: { type: 'pkcs8', format: 'jwk' },
  });

  const pubKeyBuffer = Buffer.from(publicKey.x, 'base64url');
  const pubKeyArrayString = JSON.stringify(Array.from(pubKeyBuffer));
  const privateKeyString = JSON.stringify(privateKey, null, 2);

  logger.success('Key pair generated successfully!');

  console.log(`\n${s.dim}------------------------------------------------------------------------${s.reset}`);
  logger.highlight('COPY INI KE PUBLIC_KEY_BYTES DI loadNapi.ts:');
  console.log(`${s.yellow}${pubKeyArrayString}${s.reset}`);
  console.log(`${s.dim}------------------------------------------------------------------------${s.reset}\n`);

  console.log(`${s.dim}------------------------------------------------------------------------${s.reset}`);
  logger.highlight('COPY INI KE GITHUB SECRETS (SIGNING_PRIVATE_KEY):');
  console.log(`${s.cyan}${privateKeyString}${s.reset}`);
  console.log(`${s.dim}------------------------------------------------------------------------${s.reset}\n`);

  logger.step('Setup complete! Keep your private key secure.');
} catch (err) {
  logger.error('Key generation failed:', err.message);
  process.exit(1);
}
