#!/usr/bin/env bun

import { findRepoRoot } from "./run_root.sh.ts";
import { runCommand } from "./run_command.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);

export async function runMoon(args: string[]): Promise<void> {
  await runCommand(["moon", ...args], { cwd: repoRoot });
}
