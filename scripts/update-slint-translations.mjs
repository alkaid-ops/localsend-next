#!/usr/bin/env node
// Generates the Slint gettext bundles under apps/localsend-gui-slint/translations.
//
// Every `@tr("...")` in the UI uses the Chinese source string as its msgid.
// `locale-en.json` holds the hand-written English for those msgids, aligned with
// upstream LocalSend's wording wherever the two catalogs share a string.
// All other languages come from two sources, upstream first:
//   1. `translations/upstream/<locale>.json` — a mirror of upstream LocalSend's
//      app/assets/i18n/<locale>.json, matched through the zh-CN catalog.
//   2. `translations/locale-machine-cache.json` — machine translations of the
//      strings only this client has. It is committed so the generated .po files
//      stay reproducible offline and can be corrected by hand.
//
// `--collect` prints the (msgid, english) pairs that still need machine
// translation, so scripts/translate-slint-locales.mjs can fill the cache.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const appDir = path.join(root, "apps", "localsend-gui-slint");
const uiDir = path.join(appDir, "ui");
const translationsDir = path.join(appDir, "translations");
const upstreamDir = path.join(translationsDir, "upstream");
const enFile = path.join(translationsDir, "locale-en.json");
const cacheFile = path.join(translationsDir, "locale-machine-cache.json");
const languagesFile = path.join(translationsDir, "languages.json");

// Upstream LocalSend locales (app/assets/i18n), in the official order.
const LOCALES = [
  "ar", "az", "be", "bg", "bn", "ca", "cs", "da", "de", "el", "en-IN", "es-ES",
  "et", "eu", "fa", "fi", "fil-PH", "fr", "ga", "gl", "gu", "he", "hi", "hu",
  "hy", "id", "it", "ja", "km", "ko", "ky", "lo", "ml", "mn", "ms", "ne", "nl",
  "pl", "pt-BR", "pt-PT", "ro", "ru", "si", "sk", "sl", "sr", "sr-Cyrl", "sv",
  "ta", "th", "tr", "uk", "ur", "uz", "vi", "zh-HK", "zh-TW",
];

// LocalSend's own locale code -> the BCP 47 tag used for machine translation.
const MT_TAG = {
  "en-IN": "en", "es-ES": "es", "fil-PH": "tl", "pt-BR": "pt", "pt-PT": "pt",
  "sr-Cyrl": "sr", "zh-HK": "zh-TW", "zh-CN": "zh-CN", "zh-TW": "zh-TW",
};

// Display name of each locale in its own language, as the language picker shows it.
const DISPLAY = {
  ar: "العربية", az: "Azərbaycanca", be: "Беларуская", bg: "Български", bn: "বাংলা",
  ca: "Català", cs: "Čeština", da: "Dansk", de: "Deutsch", el: "Ελληνικά",
  "en-US": "English", "en-IN": "English (India)", "es-ES": "Español", et: "Eesti",
  eu: "Euskara", fa: "فارسی", fi: "Suomi", "fil-PH": "Filipino", fr: "Français",
  ga: "Gaeilge", gl: "Galego", gu: "ગુજરાતી", he: "עברית", hi: "हिन्दी",
  hu: "Magyar", hy: "Հայերեն", id: "Bahasa Indonesia", it: "Italiano", ja: "日本語",
  km: "ខ្មែរ", ko: "한국어", ky: "Кыргызча", lo: "ລາວ", ml: "മലയാളം", mn: "Монгол",
  ms: "Bahasa Melayu", ne: "नेपाली", nl: "Nederlands", pl: "Polski",
  "pt-BR": "Português (Brasil)", "pt-PT": "Português (Portugal)", ro: "Română",
  ru: "Русский", si: "සිංහල", sk: "Slovenčina", sl: "Slovenščina", sr: "Српски",
  "sr-Cyrl": "Српски (ћирилица)", sv: "Svenska", ta: "தமிழ்", th: "ไทย", tr: "Türkçe",
  uk: "Українська", ur: "اردو", uz: "Oʻzbekcha", vi: "Tiếng Việt",
  "zh-CN": "简体中文", "zh-HK": "繁體中文（香港）", "zh-TW": "繁體中文（台灣）",
};

function slintFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = path.join(directory, entry.name);
    return entry.isDirectory() ? slintFiles(fullPath) : entry.name.endsWith(".slint") ? [fullPath] : [];
  });
}

