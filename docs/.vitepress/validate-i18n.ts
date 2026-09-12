import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import exclusions from './i18n-allowlist.json' with { type: 'json' };

const docsDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
);
const sourceDir = path.join(docsDir, 'en');
const targetDir = path.join(docsDir, 'id');
const excluded = new Set(exclusions);

function markdownFiles(directory: string, base = directory): string[] {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolutePath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      return markdownFiles(absolutePath, base);
    }
    return entry.name.endsWith('.md')
      ? [path.relative(base, absolutePath).split(path.sep).join('/')]
      : [];
  });
}

function collect(content: string) {
  const fencePattern = /^(```|~~~)[^\n]*\n[\s\S]*?^\1\s*$/gm;
  const fencedBlocks = [...content.matchAll(fencePattern)].map(
    ([block]) => block,
  );
  const withoutFences = content.replace(fencePattern, '');
  return {
    fencedBlocks,
    includePaths: [...content.matchAll(/^\s*<<<\s+([^\s{[]+)/gm)].map(
      ([, includePath]) => includePath,
    ),
    inlineCode: [...withoutFences.matchAll(/(?<!`)`([^`\n]+)`(?!`)/g)].map(
      ([, identifier]) => identifier,
    ),
    errorCodes: [...content.matchAll(/\bLVM\d{3}\b/g)].map(([code]) => code),
    headingLevels: [...content.matchAll(/^(#{1,6})\s+/gm)].map(
      ([, hashes]) => hashes.length,
    ),
  };
}

function same(left: unknown[], right: unknown[]) {
  return JSON.stringify(left) === JSON.stringify(right);
}

export function validateI18n() {
  const errors: string[] = [];
  for (const relativePath of markdownFiles(sourceDir)) {
    if (excluded.has(relativePath)) {
      continue;
    }
    const sourcePath = path.join(sourceDir, relativePath);
    const targetPath = path.join(targetDir, relativePath);
    if (!fs.existsSync(targetPath)) {
      errors.push(`missing counterpart: ${sourcePath} -> ${targetPath}`);
      continue;
    }
    const source = collect(fs.readFileSync(sourcePath, 'utf8'));
    const target = collect(fs.readFileSync(targetPath, 'utf8'));
    for (const [label, sourceValues, targetValues] of [
      ['fenced code blocks', source.fencedBlocks, target.fencedBlocks],
      ['code-group include paths', source.includePaths, target.includePaths],
      ['inline code identifiers', source.inlineCode, target.inlineCode],
      ['LVM error codes', source.errorCodes, target.errorCodes],
      ['Markdown heading levels', source.headingLevels, target.headingLevels],
    ] as const) {
      if (!same(sourceValues, targetValues)) {
        errors.push(`${label} mismatch: ${sourcePath} -> ${targetPath}`);
      }
    }
  }
  if (errors.length > 0) {
    throw new Error(
      `Documentation localization validation failed:\n${errors.join('\n')}`,
    );
  }
}
