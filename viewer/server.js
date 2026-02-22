const express = require('express');
const { WebSocketServer } = require('ws');
const chokidar = require('chokidar');
const { spawn } = require('child_process');
const path = require('path');
const http = require('http');

const watchDir = process.argv[2];
if (!watchDir) {
  console.error('Usage: node server.js <directory-to-watch>');
  process.exit(1);
}

const resolvedWatchDir = path.resolve(watchDir);
const draveurBin = path.resolve(__dirname, '../target/release/draveur');

let latestGraph = null, lastElapsed = null;

const app = express();
const server = http.createServer(app);
const wss = new WebSocketServer({ server });

app.get('/', (_, res) => res.sendFile(path.join(__dirname, 'index.html')));

function runDraveur() {
  return new Promise((resolve, reject) => {
    const proc = spawn(draveurBin, [], { cwd: resolvedWatchDir });
    let stdout = '', stderr = '';
    proc.stdout.on('data', d => stdout += d);
    proc.stderr.on('data', d => stderr += d);
    proc.on('close', code => {
      if (code !== 0) return reject(new Error(`draveur exited with code ${code}: ${stderr}`));
      try {
        const graphs = [];
        let buffer = '', depth = 0;
        for (const line of stdout.trim().split('\n')) {
          buffer += line;
          depth += (line.match(/\[/g) || []).length - (line.match(/\]/g) || []).length;
          if (depth === 0 && buffer.trim()) { graphs.push(JSON.parse(buffer)); buffer = ''; }
        }
        resolve(graphs.flat());
      } catch (e) {
        reject(new Error(`Failed to parse JSON: ${e.message}`));
      }
    });
    proc.on('error', reject);
  });
}

const broadcast = data => {
  const msg = JSON.stringify(data);
  wss.clients.forEach(c => c.readyState === 1 && c.send(msg));
};

async function updateGraph() {
  try {
    console.log('Running draveur...');
    const start = Date.now();
    latestGraph = await runDraveur();
    lastElapsed = Date.now() - start;
    console.log(`Graph updated: ${latestGraph.length} nodes in ${lastElapsed}ms`);
    broadcast({ type: 'graph', data: latestGraph, elapsed: lastElapsed });
  } catch (err) {
    console.error('Error:', err.message);
    broadcast({ type: 'error', message: err.message });
  }
}

wss.on('connection', client => {
  if (latestGraph) client.send(JSON.stringify({ type: 'graph', data: latestGraph, elapsed: lastElapsed }));
  client.on('message', async msg => {
    try { if (JSON.parse(msg).type === 'reload') await updateGraph(); }
    catch (e) { console.error('Invalid message:', e.message); }
  });
});

chokidar.watch(path.join(resolvedWatchDir, '**/*.py'), { ignoreInitial: true })
  .on('change', f => { console.log(`File changed: ${f}`); updateGraph(); })
  .on('add', f => { console.log(`File added: ${f}`); updateGraph(); });

updateGraph();

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => console.log(`Server running at http://localhost:${PORT}\nWatching: ${resolvedWatchDir}`));
