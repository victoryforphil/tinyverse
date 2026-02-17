#!/usr/bin/env bun

import { rm } from "node:fs/promises";
import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommandCapture } from "./helpers/run_command.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const smokeHome = join(repoRoot, ".tinyverse-smoke");

try {
  await rm(smokeHome, { recursive: true, force: true });

  await runSmokeStep(
    ["cargo", "run", "-p", "tinyverse_cli", "--", "--help"],
    (stdout) => stdout.includes("tinyverse"),
    "tinyverse --help includes command header",
  );

  await runSmokeStep(
    [
      "cargo",
      "run",
      "-p",
      "tinyverse_cli",
      "--",
      "--tinyverse-dir-home",
      smokeHome,
      "config",
      "print",
      "--output",
      "raw",
      "--format",
      "toml",
    ],
    (stdout) => stdout.includes("spawn") && stdout.includes("shell"),
    "config print returns expected toml keys",
  );

  console.log("Smoke // Result // PASS");
} finally {
  await rm(smokeHome, { recursive: true, force: true });
}

async function runSmokeStep(
  command: string[],
  assertOk: (stdout: string) => boolean,
  assertionName: string,
): Promise<void> {
  const result = await runCommandCapture(command, { cwd: repoRoot });

  if (result.stdout.trim().length > 0) {
    console.log(result.stdout.trimEnd());
  }

  if (result.stderr.trim().length > 0) {
    console.error(result.stderr.trimEnd());
  }

  if (result.exitCode !== 0) {
    throw new Error(`Smoke step failed (exit=${result.exitCode}): ${command.join(" ")}`);
  }

  if (!assertOk(result.stdout)) {
    throw new Error(`Smoke assertion failed: ${assertionName}`);
  }

  console.log(`Smoke // PASS // ${assertionName}`);
}
