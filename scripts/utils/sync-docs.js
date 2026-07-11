import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import pkg from 'diff-match-patch';

// Mengakses class melalui properti default
const { diff_match_patch } = pkg;
const dmp = new diff_match_patch();

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '../../');
const sourceDir = path.join(rootDir, 'docs');
const targetDir = path.join(rootDir, 'docs/id');
const IS_DRY_RUN = process.argv.includes('--dry-run');

// Helper: Normalize Line Endings
const normalize = (text) => text.replace(/\r\n/g, '\n');

function syncFile(srcPath, destPath) {
    const srcContent = normalize(fs.readFileSync(srcPath, 'utf-8'));
    
    // Jika file target belum ada, buat baru dari sumber
    if (!fs.existsSync(destPath)) {
        if (!IS_DRY_RUN) {
            fs.writeFileSync(destPath, srcContent);
        }
        console.log(`Created: ${path.relative(rootDir, destPath)}`);
        return;
    }

    const destContent = normalize(fs.readFileSync(destPath, 'utf-8'));

    // Kita butuh 'patch' dari perbedaan versi sumber sebelumnya ke versi sekarang
    // Karena kita tidak menyimpan history, kita akan menggunakan cara:
    // Menghitung diff antara (sumber asli) dan (sumber sekarang), 
    // lalu mengaplikasikan patch tersebut ke (file terjemahan).
    
    // PERINGATAN: Karena kita tidak punya file 'oldSrc', 
    // cara ini bekerja paling efektif jika kamu sinkronisasi konten 
    // secara teks, bukan per baris.
    
    const patches = dmp.patch_make(destContent, srcContent); 
    const [finalContent, results] = dmp.patch_apply(patches, destContent);

    if (!IS_DRY_RUN) {
        const tempPath = destPath + '.tmp';
        fs.writeFileSync(tempPath, finalContent);
        fs.renameSync(tempPath, destPath);
    }
    
    console.log(`${IS_DRY_RUN ? '[DRY RUN] ' : ''}Synced: ${path.relative(rootDir, destPath)}`);
}

function cleanupGhostFiles(src, dest) {
    if (!fs.existsSync(dest)) return;
    const destFiles = fs.readdirSync(dest);
    for (const file of destFiles) {
        const destPath = path.join(dest, file);
        const srcPath = path.join(src, file);
        
        if (!fs.existsSync(srcPath)) {
            if (IS_DRY_RUN) {
                console.log(`[DRY RUN] Would delete: ${path.relative(rootDir, destPath)}`);
            } else {
                fs.rmSync(destPath, { recursive: true, force: true });
                console.log(`Deleted Ghost: ${path.relative(rootDir, destPath)}`);
            }
        } else if (fs.lstatSync(destPath).isDirectory()) {
            cleanupGhostFiles(srcPath, destPath);
        }
    }
}

function syncFolder(src, dest) {
    if (!fs.existsSync(dest)) fs.mkdirSync(dest, { recursive: true });

    const files = fs.readdirSync(src);
    for (const file of files) {
        if (file === 'id' || file.startsWith('.')) continue;

        const srcPath = path.join(src, file);
        const destPath = path.join(dest, file);

        if (fs.lstatSync(srcPath).isDirectory()) {
            syncFolder(srcPath, destPath);
        } else if (file.endsWith('.md')) {
            syncFile(srcPath, destPath);
        }
    }
}

console.log(`🚀 Starting ${IS_DRY_RUN ? 'DRY RUN ' : ''}sync process...`);
syncFolder(sourceDir, targetDir);
cleanupGhostFiles(sourceDir, targetDir);
console.log('✅ Sync completed!');