const runtimeMsgids = new Set();
const msgids = new Set();
for (const file of slintFiles(uiDir)) {
  const source = fs.readFileSync(file, "utf8");
  // Matches both @tr("text") and @tr("text", argument) forms.
  for (const match of source.matchAll(/@tr\("([^"]*)"(?:\s*,[^)]*)?\)/g)) msgids.add(match[1]);
}

// Strings built in Rust (status messages, history rows) are translated through
// src/runtime_strings.rs; they join the same msgid set.
const rustDirs = [path.join(appDir, "src"), path.join(root, "apps", "localsend-tray", "src")];
const rustSources = rustDirs.flatMap((dir) => {
  if (!fs.existsSync(dir)) return [];
  return fs
    .readdirSync(dir)
    .filter((file) => file.endsWith(".rs"))
    .map((file) => path.join(dir, file));
});
for (const file of rustSources) {
  const source = fs.readFileSync(file, "utf8");
  for (const match of source.matchAll(/"([^"\\n]*[\u4e00-\u9fff][^"\\n]*)"/g)) {
    const text = match[1];
    // Named placeholders (`{error}`, `{count}`) are normalized to `{}` so the
    // same template can be translated and then filled by `localsend_i18n::t`.
    const named = text.match(/\{[a-zA-Z_][a-zA-Z0-9_]*\}/g);
    const braces = text.match(/[{}]/g)?.length ?? 0;
    if (named) {
      if (braces !== named.length * 2) continue;
      runtimeMsgids.add(text.replace(/\{[a-zA-Z_][a-zA-Z0-9_]*\}/g, "{}"));
    } else if (braces === 0) {
      runtimeMsgids.add(text);
    }
  }
}

const en = JSON.parse(fs.readFileSync(enFile, "utf8"));
const missingEn = [...msgids].filter((msgid) => !(msgid in en));
if (missingEn.length > 0) {
  throw new Error(
    `locale-en.json is missing ${missingEn.length} msgids:\n${missingEn
      .map((m) => `- ${m}`)
      .join("\n")}`,
  );
}

function flatten(value, prefix = "", out = {}) {
  for (const [key, child] of Object.entries(value)) {
    if (key.startsWith("@")) continue;
    const next = prefix ? `${prefix}.${key}` : key;
    if (child && typeof child === "object") flatten(child, next, out);
    else if (typeof child === "string") out[next] = child;
  }
  return out;
}

const upstream = new Map();
function readUpstream(locale) {
  if (!upstream.has(locale)) {
    const file = path.join(upstreamDir, `${locale}.json`);
    upstream.set(locale, fs.existsSync(file) ? flatten(JSON.parse(fs.readFileSync(file, "utf8"))) : {});
  }
  return upstream.get(locale);
}

const normalize = (text) =>
  text
    .replaceAll("“", "「").replaceAll("”", "」").replaceAll("‘", "「").replaceAll("’", "」")
    .replaceAll("…", "...")
    .replace(/[：:。.]\s*$/g, "")
    .replace(/\s+/g, "")
    .trim()
    .toLowerCase();

// Match each msgid to the upstream key whose Chinese is the same text.
const zhToKey = new Map();
for (const [key, value] of Object.entries(readUpstream("zh-CN"))) {
  const key2 = normalize(value);
  if (!zhToKey.has(key2)) zhToKey.set(key2, key);
}
const msgidKeys = new Map();
for (const msgid of msgids) {
  const key = zhToKey.get(normalize(msgid));
  if (key) msgidKeys.set(msgid, key);
}

const runtimeEnFile = path.join(translationsDir, "locale-en-runtime.json");
const runtimeEn = fs.existsSync(runtimeEnFile) ? JSON.parse(fs.readFileSync(runtimeEnFile, "utf8")) : {};
const missingRuntimeEn = [...runtimeMsgids].filter((msgid) => !(msgid in runtimeEn));
if (missingRuntimeEn.length > 0) {
  throw new Error(
    `locale-en-runtime.json is missing ${missingRuntimeEn.length} msgids:\n${missingRuntimeEn
      .map((m) => `- ${m}`)
      .join("\n")}`,
  );
}
for (const msgid of runtimeMsgids) msgids.add(msgid);

const cache = fs.existsSync(cacheFile) ? JSON.parse(fs.readFileSync(cacheFile, "utf8")) : {};

function upstreamValue(locale, msgid) {
  const key = msgidKeys.get(msgid);
  if (!key) return undefined;
  const value = readUpstream(locale)[key];
  // Values that only reference another key ("@:general.off") are not usable.
  if (typeof value !== "string" || !value || value.includes("@:") || value.includes("@.")) return undefined;
  return value;
}

function machineValue(locale, msgid) {
  return cache[locale]?.[msgid];
}

function englishFor(msgid) {
  return en[msgid] ?? runtimeEn[msgid] ?? msgid;
}

// Language names always render in their own script, so a locale must never
// machine-translate another language's name.
const NATIVE_NAMES = new Set(Object.values(DISPLAY));

function translationFor(locale, msgid) {
  if (NATIVE_NAMES.has(msgid)) return msgid;
  if (locale === "zh-CN") return msgid;
  if (locale === "en-US" || locale === "en-IN") return englishFor(msgid);
  return upstreamValue(locale, msgid) ?? machineValue(locale, msgid) ?? englishFor(msgid);
}

if (process.argv.includes("--collect")) {
  // Every msgid whose translation cannot come from upstream, per locale.
  const want = {};
  for (const locale of LOCALES) {
    // English locales use the hand-written English catalog directly.
    if (locale === "en-US" || locale === "en-IN") continue;
    const todo = [...msgids].filter(
      (msgid) => !upstreamValue(locale, msgid) && !machineValue(locale, msgid),
    );
    if (todo.length > 0) want[locale] = todo;
  }
  process.stdout.write(
    `${JSON.stringify({ english: { ...en, ...runtimeEn }, wanted: want }, null, 2)}\n`,
  );
  process.exit(0);
}

function poString(value) {
  return value.replaceAll("\\", "\\\\").replaceAll('"', '\\"').replaceAll("\n", "\\n");
}

function poFile(language, lookup) {
  const entries = [...msgids]
    .sort()
    .map((msgid) => `msgid "${poString(msgid)}"\nmsgstr "${poString(lookup(msgid))}"\n`);
  return `msgid ""\nmsgstr ""\n"Content-Type: text/plain; charset=UTF-8\\n"\n"Language: ${language}\\n"\n\n${entries.join("\n")}`;
}

for (const locale of ["en-US", "zh-CN", ...LOCALES]) {
  const output = path.join(translationsDir, locale, "LC_MESSAGES", "localsend-gui-slint.po");
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, poFile(locale, (msgid) => translationFor(locale, msgid)));
}

