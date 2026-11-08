#!/usr/bin/env node
// Fills translations/locale-machine-cache.json with machine translations for the
// msgids that upstream LocalSend does not translate. Requires network access.
//
//   node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
//   node scripts/translate-slint-locales.mjs            # fills the cache
//   node scripts/update-slint-translations.mjs          # writes the .po files
//
// Translations come from the Chrome translate endpoint, which accepts a batch of
// `q=` parameters per request. Results are cached per locale and msgid, so a
// rerun only fetches what is still missing; edit the cache by hand to correct a
// translation and rerun the generator.
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const appDir = path.join(root, "apps", "localsend-gui-slint");
const translationsDir = path.join(appDir, "translations");
const cacheFile = path.join(translationsDir, "locale-machine-cache.json");
const languagesFile = path.join(translationsDir, "languages.json");

const BATCH = 12;
const DELAY_MS = 900;
const ATTEMPTS = 3;

const { english, wanted } = JSON.parse(
  fs.readFileSync("/tmp/collect.json", "utf8"),
);
const languages = JSON.parse(fs.readFileSync(languagesFile, "utf8"));
const cache = fs.existsSync(cacheFile) ? JSON.parse(fs.readFileSync(cacheFile, "utf8")) : {};

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function translateBatch(source, target, texts) {
  const query = texts.map((text) => `q=${encodeURIComponent(text)}`).join("&");
  const url = `https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=${source}&tl=${target}&${query}`;
  for (let attempt = 1; attempt <= ATTEMPTS; attempt += 1) {
    try {
      // curl honours https_proxy/all_proxy, which Node's fetch ignores.
      const body = execFileSync(
        "curl",
        ["-sS", "--fail", "--max-time", "30", "-A", "Mozilla/5.0", url],
        { encoding: "utf8" },
      );
      const parsed = JSON.parse(body);
      // With several q= parameters the endpoint answers one entry per text;
      // a single trailing item is returned unwrapped.
      const list = Array.isArray(parsed) ? parsed : [parsed];
      if (list.length !== texts.length) {
        throw new Error(`expected ${texts.length} results, got ${list.length}`);
      }
      return list.map((item) => (typeof item === "string" ? item : item?.[0] ?? ""));
    } catch (error) {
      if (attempt === ATTEMPTS) throw error;
      await sleep(DELAY_MS * attempt);
    }
  }
  throw new Error("unreachable");
}

function save() {
  fs.writeFileSync(cacheFile, `${JSON.stringify(cache, null, 2)}\n`);
}

let done = 0;
for (const language of languages) {
  const locale = language.locale;
  const pending = (wanted[locale] ?? []).filter((msgid) => !cache[locale]?.[msgid]);
  if (pending.length === 0) continue;
  cache[locale] ??= {};
  for (let i = 0; i < pending.length; i += BATCH) {
    const batch = pending.slice(i, i + BATCH);
    // Placeholders are `{}`, which machine translators mangle. Send a token
    // they keep intact and restore it on the way back.
    const texts = batch.map((msgid) => english[msgid].replaceAll("{}", "{0}"));
    const translated = await translateBatch("en", language.mt, texts);
    batch.forEach((msgid, index) => {
      // Some words (brand names, "System" in German) legitimately translate to
      // the English text; record whatever came back so the run terminates.
      if (translated[index]) {
        cache[locale][msgid] = translated[index].replaceAll("{0}", "{}");
      }
    });
    save();
    done += batch.length;
    process.stdout.write(`\r${locale}: ${Math.min(i + BATCH, pending.length)}/${pending.length} (${done} total)`);
    await sleep(DELAY_MS);
  }
  process.stdout.write("\n");
}
save();
console.log(`cache now holds ${Object.values(cache).reduce((sum, value) => sum + Object.keys(value).length, 0)} translations`);
