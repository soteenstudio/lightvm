/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import http from 'http';
import fs from 'fs';
import { extname, resolve, relative } from 'path';

const PORT = 3000;
const ROOT_DIR = resolve('.');

const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.wasm': 'application/wasm',
};

const server = http.createServer((req, res) => {
  const requestPath = req.url === '/' ? '/.testings/browser.html' : (req.url || '/');
  const pathname = requestPath.split('?')[0].split('#')[0];
  const decodedPath = decodeURIComponent(pathname);
  const filePath = resolve(ROOT_DIR, `.${decodedPath}`);
  const pathRelativeToRoot = relative(ROOT_DIR, filePath);

  if (pathRelativeToRoot.startsWith('..') || pathRelativeToRoot === '') {
    res.writeHead(403, { 'Content-Type': 'text/plain' });
    res.end('403 Forbidden');
    return;
  }

  const ext = extname(filePath);
  let contentType = MIME_TYPES[ext] || 'application/octet-stream';

  fs.stat(filePath, (err, stats) => {
    if (err || !stats.isFile()) {
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('404 Not Found');
      return;
    }

    res.writeHead(200, {
      'Content-Type': contentType,
      'Content-Length': stats.size,
      'Cache-Control': 'no-cache',
    });

    const stream = fs.createReadStream(filePath);

    stream.pipe(res);

    stream.on('error', (streamErr) => {
      console.error('Stream Error:', streamErr);
      if (!res.headersSent) {
        res.writeHead(500);
        res.end('Internal Server Error');
      }
    });
  });
});

server.listen(PORT, () => {
  console.log(
    `\x1b[36m⠋ Server is running on:\x1b[0m \x1b[1m\x1b[32mhttp://localhost:${PORT}\x1b[0m`,
  );
  console.log(`\x1b[2mPress Ctrl+C to turn off the server\x1b[0m`);
});
