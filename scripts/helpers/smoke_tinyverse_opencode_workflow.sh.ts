#!/usr/bin/env bun

import { findRepoRoot } from "./run_root.sh.ts";

type RunResult = {
  exitCode: number;
  stdout: string;
  stderr: string;
};

type StepStatus = "PASS" | "FAIL" | "INFO";

const DEFAULT_PROMPT = "use explorer sub-agent to list all the directories";
const OPEN_CODE_MARKERS = ["OpenCode", "ctrl+t", "agents", "commands"];

function divider(label: string): string {
  const body = ` ${label} `;
  const width = Math.max(20, body.length + 2);
  const pad = "-".repeat(width - body.length);
  return `+${body}${pad}+`;
}

function printStage(title: string): void {
  console.log("\n" + divider(`STAGE ${title}`));
}

function printStep(status: StepStatus, message: string): void {
  console.log(`[${status}] ${message}`);
}

async function runCommand(args: string[]): Promise<RunResult> {
  const process = Bun.spawn(args, {
    cwd: repoRoot,
    stdout: "pipe",
    stderr: "pipe",
  });

  const [exitCode, stdout, stderr] = await Promise.all([
    process.exited,
    process.stdout.text(),
    process.stderr.text(),
  ]);

  return { exitCode, stdout, stderr };
}

async function runTinyverse(
  tinyverseArgs: string[],
  options: { allowFailure?: boolean; label?: string } = {},
): Promise<RunResult> {
  const command = ["cargo", "run", "-p", "tinyverse_cli", "--", ...tinyverseArgs];
  const label = options.label ?? tinyverseArgs.join(" ");
  printStep("INFO", label);
  console.log(`$ ${command.join(" ")}`);
  const result = await runCommand(command);

  if (result.stdout.trim().length > 0) {
    console.log(result.stdout.trimEnd());
  }
  if (result.stderr.trim().length > 0) {
    console.error(result.stderr.trimEnd());
  }

  if (!options.allowFailure && result.exitCode !== 0) {
    printStep("FAIL", `${label} (exit=${result.exitCode})`);
    throw new Error(
      `Command failed (exit=${result.exitCode}): ${command.join(" ")}`,
    );
  }

  const status: StepStatus = result.exitCode === 0 ? "PASS" : "INFO";
  const suffix = result.exitCode === 0 ? "" : ` (exit=${result.exitCode}, allowed)`;
  printStep(status, `${label}${suffix}`);

  return result;
}

async function waitForAgentPanel(sessionKey: string, retries: number): Promise<string> {
  printStage("Wait For OpenCode TUI");
  for (let attempt = 1; attempt <= retries; attempt += 1) {
    await Bun.sleep(2000);
    const view = await runTinyverse(
      [
        "view",
        "--session",
        sessionKey,
        "--panel",
        "agent",
        "--output",
        "raw",
      ],
      { allowFailure: false, label: `Capture agent panel (attempt ${attempt}/${retries})` },
    );

    const hasMarker = OPEN_CODE_MARKERS.some((marker) =>
      view.stdout.toLowerCase().includes(marker.toLowerCase()),
    );
    if (hasMarker) {
      printStep("PASS", `OpenCode TUI marker found on attempt ${attempt}/${retries}`);
      return view.stdout;
    }

    printStep("INFO", `OpenCode marker not found yet (${attempt}/${retries}), retrying`);
  }

  throw new Error("OpenCode TUI did not appear in agent panel output after retries");
}

function assertContains(text: string, needle: string, description: string): void {
  if (!text.includes(needle)) {
    throw new Error(`Expected ${description} to contain '${needle}'`);
  }
}

function parseFlag(name: string): string | undefined {
  const index = Bun.argv.indexOf(name);
  if (index === -1) {
    return undefined;
  }
  const value = Bun.argv[index + 1];
  if (!value) {
    throw new Error(`Missing value for ${name}`);
  }
  return value;
}

const repoRoot = findRepoRoot(import.meta.dir);
const key =
  parseFlag("--key") ?? `tinyverse-opencode-smoke-${Math.floor(Date.now() / 1000)}`;
const prompt = parseFlag("--prompt") ?? DEFAULT_PROMPT;

console.log(`Running tinyverse smoke workflow for session '${key}'`);

let killed = false;
try {
  printStage("Preflight Cleanup");
  await runTinyverse(["kill", key], {
    allowFailure: true,
    label: `Cleanup existing session '${key}' if present`,
  });

  printStage("Spawn");
  await runTinyverse([
    "spawn",
    "--key",
    key,
    "--agent",
    "opencode",
    "--prompt",
    prompt,
    "--clean-shell",
  ], { label: `Spawn session '${key}' with OpenCode` });

  printStage("Verify List After Spawn");
  const listAfterSpawn = await runTinyverse(["list", "--format", "text"], {
    label: "List sessions",
  });
  assertContains(listAfterSpawn.stdout, key, "list output after spawn");
  printStep("PASS", `Session '${key}' appears in list output`);

  const agentPanel = await waitForAgentPanel(key, 6);
  if (agentPanel.includes(`${prompt}Enter`)) {
    throw new Error("Agent panel still shows literal 'Enter' instead of launching OpenCode");
  }
  printStep("PASS", "Agent panel did not include literal 'Enter' suffix");

  printStage("Kill Session");
  await runTinyverse(["kill", key], { label: `Kill session '${key}'` });
  killed = true;

  printStage("Verify List After Kill");
  const listAfterKill = await runTinyverse(["list", "--format", "text"], {
    label: "List sessions",
  });
  if (listAfterKill.stdout.includes(key)) {
    throw new Error("Session key still appears in list output after kill");
  }
  printStep("PASS", `Session '${key}' is absent from list output after kill`);

  printStage("Result");
  printStep("PASS", "Smoke workflow passed: spawn -> list -> view agent -> kill -> list");
} finally {
  if (!killed) {
    printStage("Final Cleanup");
    await runTinyverse(["kill", key], {
      allowFailure: true,
      label: `Cleanup session '${key}'`,
    });
  }
}
