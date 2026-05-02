import fs from 'fs';
import {
  execSync
} from 'child_process';
import {
  join
} from 'path';
const args = process.argv.slice(2);
const isLocal = args.includes('--local');
const isDebug = args.includes('--debug');
const isProduction = args.includes('--production');
const isPublish = args.includes('--publish');
const isSilent = args.includes('--silent');
const STDIO_MODE = isSilent ? 'ignore' : 'inherit';
const PKG_PATH = join(process.cwd(), 'package.json');
const RUST_BINARY_NAME = 'liblightvm_rust.so';
const TARGET_NAME = 'lightvm.node';
const RUST_OUT_DIR = isDebug ? 'debug' : 'release';
const SOURCE_PATH = join(process.cwd(), 'rust', 'target', RUST_OUT_DIR, RUST_BINARY_NAME);
const DEST_DIR = join(process.cwd(), 'binaries');
const DEST_PATH = join(DEST_DIR, TARGET_NAME);
function togglePackageFiles(mode) {
  const pkg = JSON.parse(fs.readFileSync(PKG_PATH, 'utf-8'));
  if (!pkg.files) pkg.files = [];
  if (mode === 'local') {
    if (!pkg.files.includes('binaries')) {
      pkg.files.push('binaries');
    }
  } else {
    pkg.files = pkg.files.filter(file => file !== 'binaries');
  }
  fs.writeFileSync(PKG_PATH, JSON.stringify(pkg, null, 2));
}
try {
  console.log(`🏗️  Building Rust (${RUST_OUT_DIR})...`);
  const buildCmd = isDebug ? 'cargo build' : 'cargo build --release';
  execSync(buildCmd, {
    cwd: join(process.cwd(), 'rust'),
    stdio: STDIO_MODE
  });
  console.log('🏗️  Building JS...');
  execSync('npm run build', {
    stdio: STDIO_MODE
  });
  if (isLocal) {
    if (!fs.existsSync(DEST_DIR)) fs.mkdirSync(DEST_DIR);
    if (fs.existsSync(SOURCE_PATH)) {
      fs.copyFileSync(SOURCE_PATH, DEST_PATH);
      console.log(`✅ Copied binary to ${DEST_PATH}`);
    } else {
      throw new Error(`Binary not found at ${SOURCE_PATH}`);
    }
    togglePackageFiles('local');
    console.log('📦 Packing local tarball (with binaries)...');
    execSync('npm pack', {
      stdio: 'inherit'
    });
    togglePackageFiles('prod');
  } else if (isProduction) {
    togglePackageFiles('prod');
    if (fs.existsSync(DEST_PATH)) {
      fs.unlinkSync(DEST_PATH);
      console.log('🧹 Cleaned local binaries for production safety.');
    }
    if (isPublish) {
      console.log('🚀 Publishing to NPM Registry...');
      execSync('npm publish --access public', {
        stdio: 'inherit'
      });
    } else {
      console.log('📦 Packing production tarball (clean, no binaries)...');
      execSync('npm pack', {
        stdio: 'inherit'
      });
      console.log('✅ Production build complete.');
    }
  } else {
    console.log('💡 Usage: --local, --production, --production --publish, or add --silent');
  }
} catch (err) {
  togglePackageFiles('prod');
  console.error('❌ Build failed:', err.message);
  process.exit(1);
}