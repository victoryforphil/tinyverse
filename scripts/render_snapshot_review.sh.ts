#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { mkdir, readdir } from "node:fs/promises";
import { join, relative, resolve } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import {
  SNAPSHOT_REVIEW_ROOT,
  listSnapshotReviewTimestamps,
  resolvePathFromRoot,
} from "./helpers/insta_snapshots.sh.ts";

type SnapshotStatus = "changed" | "unchanged" | "added" | "removed";

type SnapshotDiff = {
  relativePath: string;
  status: SnapshotStatus;
  beforeBody: string;
  afterBody: string;
};

function parseSnapshotBody(contents: string): string {
  const lines = contents.split("\n");
  if (lines[0] !== "---") {
    return contents;
  }

  const secondFenceIndex = lines.indexOf("---", 1);
  if (secondFenceIndex < 0) {
    return contents;
  }

  return lines.slice(secondFenceIndex + 1).join("\n");
}

function stripAnsi(value: string): string {
  return value.replace(/\u001b\[[0-9;]*m/g, "");
}

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

async function readFileIfExists(path: string): Promise<string | null> {
  if (!existsSync(path)) {
    return null;
  }
  return Bun.file(path).text();
}

async function collectSnapshotFiles(rootDir: string): Promise<string[]> {
  if (!existsSync(rootDir)) {
    return [];
  }

  const files: string[] = [];
  const pendingDirs = [rootDir];

  while (pendingDirs.length > 0) {
    const currentDir = pendingDirs.pop();
    if (!currentDir) {
      break;
    }

    const entries = await readdir(currentDir, { withFileTypes: true });
    for (const entry of entries) {
      const absolutePath = join(currentDir, entry.name);
      if (entry.isDirectory()) {
        pendingDirs.push(absolutePath);
        continue;
      }

      if (entry.isFile() && entry.name.endsWith(".snap")) {
        files.push(relative(rootDir, absolutePath));
      }
    }
  }

  return files.sort((left, right) => left.localeCompare(right));
}

function summarizeStatus(diffs: SnapshotDiff[]): Record<SnapshotStatus, number> {
  return diffs.reduce(
    (counts, diff) => {
      counts[diff.status] += 1;
      return counts;
    },
    { changed: 0, unchanged: 0, added: 0, removed: 0 } satisfies Record<SnapshotStatus, number>,
  );
}

function renderHtml(reviewName: string, diffs: SnapshotDiff[]): string {
  const totals = summarizeStatus(diffs);
  const sections = diffs
    .map((diff) => {
      const beforeLabel = diff.status === "added" ? "Before (missing)" : "Before";
      const afterLabel = diff.status === "removed" ? "After (missing)" : "After";
      const beforeText = diff.beforeBody.length > 0 ? diff.beforeBody : "(missing)";
      const afterText = diff.afterBody.length > 0 ? diff.afterBody : "(missing)";

      return `
<section class="snapshot ${diff.status}">
  <h2><code>${escapeHtml(diff.relativePath)}</code> <span class="badge">${diff.status}</span></h2>
  <div class="columns">
    <div>
      <h3>${beforeLabel}</h3>
      <pre>${escapeHtml(stripAnsi(beforeText))}</pre>
    </div>
    <div>
      <h3>${afterLabel}</h3>
      <pre>${escapeHtml(stripAnsi(afterText))}</pre>
    </div>
  </div>
</section>`;
    })
    .join("\n");

  return `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Snapshot Review ${escapeHtml(reviewName)}</title>
  <style>
    :root {
      color-scheme: light;
      --bg: #f4f6f8;
      --panel: #ffffff;
      --ink: #20262d;
      --subtle: #5f6b76;
      --line: #d4dde6;
      --changed: #ffd67d;
      --unchanged: #d8e7ff;
      --added: #bfe5bf;
      --removed: #efb4b4;
    }
    body {
      margin: 0;
      padding: 24px;
      background: radial-gradient(circle at 0 0, #ffffff 0, var(--bg) 50%);
      color: var(--ink);
      font-family: "Iosevka", "JetBrains Mono", "SFMono-Regular", Menlo, monospace;
    }
    h1, h2, h3 { margin: 0; }
    h1 { font-size: 20px; }
    .meta {
      margin-top: 10px;
      color: var(--subtle);
      font-size: 13px;
    }
    .snapshot {
      margin-top: 24px;
      padding: 16px;
      border: 1px solid var(--line);
      border-left-width: 6px;
      border-radius: 10px;
      background: var(--panel);
    }
    .snapshot.changed { border-left-color: var(--changed); }
    .snapshot.unchanged { border-left-color: var(--unchanged); }
    .snapshot.added { border-left-color: var(--added); }
    .snapshot.removed { border-left-color: var(--removed); }
    .badge {
      margin-left: 8px;
      border-radius: 99px;
      border: 1px solid var(--line);
      padding: 2px 8px;
      font-size: 12px;
      color: var(--subtle);
      text-transform: uppercase;
      letter-spacing: 0.04em;
    }
    .columns {
      margin-top: 12px;
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 16px;
    }
    h3 {
      margin-bottom: 6px;
      color: var(--subtle);
      font-size: 13px;
      text-transform: uppercase;
      letter-spacing: 0.06em;
    }
    pre {
      margin: 0;
      padding: 12px;
      min-height: 80px;
      max-height: 420px;
      overflow: auto;
      border: 1px solid var(--line);
      border-radius: 8px;
      background: #fbfcfe;
      line-height: 1.4;
      font-size: 12px;
      white-space: pre;
    }
    @media (max-width: 1100px) {
      .columns {
        grid-template-columns: 1fr;
      }
    }
  </style>
</head>
<body>
  <h1>Snapshot Review ${escapeHtml(reviewName)}</h1>
  <p class="meta">changed: ${totals.changed} | unchanged: ${totals.unchanged} | added: ${totals.added} | removed: ${totals.removed}</p>
  ${sections}
</body>
</html>
`;
}

function renderMarkdownIndex(reviewName: string, diffs: SnapshotDiff[]): string {
  const totals = summarizeStatus(diffs);
  const rows = diffs
    .map((diff) => `| \`${diff.relativePath}\` | ${diff.status} |`)
    .join("\n");

  return `# Snapshot Review ${reviewName}

- changed: ${totals.changed}
- unchanged: ${totals.unchanged}
- added: ${totals.added}
- removed: ${totals.removed}

Open \`index.html\` in VS Code or a browser for side-by-side rendering.

| Snapshot | Status |
|---|---|
${rows}
`;
}

async function renderRootIndex(reviewRoot: string, timestamps: string[]): Promise<void> {
  const lines = timestamps.map((timestamp) => `- [${timestamp}](./${timestamp}/index.md) | [html](./${timestamp}/index.html)`);
  const markdown = `# Snapshot Reviews

Generated visual review artifacts for \`cargo insta review\` runs.

${lines.join("\n")}
`;

  await Bun.write(join(reviewRoot, "index.md"), markdown);
}

const repoRoot = findRepoRoot(import.meta.dir);
const reviewRoot = resolve(repoRoot, SNAPSHOT_REVIEW_ROOT);
await mkdir(reviewRoot, { recursive: true });

const argSelector = Bun.argv[2];
const knownTimestamps = await listSnapshotReviewTimestamps(reviewRoot);
const selectedTimestamp = argSelector ?? knownTimestamps[0];

if (!selectedTimestamp) {
  throw new Error(`No snapshot review folders found in ${reviewRoot}`);
}

const reviewDir = argSelector && existsSync(resolve(argSelector))
  ? resolve(argSelector)
  : resolvePathFromRoot(repoRoot, join(SNAPSHOT_REVIEW_ROOT, selectedTimestamp));

const beforeDir = join(reviewDir, "before");
const afterDir = join(reviewDir, "after");

if (!existsSync(beforeDir) || !existsSync(afterDir)) {
  throw new Error(`Expected before/after folders in ${reviewDir}`);
}

const beforeFiles = await collectSnapshotFiles(beforeDir);
const afterFiles = await collectSnapshotFiles(afterDir);
const allFiles = [...new Set([...beforeFiles, ...afterFiles])].sort((left, right) =>
  left.localeCompare(right),
);

const diffs: SnapshotDiff[] = [];

for (const snapshotFile of allFiles) {
  const beforeText = await readFileIfExists(join(beforeDir, snapshotFile));
  const afterText = await readFileIfExists(join(afterDir, snapshotFile));
  const beforeBody = beforeText ? parseSnapshotBody(beforeText) : "";
  const afterBody = afterText ? parseSnapshotBody(afterText) : "";

  const status: SnapshotStatus = beforeText === null
    ? "added"
    : afterText === null
      ? "removed"
      : beforeBody === afterBody
        ? "unchanged"
        : "changed";

  diffs.push({
    relativePath: snapshotFile,
    status,
    beforeBody,
    afterBody,
  });
}

const reviewName = relative(reviewRoot, reviewDir) || selectedTimestamp;
await Bun.write(join(reviewDir, "index.html"), renderHtml(reviewName, diffs));
await Bun.write(join(reviewDir, "index.md"), renderMarkdownIndex(reviewName, diffs));
await renderRootIndex(reviewRoot, await listSnapshotReviewTimestamps(reviewRoot));

console.log(`Rendered review page at ${relative(repoRoot, join(reviewDir, "index.html"))}`);
