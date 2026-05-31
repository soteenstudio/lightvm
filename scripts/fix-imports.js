import fs from 'fs';
import path from 'path';
const sourceDir = path.join(process.cwd(), 'ts', 'src', 'generated');
const targetDir = path.join(process.cwd(), 'ts', 'src', 'generated');
if (!fs.existsSync(targetDir)) {
  fs.mkdirSync(targetDir, { recursive: true });
}
function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');
  content = content.replace(/from\s+["'](\.\.?\/.*?)["']/g, (match, p1) => {
    if (p1.endsWith('.js') || p1.endsWith('.ts')) return match;
    return `from '${p1}.js'`;
  });
  const fileName = path.basename(filePath);
  const targetPath = path.join(targetDir, fileName);
  fs.writeFileSync(targetPath, content);
  console.log(`Successfully processed: ${fileName}`);
}
fs.readdirSync(sourceDir).forEach(file => {
  if (file.endsWith('.ts')) {
    processFile(path.join(sourceDir, file));
  }
});
