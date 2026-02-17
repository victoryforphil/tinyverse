#!/usr/bin/env bun

import { mkdirSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { resolveShellRcTarget, shellLabel } from "./helpers/shell_rc.sh.ts";

const START_MARKER = "# >>> tinyverse aliases >>>";
const END_MARKER = "# <<< tinyverse aliases <<<";
const dryRun = Bun.argv.includes("--dry-run");

type AliasConfig = {
  sourceEnvVar: string;
  aliases: Record<string, string>;
};

const repoRoot = findRepoRoot(import.meta.dir);
const repoRealPath = realpathSync(repoRoot);
const aliasConfig = readAliasConfig(join(repoRoot, "scripts/sys_install.aliases.json"));
const rcTarget = resolveShellRcTarget();

mkdirSync(dirname(rcTarget.path), { recursive: true });

const existingContent = readFile(rcTarget.path);
const updatedContent = upsertManagedBlock(
  existingContent,
  buildAliasBlock(repoRealPath, aliasConfig),
);

if (existingContent === updatedContent) {
  console.log(
    `Sys Install // Shell RC // Already configured (file=${rcTarget.path},shell=${shellLabel(rcTarget)})`,
  );
  process.exit(0);
}

if (dryRun) {
  console.log(`Sys Install // Dry Run // Would update ${rcTarget.path}`);
  console.log("Sys Install // Dry Run // Managed block preview:");
  console.log(buildAliasBlock(repoRealPath, aliasConfig));
  process.exit(0);
}

writeFileSync(rcTarget.path, updatedContent, "utf8");

console.log(
  `Sys Install // Shell RC // Updated aliases (file=${rcTarget.path},shell=${shellLabel(rcTarget)})`,
);
console.log(`Sys Install // Next // Run: source ${rcTarget.path}`);

function readAliasConfig(path: string): AliasConfig {
  const raw = readFileSync(path, "utf8");
  const parsed = JSON.parse(raw) as Partial<AliasConfig>;

  if (!parsed || typeof parsed !== "object") {
    throw new Error(`Sys Install // Config // Invalid config file (${path})`);
  }

  if (!parsed.sourceEnvVar || typeof parsed.sourceEnvVar !== "string") {
    throw new Error(`Sys Install // Config // Missing sourceEnvVar (${path})`);
  }

  if (!parsed.aliases || typeof parsed.aliases !== "object") {
    throw new Error(`Sys Install // Config // Missing aliases map (${path})`);
  }

  const aliases = Object.entries(parsed.aliases)
    .filter(([alias, pkg]) => alias.trim() && typeof pkg === "string" && pkg.trim())
    .reduce<Record<string, string>>((acc, [alias, pkg]) => {
      acc[alias.trim()] = pkg.trim();
      return acc;
    }, {});

  if (Object.keys(aliases).length === 0) {
    throw new Error(`Sys Install // Config // No aliases defined (${path})`);
  }

  return {
    sourceEnvVar: parsed.sourceEnvVar,
    aliases,
  };
}

function readFile(path: string): string {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return "";
  }
}

function buildAliasBlock(repoPath: string, config: AliasConfig): string {
  const escapedPath = repoPath.replace(/"/g, '\\"');
  const lines = [
    START_MARKER,
    `export ${config.sourceEnvVar}="${escapedPath}"`,
  ];

  const sortedAliases = Object.keys(config.aliases).sort((a, b) => a.localeCompare(b));
  for (const aliasName of sortedAliases) {
    const packageName = config.aliases[aliasName];
    lines.push(
      `alias ${aliasName}='cargo run --release --manifest-path "$${config.sourceEnvVar}/Cargo.toml" -p ${packageName} --'`,
    );
  }

  lines.push(END_MARKER);
  return lines.join("\n");
}

function upsertManagedBlock(content: string, block: string): string {
  const escapedStart = escapeRegex(START_MARKER);
  const escapedEnd = escapeRegex(END_MARKER);
  const blockPattern = new RegExp(`${escapedStart}[\\s\\S]*?${escapedEnd}\\n*`, "m");

  if (blockPattern.test(content)) {
    const replaced = content.replace(blockPattern, `${block}\n`);
    return normalizeTrailingNewline(replaced);
  }

  if (!content.trim()) {
    return `${block}\n`;
  }

  const normalized = normalizeTrailingNewline(content);
  return `${normalized}\n${block}\n`;
}

function normalizeTrailingNewline(value: string): string {
  return `${value.replace(/\s*$/, "")}\n`;
}

function escapeRegex(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
