#!/usr/bin/env node
// Translates README.md into every locale the app ships.
//
// Polish first, translate second: the README is split into blocks and each
// block keeps its markdown shape, so only the prose moves through the
// translator. Code fences, HTML, badges, URLs and image paths are copied
// verbatim — a machine translation of a shell command or a link target would
// silently break the file.
//
//   node scripts/translate-readme.mjs            # fill docs/i18n-cache.json
//   node scripts/translate-readme.mjs --render   # write docs/<locale>/README.md
//
// The cache is committed, so the generated READMEs are reproducible offline and
// any sentence can be corrected by editing one entry.
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const sourceFile = path.join(root, "README.md");
const docsDir = path.join(root, "docs");
const cacheFile = path.join(docsDir, "i18n-cache.json");

const BATCH = 10;
const DELAY_MS = 700;
const ATTEMPTS = 3;

// The locale list the app ships, plus the display names used in the switcher.
const languages = JSON.parse(
  fs.readFileSync(
    path.join(root, "apps", "localsend-gui-slint", "translations", "languages.json"),
    "utf8",
  ),
);
// `languages.json` lists only the locales translated *from* the msgid language,
// so the source locale (zh-CN) is not in it. The README is written in English,
// so zh-CN needs a generated translation like any other.
const LOCALES = [...new Set([...languages.map((entry) => entry.locale), "zh-CN"])].sort();

// Simplified Chinese is hand-written (docs/zh-CN/README.md), because it is the
// project's own language and the machine output was noticeably wrong there.
// The generator translates every other locale and leaves this file alone.
const HAND_WRITTEN = new Set(["zh-CN"]);
const DISPLAY = Object.fromEntries(languages.map((entry) => [entry.locale, entry.name]));
DISPLAY["en-US"] = "English";
DISPLAY["zh-CN"] = "简体中文";

// The README is written in English; every other locale is generated from it.
const SOURCE_LOCALE = "en-US";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

/** Text that must never reach the translator. */
function isProtected(line) {
  const trimmed = line.trim();
  return (
    trimmed === "" ||
    trimmed.startsWith("```") ||
    trimmed.startsWith("<") || // HTML blocks and <picture>/<source>
    trimmed.startsWith("[![") || // badge rows
    trimmed.startsWith("![") || // standalone images
    trimmed.startsWith("{") ||
    /^[|\s:-]+$/.test(trimmed) || // table separator rows
    /^#+\s*$/.test(trimmed)
  );
}

/**
 * A translatable piece of one line.
 *
 * `prefix` is the markdown lead-in that must survive ("## ", "- ", "| "), and
 * the value is the prose between it and the trailing markdown (" |", "**").
 */
function splitLine(line) {
  if (isProtected(line)) return null;

  const heading = line.match(/^(#{1,6}\s+)(.*)$/);
  if (heading) return { prefix: heading[1], text: heading[2], suffix: "" };

  const bullet = line.match(/^(\s*[-*]\s+)(.*)$/);
  if (bullet) return { prefix: bullet[1], text: bullet[2], suffix: "" };

  if (line.startsWith("|")) {
    // Translate cell by cell, keeping the pipe layout intact.
    const cells = line.split("|");
    const inner = cells.slice(1, -1);
    if (inner.some((cell) => cell.trim() !== "")) {
      return { prefix: "|", text: inner.join("|"), suffix: "|", table: true };
    }
    return null;
  }

  return { prefix: "", text: line, suffix: "" };
}

/**
 * Replace the parts of a line that a translator must not touch with tokens
 * that survive translation, then restore them afterwards.
 */
// Product and platform names that must survive translation unchanged; machine
// translators otherwise render "Windows" as the noun for a window.
const TERMS = [
  "localsend-next", "LocalSend", "slint", "Slint", "Rust", "macOS", "Windows",
  "Linux", "Debian", "Ubuntu", "Fedora", "RHEL", "AppImage", "DMG", "ZIP",
  "powershell", "PowerShell", "x86_64", "aarch64", "arm64", "GTK3", "AppIndicator",
  "FemtoVG", "TUI", "GUI", "RPC", "TLS", "SHA256SUMS", "Telemetry", "Android",
];

function protectInline(text) {
  const saved = [];
  const stash = (value) => {
    saved.push(value);
    return `@@${saved.length - 1}@@`;
  };

  let out = text;
  for (const term of TERMS) {
    const pattern = new RegExp(`(?<![\\w-])${term.replace(/[-/\\^$*+?.()|[\]{}]/g, "\\$&")}(?![\\w-])`, "g");
    out = out.replace(pattern, (match) => stash(match));
  }

  out = out
    // Inline code spans.
    .replace(/`[^`]+`/g, (match) => stash(match))
    // Link targets, keeping the visible text translatable.
    .replace(/\]\(([^)]+)\)/g, (_match, target) => `](${stash(target)})`)
    // Bare URLs and mailto links.
    .replace(/https?:\/\/[^\s)]+/g, (match) => stash(match))
    // Bold/italic markers are translation-safe, but backtick-free paths are not.
    .replace(/\b[a-zA-Z0-9_-]+\/[a-zA-Z0-9._/-]+/g, (match) => stash(match));

  return { out, saved };
}

function restoreInline(text, saved) {
  let out = text;
  // Tokens can come back re-spaced or re-ordered by the translator; loop until
  // stable and fall back to the original token text when a slot is missing.
  for (let pass = 0; pass < 3; pass += 1) {
    const next = out.replace(/@@\s*(\d+)\s*@@/g, (match, index) => saved[Number(index)] ?? match);
    if (next === out) break;
    out = next;
  }
  return out;
}

function collectSegments(markdown) {
  const segments = [];
  let inFence = false;
  for (const line of markdown.split("\n")) {
    if (line.trim().startsWith("```")) {
      inFence = !inFence;
      continue;
    }
    if (inFence) continue;
    const parts = splitLine(line);
    if (!parts) continue;
    if (parts.table) {
      for (const cell of parts.text.split("|")) {
        const { out } = protectInline(cell.trim());
        if (out) segments.push(out);
      }
      continue;
    }
    const { out } = protectInline(parts.text.trim());
    if (out && /[a-zA-Z]/.test(out)) segments.push(out);
  }
  return [...new Set(segments)];
}

