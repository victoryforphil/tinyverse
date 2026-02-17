#!/usr/bin/env bun

import { resolve } from "node:path";
import {
  buildPageArtifacts,
  buildSnapshotIndexMarkdown,
  buildSnapshotPageMarkdown,
  dedupeSorted,
  discoverUrlsFromRootPage,
  discoverUrlsFromSitemap,
  fileStemFromSourcePath,
  mapConcurrent,
  outputDirectoryRelative,
  prepareOutputDirectory,
  resolveOutputDirectory,
  scrapeDocsPage,
  sourcePathFromUrl,
  summarizeArtifacts,
} from "../helpers/docs_scrape.sh.ts";
import { findRepoRoot } from "../helpers/run_root.sh.ts";

type DiscoveryMethod = "sitemap+seed" | "html+seed";

type DiscoveryResult = {
  urls: string[];
  method: DiscoveryMethod;
  detail: string;
};

const SOURCE_KEY = "diesel";
const SOURCE_ROOT = "https://diesel.rs/guides/";
const GUIDES_ROOT = new URL(SOURCE_ROOT);
const GUIDES_HOST = GUIDES_ROOT.hostname;
const GUIDES_PATH = GUIDES_ROOT.pathname.replace(/\/+$/, "");
const GUIDES_SITEMAP_URL = "https://diesel.rs/sitemap.xml";
const API_ROOT_URL = "https://docs.diesel.rs/main/diesel/index.html";
const API_HOST = new URL(API_ROOT_URL).hostname;
const API_PREFIX = "/main/diesel/";
const DEFAULT_OUTPUT_DIR_RELATIVE = "docs/external/diesel";
const CONCURRENCY = 4;

const GUIDE_SEED_URLS = [
  "https://diesel.rs/guides/",
  "https://diesel.rs/guides/getting-started.html",
  "https://diesel.rs/guides/all-about-selects.html",
  "https://diesel.rs/guides/all-about-updates.html",
  "https://diesel.rs/guides/all-about-inserts.html",
  "https://diesel.rs/guides/relations.html",
  "https://diesel.rs/guides/composing-applications.html",
  "https://diesel.rs/guides/schema-in-depth.html",
  "https://diesel.rs/guides/extending-diesel.html",
  "https://diesel.rs/guides/configuring-diesel-cli.html",
  "https://diesel.rs/guides/migration_guide.html",
];

const API_SEED_URLS = [API_ROOT_URL, "https://docs.diesel.rs/main/diesel/all.html"];

function normalizeGuideUrl(url: string): string | null {
  try {
    const parsed = new URL(url, SOURCE_ROOT);
    if (parsed.hostname !== GUIDES_HOST) {
      return null;
    }

    parsed.hash = "";
    parsed.search = "";
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";

    if (parsed.pathname !== GUIDES_PATH && !parsed.pathname.startsWith(`${GUIDES_PATH}/`)) {
      return null;
    }

    if (!parsed.pathname.endsWith(".html") && parsed.pathname !== GUIDES_PATH) {
      return null;
    }

    return parsed.toString();
  } catch {
    return null;
  }
}

function normalizeApiUrl(url: string): string | null {
  try {
    const parsed = new URL(url, API_ROOT_URL);
    if (parsed.hostname !== API_HOST) {
      return null;
    }

    parsed.hash = "";
    parsed.search = "";
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";

    if (!parsed.pathname.startsWith(API_PREFIX)) {
      return null;
    }

    if (!parsed.pathname.endsWith("index.html") && !parsed.pathname.endsWith("all.html")) {
      return null;
    }

    return parsed.toString();
  } catch {
    return null;
  }
}

function fileStemFromUrl(url: string): string {
  const sourcePath = sourcePathFromUrl(url, { emptyPathFallback: "/guides" });

  if (sourcePath.startsWith("/guides")) {
    return fileStemFromSourcePath(sourcePath, {
      rootPath: "/guides",
      rootStem: "guides",
      prefix: "diesel",
      stripMarkdownExtension: true,
    });
  }

  if (sourcePath.startsWith(API_PREFIX)) {
    return fileStemFromSourcePath(sourcePath, {
      rootPath: API_PREFIX.replace(/\/+$/, ""),
      rootStem: "api",
      prefix: "diesel",
      stripMarkdownExtension: true,
    });
  }

  return fileStemFromSourcePath(sourcePath, {
    prefix: "diesel",
    rootStem: "page",
    stripMarkdownExtension: true,
  });
}

