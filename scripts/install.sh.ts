#!/usr/bin/env bun

import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommandSteps } from "./helpers/run_steps.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);

await runCommandSteps([
  {
    name: "Install proto toolchain",
    command: "proto",
    args: ["install"],
    cwd: repoRoot,
  },
  {
    name: "Install workspace projects",
    command: "moon",
    args: [":install"],
    cwd: repoRoot,
  },
  {
    name: "Install cargo-nextest",
    command: "cargo",
    args: ["install", "cargo-nextest", "--locked"],
    cwd: repoRoot,
  },
  {
    name: "Install Diesel CLI (sqlite)",
    command: "cargo",
    args: [
      "install",
      "diesel_cli",
      "--no-default-features",
      "--features",
      "sqlite-bundled",
    ],
    cwd: repoRoot,
  },
]);
