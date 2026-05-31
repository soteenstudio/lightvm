import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const targetDir = path.join(__dirname, '../ts', 'benchmarks');
const files = fs.readdirSync(targetDir).filter(file => file.endsWith('.ts'));
files.forEach(file => {
  console.log(`Running: ${file}...`);
  try {
    execSync(`npx tsx "${path.join(targetDir, file)}"`, { stdio: 'inherit' });
  } catch (err) {
    console.error(`Error while running ${file}`);
  }
});