/** One request per batch; the endpoint answers an array in the same order. */
async function translateBatch(target, texts) {
  const query = texts.map((text) => `q=${encodeURIComponent(text)}`).join("&");
  const url = `https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=en&tl=${target}&${query}`;
  for (let attempt = 1; attempt <= ATTEMPTS; attempt += 1) {
    try {
      // curl honours https_proxy/all_proxy, which Node's fetch ignores.
      const body = execFileSync(
        "curl",
        ["-sS", "--fail", "--max-time", "40", "-A", "Mozilla/5.0", url],
        { encoding: "utf8" },
      );
      const parsed = JSON.parse(body);
      const list = Array.isArray(parsed) ? parsed : [parsed];
      if (list.length !== texts.length) {
        throw new Error(`expected ${texts.length} results, got ${list.length}`);
      }
      return list.map((item) => (typeof item === "string" ? item : (item?.[0] ?? "")));
    } catch (error) {
      if (attempt === ATTEMPTS) throw error;
      await sleep(DELAY_MS * attempt);
    }
  }
  throw new Error("unreachable");
}

/** The language switcher line every generated README carries. */
function switcher(current) {
  const order = [SOURCE_LOCALE, ...LOCALES.filter((locale) => locale !== SOURCE_LOCALE)];
  return order
    .map((locale) => {
      const label = DISPLAY[locale] ?? locale;
      if (locale === current) return `**${label}**`;
      const href =
        locale === SOURCE_LOCALE ? "../../README.md" : `../../docs/${locale}/README.md`;
      return `[${label}](${href})`;
    })
    .join(" | ");
}

