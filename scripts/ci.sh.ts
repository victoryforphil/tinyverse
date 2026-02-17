#!/usr/bin/env bun

import { mkdir, copyFile } from "node:fs/promises";
import { basename, join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommand } from "./helpers/run_command.sh.ts";

type CiMode =
  | "full"
  | "docker"
  | "build"
  | "check"
  | "test"
  | "smoke"
  | "docker-smoke"
  | "snapshots"
  | "binaries"
  | "cargo-publish-prep";

const repoRoot = findRepoRoot(import.meta.dir);
const mode = (Bun.argv[2] as CiMode | undefined) ?? "full";

if (mode === "build") {
  await runCommand(["bun", "scripts/build.sh.ts"], { cwd: repoRoot });
  process.exit(0);
}

if (mode === "check") {
  await runCommand(["bun", "scripts/check.sh.ts"], { cwd: repoRoot });
  process.exit(0);
}

if (mode === "test") {
  await runCommand(["bun", "scripts/test.sh.ts"], { cwd: repoRoot });
  process.exit(0);
}

if (mode === "smoke") {
  await runCommand(["bun", "scripts/smoke_cli.sh.ts"], { cwd: repoRoot });
  process.exit(0);
}

if (mode === "docker-smoke") {
  await runCommand(["bun", "scripts/docker_build.sh.ts", "ci"], { cwd: repoRoot });
  await runCommand(["bun", "scripts/docker_ci.sh.ts", "run", "--", "bun", "scripts/smoke_cli.sh.ts"], {
    cwd: repoRoot,
  });
  process.exit(0);
}

if (mode === "snapshots") {
  await runCommand(["cargo", "test", "-p", "tinyverse_ui"], { cwd: repoRoot });
  await runCommand(["bun", "scripts/render_snapshot_review.sh.ts", "20260216-225543"], {
    cwd: repoRoot,
  });
  process.exit(0);
}

if (mode === "binaries") {
  const artifactDir = process.env.CI_ARTIFACT_DIR;
  await runCommand(["cargo", "build", "--release", "-p", "tinyverse_cli"], { cwd: repoRoot });

  if (!artifactDir) {
    process.exit(0);
  }

  const exeName = process.platform === "win32" ? "tinyverse_cli.exe" : "tinyverse_cli";
  const sourcePath = join(repoRoot, "target", "release", exeName);
  await mkdir(artifactDir, { recursive: true });
  await copyFile(sourcePath, join(artifactDir, basename(sourcePath)));
  process.exit(0);
}

if (mode === "cargo-publish-prep") {
  const enabled = process.env.CI_ENABLE_CARGO_PUBLISH === "true";
  if (!enabled) {
    console.log("Cargo publish is disabled. Set CI_ENABLE_CARGO_PUBLISH=true to run cargo publish --dry-run.");
    process.exit(0);
  }

  await runCommand(["cargo", "publish", "-p", "tinyverse_cli", "--dry-run"], {
    cwd: repoRoot,
  });
  process.exit(0);
}

if (mode === "docker") {
  await runCommand(["cargo", "build", "-p", "tinyverse_cli"], { cwd: repoRoot });
  await runCommand(["bun", "scripts/helpers/test_rust.sh.ts", "-p", "tinyverse_cli"], {
    cwd: repoRoot,
  });
  await runCommand(["bun", "scripts/smoke_cli.sh.ts"], { cwd: repoRoot });
  process.exit(0);
}

if (mode !== "full") {
  throw new Error(`Unknown CI mode: ${mode}`);
}

await runCommand(["bun", "scripts/ci.sh.ts", "build"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "check"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "test"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "docker-smoke"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "snapshots"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "binaries"], { cwd: repoRoot });
await runCommand(["bun", "scripts/ci.sh.ts", "cargo-publish-prep"], { cwd: repoRoot });
