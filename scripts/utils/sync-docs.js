import { cp, readdir, stat, readFile } from 'node:fs/promises';
import { join } from 'node:path';

const sourceDir = 'docs/en';
const targetDir = 'docs/id';

async function syncFiles(src, dest) {
  // Pastikan folder dest ada
  const entries = await readdir(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = join(src, entry.name);
    const destPath = join(dest, entry.name);

    if (entry.isDirectory()) {
      await syncFiles(srcPath, destPath);
    } else {
      try {
        const destStat = await stat(destPath);
        const srcStat = await stat(srcPath);

        // Cek apakah ukuran file berubah (indikasi ada konten baru/tambahan)
        if (srcStat.size !== destStat.size) {
          console.log(`⚠️ PERHATIAN: Perubahan terdeteksi di '${entry.name}'.`);
          console.log(`   Size source: ${srcStat.size} bytes | Size target: ${destStat.size} bytes.`);
          console.log(`   Silakan cek manual, mungkin ada konten baru yang perlu di-translate.`);
        } else {
          console.log(`✅ ${entry.name} - Up to date.`);
        }
      } catch {
        // Kalau belum ada, langsung copy
        await cp(srcPath, destPath);
        console.log(`✨ Copy file baru: ${entry.name}`);
      }
    }
  }
}

async function startSync() {
  console.log('🚀 Sync check dimulai...');
  await syncFiles(sourceDir, targetDir);
  console.log('✅ Selesai!');
}

startSync();