function renderLocale(markdown, locale, cache) {
  const translations = { ...(cache[locale] ?? {}), ...overrideSource(locale) };
  const lines = [];
  let inFence = false;

  for (const line of markdown.split("\n")) {
    if (line.trim().startsWith("```")) {
      inFence = !inFence;
      lines.push(line);
      continue;
    }
    if (inFence) {
      lines.push(line);
      continue;
    }
    // The badge/logo block stays verbatim; only the prose below it moves.
    if (line.includes("[![Latest release]") || line.startsWith("<div align")
        || line.startsWith("</div>") || line.startsWith("    <img")) {
      lines.push(line);
      continue;
    }

    const parts = splitLine(line);
    if (!parts) {
      lines.push(line);
      continue;
    }

    const translate = (text) => {
      const { out, saved } = protectInline(text.trim());
      const translated = translations[out] ?? out;
      return restoreInline(translated, saved);
    };

    if (parts.table) {
      const cells = parts.text.split("|").map((cell) => {
        const trimmed = cell.trim();
        if (!trimmed) return cell;
        return ` ${translate(trimmed)} `;
      });
      lines.push(`|${cells.join("|")}|`);
      continue;
    }

    lines.push(`${parts.prefix}${translate(parts.text)}${parts.suffix}`);
  }

  const body = lines.join("\n");

  // The generated file sits one level down (docs/<locale>/README.md), so every
  // repository-relative target needs one more ".." than the root README uses.
  // Absolute URLs, anchors and the switcher (already written with the correct
  // prefix) are left alone.
  const rehomed = body
    .replace(
      /\]\((?!https?:|mailto:|#|\.\.\/)([^)]+)\)/g,
      (_match, target) => `](../../${target})`,
    )
    // <img src="..."> and <source srcset="..."> inside the screenshot block.
    .replace(
      /\b(src|srcset)="(?!https?:|\.\.\/)([^"]+)"/g,
      (_match, attribute, target) => `${attribute}="../../${target}"`,
    );

  // The switcher is generated from locale data, so drop whatever the source
  // carried before inserting a fresh one: the root README marks its block with
  // `<!-- locale-switcher -->`, and any older output without the marker is
  // removed by matching a `</div>` wrapper that lists other READMEs.
  const switcherBlock = `<div align="center">\n\n${switcher(locale)}\n\n</div>`;
  let cleaned = rehomed;

  const markerIndex = cleaned.indexOf("<!-- locale-switcher -->");
  if (markerIndex !== -1) {
    const closing = cleaned.indexOf("</div>", markerIndex);
    cleaned =
      cleaned.slice(0, markerIndex) + cleaned.slice(closing + "</div>".length);
  }

  const out = cleaned.split("\n");
  const isSwitcherStart = (index) =>
    (out[index] ?? "").trim() === "</div>" &&
    (out[index + 1] ?? "").trim() === "" &&
    (out[index + 2] ?? "").includes("README.md)");
  const kept = [];
  for (let index = 0; index < out.length; index += 1) {
    if (isSwitcherStart(index)) {
      while (index < out.length && (out[index] ?? "").trim() !== "</div>") index += 1;
      while (index + 1 < out.length && (out[index + 1] ?? "").trim() === "") index += 1;
      continue;
    }
    kept.push(out[index]);
  }

  const insertAt = kept.findIndex((line) => line.trim() === "</div>");
  const at = insertAt === -1 ? 0 : insertAt + 1;
  const withSwitcher = [
    ...kept.slice(0, at),
    "",
    switcherBlock,
    ...kept.slice(at),
  ].join("\n");
  return withSwitcher;
}

const source = fs.readFileSync(sourceFile, "utf8");
const segments = collectSegments(source);
// Hand-written corrections win over the machine cache; both are committed.
// The generator never writes to this file, so a rerun cannot lose a fix.
const overridesFile = path.join(docsDir, "i18n-overrides.json");
const overrideSource = (locale) => {
  if (!overrideSource._cache) {
    overrideSource._cache = fs.existsSync(overridesFile)
      ? JSON.parse(fs.readFileSync(overridesFile, "utf8"))
      : {};
  }
  return overrideSource._cache[locale] ?? {};
};

const cache = fs.existsSync(cacheFile) ? JSON.parse(fs.readFileSync(cacheFile, "utf8")) : {};

if (process.argv.includes("--render")) {
  fs.mkdirSync(docsDir, { recursive: true });
  for (const locale of LOCALES) {
    if (HAND_WRITTEN.has(locale)) continue;
    const output = path.join(docsDir, locale, "README.md");
    fs.mkdirSync(path.dirname(output), { recursive: true });
    fs.writeFileSync(output, renderLocale(source, locale, cache));
  }
  console.log(`rendered ${LOCALES.length} README files for ${segments.length} segments`);
  process.exit(0);
}

let done = 0;
for (const locale of LOCALES) {
  if (HAND_WRITTEN.has(locale)) continue;
  const pending = segments.filter((text) => !cache[locale]?.[text]);
  if (pending.length === 0) continue;
  cache[locale] ??= {};
  for (let i = 0; i < pending.length; i += BATCH) {
    const batch = pending.slice(i, i + BATCH);
    const translated = await translateBatch(locale, batch);
    batch.forEach((text, index) => {
      if (translated[index]) cache[locale][text] = translated[index];
    });
    fs.writeFileSync(cacheFile, `${JSON.stringify(cache, null, 2)}\n`);
    done += batch.length;
    process.stdout.write(`\r${locale}: ${Math.min(i + BATCH, pending.length)}/${pending.length} (${done})`);
    await sleep(DELAY_MS);
  }
  process.stdout.write("\n");
}
console.log(`cache holds ${Object.values(cache).reduce((sum, v) => sum + Object.keys(v).length, 0)} segments`);