// Rust-side lookups (runtime_strings.rs) get the same catalog as a compact table.
function rustTable() {
  const lines = [
    "pub(crate) static TABLE: &[(&str, &[(&str, &str)])] = &[",
  ];
  for (const locale of ["zh-CN", "en-US", "en-IN", ...LOCALES]) {
    const entries = [...runtimeMsgids]
      .sort()
      .map((msgid) => `        (${rustString(msgid)}, ${rustString(translationFor(locale, msgid))}),`);
    lines.push(`    (${rustString(locale)}, &[`);
    lines.push(...entries);
    lines.push("    ]),");
  }
  lines.push("];");
  lines.push("");
  return lines.join("\n");
}

// Rust rejects invisible characters (ZWNJ/ZWJ in Persian or Urdu, NBSP from
// some translators) inside string literals, so escape them.
const INVISIBLE = { "\u00a0": "\\u{a0}", "\u200b": "\\u{200b}", "\u200c": "\\u{200c}", "\u200d": "\\u{200d}", "\u200e": "\\u{200e}", "\u200f": "\\u{200f}", "\ufeff": "\\u{feff}" };

function rustString(value) {
  let escaped = value
    .replaceAll("\\", "\\\\")
    .replaceAll('"', '\\"')
    .replaceAll("\n", "\\n")
    .replaceAll("\t", "\\t");
  for (const [char, code] of Object.entries(INVISIBLE)) escaped = escaped.replaceAll(char, code);
  return `"${escaped}"`;
}

fs.writeFileSync(path.join(root, "crates", "localsend-i18n", "src", "lookup_generated.rs"), rustTable());

// The picker reads this: tag, display name, and the upstream tag used for mt.
fs.writeFileSync(
  languagesFile,
  `${JSON.stringify(
    [...LOCALES].map((locale) => ({
      locale,
      name: DISPLAY[locale],
      mt: MT_TAG[locale] ?? locale,
    })),
    null,
    2,
  )}\n`,
);

console.log(`wrote ${LOCALES.length + 2} catalogs for ${msgids.size} msgids`);
