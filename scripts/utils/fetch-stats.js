import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const packages = [
  'lightvm',
  "@lightvm/core-linux-x64",
  "@lightvm/core-linux-arm64",
  "@lightvm/core-linux-ia32",
  "@lightvm/core-win32-x64",
  "@lightvm/core-win32-ia32",
  "@lightvm/core-darwin-x64",
  "@lightvm/core-android-arm64",
  "@lightvm/core-android-arm",
  "@lightvm/core-linux-musl-x64",
  "@lightvm/core-linux-musl-arm64",
  "@lightvm/core-linux-musl-ia32",
  "@lightvm/core-freebsd-x64",
  "@lightvm/core-browser-wasm"
];

const rustCrates = ['lightvm'];

async function getDownloads(range) {
  const rangePath =
    range === 'td'
      ? `point/2026-05-01:${new Date().toISOString().split('T')[0]}`
      : `point/${range}`;

  const npmRequests = packages.map(pkg =>
    fetch(`https://api.npmjs.org/downloads/${rangePath}/${encodeURIComponent(pkg)}`)
      .then(r => r.ok ? r.json() : { downloads: 0 })
      .then(d => d.downloads || 0)
      .catch(() => 0)
  );

  const crateRequests = rustCrates.map(crate => {
    if (range !== 'td') return Promise.resolve(0);

    return fetch(`https://crates.io/api/v1/crates/${crate}`)
      .then(r => r.ok ? r.json() : null)
      .then(d => d?.crate?.downloads || 0)
      .catch(() => 0);
  });

  const downloads = await Promise.all([...npmRequests, ...crateRequests]);

  return downloads.reduce((a, b) => a + b, 0);
}

async function run() {
  console.log('🔄 Syncing stats...');

  try {
    const weekly = await getDownloads('last-week');
    const allTime = await getDownloads('td');

    const data = {
      weekly: (weekly / 1000).toFixed(1),
      allTime: (allTime / 1000).toFixed(1),
      updatedAt: new Date().toISOString()
    };

    const outputDir = path.join(__dirname, '../../docs/data');
    fs.mkdirSync(outputDir, { recursive: true });

    fs.writeFileSync(
      path.join(outputDir, 'stats.json'),
      JSON.stringify(data, null, 2)
    );

    console.log('✅ Stats updated');
  } catch (err) {
    console.error(err);

    fs.writeFileSync(
      path.join(__dirname, '../../docs/data/stats.json'),
      JSON.stringify({
        weekly: '0.0',
        allTime: '0.0'
      })
    );
  }
}

run();