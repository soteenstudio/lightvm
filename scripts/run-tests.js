import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Helper biar dapet __dirname di ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function run() {
  try {
    // 1. Build project (silent)
    console.log('Building project...');
    execSync('npm run build', { stdio: 'ignore' });
    console.log('Build success!');

    // 2. Run tests (inherit logs)
    console.log('Running tests...');
    try {
      execSync('npx unitry ./ts/tests', { stdio: 'inherit' });
    } catch (err) {
      console.log('Tests finished with some failures.');
    }

    // 3. Delete dist folder
    const distPath = path.resolve(__dirname, '../ts/dist');
    if (fs.existsSync(distPath)) {
      fs.rmSync(distPath, { recursive: true, force: true });
      console.log('Cleanup complete: ./ts/dist deleted.');
    }
  } catch (error) {
    console.error('Error during execution:', error.message);
    process.exit(1);
  }
}

run();
