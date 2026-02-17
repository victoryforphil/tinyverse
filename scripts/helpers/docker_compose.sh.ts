#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./run_root.sh.ts";
import { runCommand } from "./run_command.sh.ts";

export const repoRoot = findRepoRoot(import.meta.dir);
export const composeFilePath = join(repoRoot, "docker", "compose.devcontainers.yml");

export async function runDockerCompose(args: string[]): Promise<void> {
  await runCommand(["docker", "compose", "-f", composeFilePath, ...args], {
    cwd: repoRoot,
  });
}