async function discoverDocsUrls(): Promise<DiscoveryResult> {
  const [sitemapGuideUrls, htmlGuideUrls, htmlApiUrls] = await Promise.all([
    discoverUrlsFromSitemap(GUIDES_SITEMAP_URL, normalizeGuideUrl),
    discoverUrlsFromRootPage(SOURCE_ROOT, normalizeGuideUrl),
    discoverUrlsFromRootPage(API_ROOT_URL, normalizeApiUrl),
  ]);

  const guideUrls = sitemapGuideUrls.length > 0 ? sitemapGuideUrls : htmlGuideUrls;
  const method: DiscoveryMethod = sitemapGuideUrls.length > 0 ? "sitemap+seed" : "html+seed";
  const normalizedGuideUrls = guideUrls.map((url) => normalizeGuideUrl(url)).filter((url): url is string => Boolean(url));
  const normalizedHtmlApiUrls = htmlApiUrls.map((url) => normalizeApiUrl(url)).filter((url): url is string => Boolean(url));

  const normalizedGuideSeeds = GUIDE_SEED_URLS.map((url) => normalizeGuideUrl(url)).filter(
    (url): url is string => Boolean(url),
  );
  const normalizedApiSeeds = API_SEED_URLS.map((url) => normalizeApiUrl(url)).filter((url): url is string => Boolean(url));

  const mergedUrls = dedupeSorted([
    ...normalizedGuideSeeds,
    ...normalizedGuideUrls,
    ...normalizedApiSeeds,
    ...normalizedHtmlApiUrls,
  ]);
  const detail = `guides=${normalizedGuideUrls.length},api=${normalizedHtmlApiUrls.length},seed=${GUIDE_SEED_URLS.length + API_SEED_URLS.length},sitemap=${GUIDES_SITEMAP_URL}`;

  return {
    urls: mergedUrls,
    method,
    detail,
  };
}

const repoRoot = findRepoRoot(import.meta.dir);
const outputArg = Bun.argv[2];
const outputDir = resolveOutputDirectory(repoRoot, DEFAULT_OUTPUT_DIR_RELATIVE, outputArg);
const outputDirRelative = outputDirectoryRelative(repoRoot, outputDir);

prepareOutputDirectory(outputDir);

const discovery = await discoverDocsUrls();
if (discovery.urls.length === 0) {
  throw new Error(`No docs URLs found for source ${SOURCE_KEY} (sitemap=${GUIDES_SITEMAP_URL})`);
}

const scrapeResults = await mapConcurrent(discovery.urls, CONCURRENCY, (url, index) =>
  scrapeDocsPage(url, index + 1, discovery.urls.length),
);

const artifacts = buildPageArtifacts(
  scrapeResults,
  (result) => fileStemFromUrl(result.url),
  (result) => sourcePathFromUrl(result.url, { emptyPathFallback: "/guides" }),
);

const capturedAt = new Date().toISOString();

for (const artifact of artifacts) {
  await Bun.write(
    resolve(outputDir, artifact.fileName),
    buildSnapshotPageMarkdown(artifact, capturedAt, {
      sourceKey: SOURCE_KEY,
      sourceRoot: SOURCE_ROOT,
      baseKeywords: "diesel, rust, orm, guides, api",
      summaryFallback: "Diesel documentation page snapshot.",
      collectionMethodNote: "sitemap-first guide discovery with seed URLs and API index fallback.",
    }),
  );
}

await Bun.write(
  resolve(outputDir, "index.ext.md"),
  buildSnapshotIndexMarkdown(artifacts, capturedAt, {
    sourceKey: SOURCE_KEY,
    sourceRoot: SOURCE_ROOT,
    outputDirRelative,
    discovery,
    scopeText: `${artifacts.length} Diesel guides + API index pages`,
    indexKeywords: "diesel, rust orm, diesel guides, diesel api index",
    indexSummary: "This index links one `.ext.md` file per Diesel docs page snapshot.",
    notes: [
      "- Discovery is sitemap-first for guides with root-page fallback and always-on seed URLs.",
      "- API coverage is scoped to Diesel crate entry pages under docs.diesel.rs/main/diesel/.",
      "- Re-run: bun scripts/scrapes/scrape_diesel_docs.sh.ts",
    ],
  }),
);

const stats = summarizeArtifacts(artifacts);
console.log(
  `Docs // Scrape // Wrote split external docs (source=${SOURCE_KEY},discovery=${discovery.method},pages=${artifacts.length},ok=${stats.successCount},failed=${stats.failureCount},blocked=${stats.blockedPages.join(",") || "none"},dir=${outputDirRelative})`,
);
