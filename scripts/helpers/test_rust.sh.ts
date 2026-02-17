#!/usr/bin/env bun

/**
 * Rust test runner: prefers nextest, falls back to cargo test.
 * Transparently passes all args to the underlying test runner.
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

// Check if nextest command itself failed (not just test failure)
// Only fallback if nextest binary is unavailable
const checkNextest = Bun.spawn(["cargo", "nextest", "--version"], {
  stdout: "pipe",
  stderr: "pipe",
});

const nextestExitCode = await checkNextest.exited;

if (nextestExitCode !== 0) {
  console.warn("⚠ cargo-nextest unavailable, falling back to cargo test");
  
  // Fall back to cargo test with same args
  const fallbackResult = await Bun.spawn(["cargo", "test", ...args], {
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }).exited;

  process.exit(fallbackResult);
}

// nextest is available but tests failed; propagate the failure
process.exit(result);
