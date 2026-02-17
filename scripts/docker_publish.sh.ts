#!/usr/bin/env bun

import { join } from "node:path";
import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommand } from "./helpers/run_command.sh.ts";

type PublishOptions = {
  imageRepo: string;
  tags: string[];
  targets: string[];
  platforms: string;
  push: boolean;
};

const repoRoot = findRepoRoot(import.meta.dir);
const dockerfilePath = join(repoRoot, "docker", "Dockerfile");
const options = parseOptions(Bun.argv.slice(2));

for (const target of options.targets) {
  for (const tag of options.tags) {
    const imageTag = `${options.imageRepo}/${target}:${tag}`;
    const args = [
      "docker",
      "buildx",
      "build",
      "--file",
      dockerfilePath,
      "--target",
      target,
      "--platform",
      options.platforms,
      "--tag",
      imageTag,
      options.push ? "--push" : "--load",
      repoRoot,
    ];

    await runCommand(args, { cwd: repoRoot });
  }
}

function parseOptions(args: string[]): PublishOptions {
  let imageRepo = process.env.CI_DOCKER_IMAGE_REPO ?? "";
  const tags: string[] = [];
  let targets = ["run", "ci", "agentbox"];
  let platforms = "linux/amd64";
  const push = args.includes("--push");

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];

    if (arg === "--image-repo") {
      const value = args[index + 1];
      if (!value) {
        throw new Error("Missing value for --image-repo");
      }
      imageRepo = value;
      index += 1;
      continue;
    }

    if (arg === "--tag") {
      const value = args[index + 1];
      if (!value) {
        throw new Error("Missing value for --tag");
      }
      tags.push(value);
      index += 1;
      continue;
    }

    if (arg === "--targets") {
      const value = args[index + 1];
      if (!value) {
        throw new Error("Missing value for --targets");
      }
      targets = value.split(",").map((entry) => entry.trim()).filter(Boolean);
      index += 1;
      continue;
    }

    if (arg === "--platforms") {
      const value = args[index + 1];
      if (!value) {
        throw new Error("Missing value for --platforms");
      }
      platforms = value;
      index += 1;
      continue;
    }
  }

  if (!imageRepo) {
    throw new Error("Missing image repository (set --image-repo or CI_DOCKER_IMAGE_REPO)");
  }

  if (tags.length === 0) {
    tags.push("local");
  }

  return {
    imageRepo,
    tags,
    targets,
    platforms,
    push,
  };
}
