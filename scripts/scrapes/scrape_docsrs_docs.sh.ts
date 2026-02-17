#!/usr/bin/env bun

import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  fileStemFromSourcePath,
  mapConcurrent,
  outputDirectoryRelative,
  prepareOutputDirectory,
  resolveOutputDirectory,
  sanitizeSegment,
  scrapeDocsPage,
  sourcePathFromUrl,
  summarizeArtifacts,
} from "../helpers/docs_scrape.sh.ts";
import {
  discoverDocsRsUrls,
  docsRsPathPrefix,
  docsRsRootUrl,
  parseDocsRsTargetFromUrl,
} from "../helpers/docsrs_scrape.sh.ts";
import { findRepoRoot } from "../helpers/run_root.sh.ts";

const DEFAULT_DOCS_RS_URL = "https://docs.rs/tmux_interface/latest/tmux_interface/";
const DEFAULT_OUTPUT_SOURCE_PREFIX = "docsrs";
const CONCURRENCY = 4;

function isHttpUrl(value: string): boolean {
  return /^https?:\/\//i.test(value);
}

const firstArg = Bun.argv[2];
const secondArg = Bun.argv[3];
const docsRootArg = firstArg && isHttpUrl(firstArg) ? firstArg : DEFAULT_DOCS_RS_URL;
const outputArg = firstArg && !isHttpUrl(firstArg) ? firstArg : secondArg;

const target = parseDocsRsTargetFromUrl(docsRootArg);
const docsRoot = docsRsRootUrl(target);
const pathPrefix = docsRsPathPrefix(target);
const sourceKey = `${DEFAULT_OUTPUT_SOURCE_PREFIX}_${sanitizeSegment(target.crateName)}`;
const defaultOutputDirRelative = `docs/external/${sourceKey}`;

const repoRoot = findRepoRoot(import.meta.dir);
const outputDir = resolveOutputDirectory(repoRoot, defaultOutputDirRelative, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const discovery = await discoverDocsRsUrls(target);
if (discovery.urls.length === 0) {
  throw new Error(`Docs // Scrape // No docs.rs URLs found (root=${docsRoot},crate=${target.crateName})`);
}

const scrapeResults = await mapConcurrent(discovery.urls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, discovery.urls.length),
);

const moduleRootStem = [
  "docsrs",
  ...target.modulePath
    .split("/")
    .filter(Boolean)
    .map((segment) => sanitizeSegment(segment)),
]
  .filter(Boolean)
  .join("__");

const artifacts = buildPageArtifacts(
  scrapeResults,
  (result) =>
    fileStemFromSourcePath(sourcePathFromUrl(result.url), {
      rootPath: pathPrefix,
      rootStem: moduleRootStem || "docsrs",
      trimPrefixPath: `/${target.crateName}/${target.version}`,
      stripMarkdownExtension: true,
    }),
  (result) => sourcePathFromUrl(result.url),
);

const capturedAt = new Date().toISOString();

for (const artifact of artifacts) {
  await Bun.write(
    resolve(outputDir, artifact.fileName),
    buildSnapshotPageMarkdown(artifact, capturedAt, {
      sourceKey,
      sourceRoot: docsRoot,
      baseKeywords: `docs.rs, rust, ${target.crateName}, ${target.modulePath}`,
      summaryFallback: `docs.rs snapshot for ${target.crateName}.`,
      collectionMethodNote: "docs.rs crate sitemap discovery with in-page link expansion.",
      dropKeywordPathSegments: 2,
    }),
  );
}

await Bun.write(
  resolve(outputDir, "index.ext.md"),
  buildSnapshotIndexMarkdown(artifacts, capturedAt, {
    sourceKey,
    sourceRoot: docsRoot,
    outputDirRelative,
    discovery,
    scopeText: `${artifacts.length} pages under ${pathPrefix}`,
    indexKeywords: `docs.rs, rust, ${target.crateName}, docs index`,
    indexSummary: `This index links one .ext.md snapshot per discovered docs.rs page for ${target.crateName}.`,
    notes: [
      "- Per-page files are flattened and prefixed from docs.rs path segments.",
      "- Re-run this scraper to refresh all snapshots in this directory.",
    ],
  }),
);

const stats = summarizeArtifacts(artifacts);
console.log(
  `Docs // Scrape // Wrote split external docs (source=${sourceKey},crate=${target.crateName},version=${target.version},module=${target.modulePath},discovery=${discovery.method},pages=${artifacts.length},ok=${stats.successCount},failed=${stats.failureCount},blocked=${stats.blockedPages.join(",") || "none"},dir=${outputDir})`,
);
