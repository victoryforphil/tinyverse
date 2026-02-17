#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import {
  DEFAULT_SNAPSHOT_DIR,
  copySnapshotDirectory,
  createSnapshotReviewPaths,
  resolvePathFromRoot,
} from "./helpers/insta_snapshots.sh.ts";

type ReviewOptions = {
  snapshotDir: string;
  dryRun: boolean;
  cargoArgs: string[];
};

function parseOptions(args: string[]): ReviewOptions {
  const cargoArgs: string[] = [];
  let snapshotDir = DEFAULT_SNAPSHOT_DIR;
  let dryRun = false;

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];

    if (arg === "--snapshot-dir") {
      const value = args[index + 1];
      if (!value) {
        throw new Error("Missing value for --snapshot-dir");
      }
      snapshotDir = value;
      index += 1;
      continue;
    }

    if (arg === "--dry-run") {
      dryRun = true;
      continue;
    }

    cargoArgs.push(arg);
  }

  return {
    snapshotDir,
    dryRun,
    cargoArgs,
  };
}

const repoRoot = findRepoRoot(import.meta.dir);
const { snapshotDir, dryRun, cargoArgs } = parseOptions(Bun.argv.slice(2));
const resolvedSnapshotDir = resolvePathFromRoot(repoRoot, snapshotDir);

if (!existsSync(resolvedSnapshotDir)) {
  throw new Error(`Snapshot directory does not exist: ${resolvedSnapshotDir}`);
}

const review = await createSnapshotReviewPaths(repoRoot);

console.log(`Snapshot review: ${review.timestamp}`);
console.log(`Copying pre-review snapshots from ${resolvedSnapshotDir}`);
await copySnapshotDirectory(resolvedSnapshotDir, review.beforeDir);

let reviewExitCode = 0;

try {
  if (dryRun) {
    console.log("Dry run enabled. Skipping cargo insta review.");
  } else {
    const reviewCommand = ["cargo", "insta", "review", ...cargoArgs];
    console.log(`Running ${reviewCommand.join(" ")}`);

    const reviewProcess = Bun.spawn(reviewCommand, {
      cwd: repoRoot,
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });

    reviewExitCode = await reviewProcess.exited;
  }
} finally {
  console.log(`Copying post-review snapshots from ${resolvedSnapshotDir}`);
  await copySnapshotDirectory(resolvedSnapshotDir, review.afterDir);
}

const relativeReviewDir = review.reviewDir.replace(`${repoRoot}/`, "");
console.log(`Stored review snapshots in ${relativeReviewDir}`);
console.log(`Render with: bun scripts/render_snapshot_review.sh.ts ${review.timestamp}`);

if (reviewExitCode !== 0) {
  process.exit(reviewExitCode);
}
