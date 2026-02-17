#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { cp, mkdir, readdir } from "node:fs/promises";
import { isAbsolute, join, resolve } from "node:path";

export const DEFAULT_SNAPSHOT_DIR = "tinyverse_ui/tests/snapshots";
export const SNAPSHOT_REVIEW_ROOT = "docs/examples/tinyverse_ui/snapshot_reviews";

export type SnapshotReviewPaths = {
  timestamp: string;
  reviewRoot: string;
  reviewDir: string;
  beforeDir: string;
  afterDir: string;
};

function formatPart(value: number): string {
  return value.toString().padStart(2, "0");
}

export function formatTimestamp(date: Date = new Date()): string {
  const year = date.getFullYear();
  const month = formatPart(date.getMonth() + 1);
  const day = formatPart(date.getDate());
  const hour = formatPart(date.getHours());
  const minute = formatPart(date.getMinutes());
  const second = formatPart(date.getSeconds());
  return `${year}${month}${day}-${hour}${minute}${second}`;
}

export function resolvePathFromRoot(repoRoot: string, candidatePath: string): string {
  return isAbsolute(candidatePath) ? candidatePath : resolve(repoRoot, candidatePath);
}

export async function createSnapshotReviewPaths(
  repoRoot: string,
): Promise<SnapshotReviewPaths> {
  const reviewRoot = resolve(repoRoot, SNAPSHOT_REVIEW_ROOT);
  await mkdir(reviewRoot, { recursive: true });

  const baseTimestamp = formatTimestamp();
  let timestamp = baseTimestamp;
  let suffix = 1;
  let reviewDir = join(reviewRoot, timestamp);

  while (existsSync(reviewDir)) {
    timestamp = `${baseTimestamp}-${suffix}`;
    reviewDir = join(reviewRoot, timestamp);
    suffix += 1;
  }

  const beforeDir = join(reviewDir, "before");
  const afterDir = join(reviewDir, "after");
  await mkdir(beforeDir, { recursive: true });
  await mkdir(afterDir, { recursive: true });

  return {
    timestamp,
    reviewRoot,
    reviewDir,
    beforeDir,
    afterDir,
  };
}

export async function copySnapshotDirectory(sourceDir: string, destinationDir: string): Promise<void> {
  await cp(sourceDir, destinationDir, { recursive: true, force: true });
}

export async function listSnapshotReviewTimestamps(reviewRoot: string): Promise<string[]> {
  if (!existsSync(reviewRoot)) {
    return [];
  }

  const entries = await readdir(reviewRoot, { withFileTypes: true });
  return entries
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort((left, right) => right.localeCompare(left));
}
