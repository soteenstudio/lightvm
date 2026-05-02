import { createRequire } from 'module';
import { join } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';
const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let cachedNative = null;
export function loadNapi() {
  if (cachedNative) return cachedNative;
  try {
    const localPath = join(__dirname, '../binaries/lightvm.node');
    cachedNative = require(localPath);
    return cachedNative;
  } catch (e) {}
  const { platform, arch } = process;
  let packageName = '';
  if (platform === 'linux' && arch === 'x64') {
    packageName = '@lightvm/core-linux-x64';
  } else if (platform === 'win32' && arch === 'x64') {
    packageName = '@lightvm/core-win32-x64';
  } else if (platform === 'darwin' && arch === 'x64') {
    packageName = '@lightvm/core-darwin-x64';
  } else if (platform === 'android' && arch === 'arm64') {
    packageName = '@lightvm/core-android-arm64';
  } else {
    throw new Error(`LightVM: Platform ${platform} ${arch} gak didukung, Clay.`);
  }
  try {
    cachedNative = require(packageName);
    return cachedNative;
  } catch (err) {
    throw new Error(
      `LightVM: Gagal load biner untuk ${packageName}. Pastiin koneksi aman pas install.`
    );
  }
}
