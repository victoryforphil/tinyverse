#!/usr/bin/env bun

import { runMoon } from "./helpers/moon.sh.ts";

const args = Bun.argv.slice(2);
const taskArgs = args.length > 0 ? args : ["run", "tinyverse_cli:build", "--watch"];

await runMoon(taskArgs);
