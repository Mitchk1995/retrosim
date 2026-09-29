import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap(entry => {
    if (['.git', 'node_modules', 'target'].includes(entry.name)) return [];
    const full = path.join(dir, entry.name);
    return entry.isDirectory() ? walk(full) : [full];
  });
}
const documents = walk(root).filter(file => file.endsWith('.md'));
for (const file of documents) {
  const content = fs.readFileSync(file, 'utf8');
  for (const match of content.matchAll(/!?\[[^\]]*\]\(([^\s)]+)(?:\s+"[^"]*")?\)/g)) {
    const link = match[1];
    if (/^(?:https?:|mailto:|#)/.test(link)) continue;
    const target = path.resolve(path.dirname(file), decodeURIComponent(link.split('#')[0]));
    assert.ok(target === root || target.startsWith(root + path.sep), 'Link leaves project: ' + link);
    assert.ok(fs.existsSync(target), 'Missing link target in ' + path.relative(root, file) + ': ' + link);
  }
}
const manifest = JSON.parse(fs.readFileSync(path.join(root, 'prototypes/combat-lab/demo-manifest.json'), 'utf8'));
assert.equal(manifest.schemaVersion, 1);
const ids = manifest.modes.map(mode => mode.id);
assert.equal(new Set(ids).size, ids.length, 'Combat mode IDs must be unique');
for (const id of manifest.phaseOne) assert.ok(ids.includes(id), 'Unknown first-phase combat mode: ' + id);
assert.equal(manifest.phaseOne.length, 2, 'First comparison should stay focused on two modes');
assert.equal(new Set(manifest.phaseOne).size, 2, 'First comparison must use two distinct modes');
const reference = JSON.parse(fs.readFileSync(path.join(root, 'assets/reference/reference.json'), 'utf8'));
const png = fs.readFileSync(path.join(root, 'assets/reference', reference.file));
assert.equal(png.subarray(0, 8).toString('hex'), '89504e470d0a1a0a', 'Reference is not a PNG');
assert.equal(createHash('sha256').update(png).digest('hex'), reference.sha256, 'Approved reference image changed; review and update provenance deliberately');
assert.equal(png.readUInt32BE(16), reference.width);
assert.equal(png.readUInt32BE(20), reference.height);
console.log('Foundation OK: ' + documents.length + ' documents, ' + ids.length + ' combat options, and approved reference verified.');
