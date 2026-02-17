#!/usr/bin/env bun

import { runDockerCompose } from "./helpers/docker_compose.sh.ts";

const args = Bun.argv.slice(2);
const commandArgs = args[0] === "run" ? args.slice(1) : args;
const normalizedCommandArgs = commandArgs[0] === "--" ? commandArgs.slice(1) : commandArgs;
const command = normalizedCommandArgs.length > 0
  ? normalizedCommandArgs
  : ["bun", "scripts/ci.sh.ts", "docker"];

await runDockerCompose(["run", "--rm", "ci", ...command]);
