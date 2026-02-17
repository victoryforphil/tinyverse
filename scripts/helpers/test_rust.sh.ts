#!/usr/bin/env bun

/**
 * Rust test runner: prefers nextest, falls back to cargo test.
 * Usage: test_rust.sh.ts [cargo_args...]
 */

const args = Bun.argv.slice(2);

// Try nextest first
let result = await Bun.spawn(["cargo", "nextest", "run", ...args], {
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
}).exited;

if (result === 0) {
  process.exit(0);
}

// Check if nextest is not installed or unsupported
const whichNextest = await Bun.spawn(["which", "cargo-nextest"], {
  stdout: "pipe",
  stderr: "pipe",
}).exited;

if (whichNextest !== 0) {
  console.warn("⚠ nextest not found, falling back to cargo test");
}

// Fall back to cargo test
result = await Bun.spawn(["cargo", "test", ...args], {
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
}).exited;

process.exit(result);
