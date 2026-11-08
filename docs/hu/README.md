<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | **Magyar** | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



A localsend-next egy LocalSend-kompatibilis asztali kliens, amely Rust nyelven íródott. A hivatalos protokollt beszéli, így mindkét irányban felfedezi és továbbítja a fájlokat a hivatalos LocalSend alkalmazással, miközben natív asztali programként fut a Flutter alkalmazás helyett.

A hálózati oldal a hivatalos LocalSend Rust mag, amelyet változatlan formában szállítanak. Minden, ami felette van – démon, RPC réteg, tálca, terminál UI és a Slint GUI – e mag köré épül.

## Demó

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Elküld | Beállítások elemre |
| --- | --- |
| ![Oldal küldése a fájllal, mappával, szöveg- és vágólap-forrásokkal és közeli eszközökkel](../../docs/images/readme-send-light.png) | ![Beállítások oldal témával, nyelvvel, hálózattal és átviteli lehetőségekkel](../../docs/images/readme-settings-light.png) |

## Funkciók

- Felfedezés és átvitel a hivatalos LocalSend kliensekkel szemben, a szállított upstream magon, nem pedig újbóli megvalósítás
- Egy kanonikus állapottulajdonos: a démon tartalmazza a beállításokat, az identitást, a kedvenceket, az előzményeket, az átviteleket és a TLS identitást, így minden frontend ugyanazt látja
- GUI, terminál felhasználói felület és tálca, mint független frontendek ugyanazon a helyi RPC felett; a tálca birtokolja a démont és a GUI folyamat életciklusát
- Valós idejű fájlonkénti haladás, előzmények megnyitási és felfedési műveletekkel, kedvencek, kézi eszközbevitel és webes megosztás fogadási linkek
- 57 interfésznyelv, amely megfelel az upstream LocalSend listájának, váltható futás közben vagy a rendszer területi beállításai szerint
- Natív asztali integráció: értesítések csak a tálcáról, indítás bejelentkezéskor, kicsinyítés a tálcára, ablakelhelyezési állandóság és egy macOS ablak, amely berajzolódik a címsorba
- Csomagolás macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) és Windows (ZIP), x86_64 és aarch64 számára

## Építészet

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

A démon az egyetlen olyan folyamat, amely a hálózattal kommunikál vagy alkalmazásállapotot ír. A frontendek soha nem függenek a szállított magtól, és soha nem generálják maguk a démont; a tálca igen. A fájlbájtok nem haladnak RPC felett – a démon feloldja az elérési utakat, kivonatolja a tartalmat és rögzíti az előzményeket.

| Bináris | Szerep |
| --- | --- |
| `localsendd` | A hálózati szolgáltatás és a kanonikus alkalmazás állapota |
| `localsend-tray` | Daemon és GUI életciklus tulajdonos, értesítések |
| `localsend-gui-slint` | A formális GUI, Slint a FemtoVG rendererrel |
| `localsend-tui` | Terminál frontend |
| `localsend-launcher` | Explicit indító tálca nélküli környezetekhez |
| `localsend-androidd` | Önálló root-csak Android fogadó démon (MVP) |

A `localsend-gui-egui`, `-fltk`, `-iced` és `-tauri` könyvtárak kísérleti portok, és a gyökér munkaterületen kívül maradnak, amíg az RPC integrációjuk be nem fejeződik.

## Letöltés

A kiadási eszközök egy elnevezési sémát követnek – `LocalSend-<version>-<platform>-<arch>.<ext>`, `linux` / `windows` / `macos` platformokkal és `x86_64` / `aarch64` architektúrákkal:

| Platform | Eszköz |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Bármilyen Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

A `SHA256SUMS.txt` a kiadás minden elemére vonatkozik. A CI által készített macOS build ad-hoc aláírású, nem pedig a fejlesztői azonosító aláírása vagy közjegyzői hitelesítése.

## Gyors kezdés

### macOS

```bash
open LocalSend-<version>-macos-<arch>.dmg
# drag LocalSend.app into Applications
```

### Debian / Ubuntu

```bash
sudo dpkg -i LocalSend-<version>-linux-<arch>.deb
```

### Fedora / RHEL

```bash
sudo rpm -i LocalSend-<version>-linux-<arch>.rpm
```

### AppImage

```bash
chmod +x LocalSend-<version>-linux-<arch>.AppImage
./LocalSend-<version>-linux-<arch>.AppImage
```

A Linux csomagok telepítenek egy asztali bejegyzést, amely elindítja a `localsend-tray` programot, amely a démon életciklusának tulajdonosa. Állapotsor nélküli asztali számítógépeken a tálca a GUI helyett nyitva tartja; `LocalSend --launcher` ezt a módot kifejezetten a AppImage közül választja ki.

### Windows

Csomagolja ki és futtassa a `LocalSend.exe`. Az archívum egymás mellett tartja a GUI, a démont, a tálcát és az indítót; tartsa őket együtt.

## Viselkedés, amit érdemes tudni

- **A kilépés egy művelet.** A tálcáról történő kilépés leállítja a GUI végpont-hatókörű `gui.pid`-ban rögzített GUI-t, leállítja a tulajdonolt démont egy meghatározott időkorláton belül, és kilép anélkül, hogy folyamatot vagy elindított életben tartási feladatot hagyna hátra.
- **Az értesítések a tálcáról érkeznek.** A GUI soha nem tesz közzé saját értesítéseket, így minden platformnak egyetlen kézbesítési útvonala van.
- **Az ablak állapota végpontonként értendő.** Az elhelyezést a rendszer az adott végpont démonkonfigurációja mellett tárolja, nem pedig egy megosztott globális helyen, így két példány nem vesz részt az ablakgeometrián.
- **A démon a mérvadó.** A GUI-ban módosított beállításokat a rendszer elküldi a démonnak, és egy pillanatfelvételen keresztül visszaviszi; a frontend nem tart magán példányt, amely elsodródhatna.

## Nyelvek

Az interfész 57 nyelven érkezik, amelyek megfelelnek a LocalSend felfelé irányuló listájának, és az egyszerűsített kínai az msgid nyelv. Válasszon egyet a **Beállítások → Nyelv** alatt, vagy hagyja a "Rendszer követése" opciót az operációs rendszer területi beállításának használatához.

| | |
| --- | --- |
| Csomagolt katalógusok | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI forráskarakterláncok | `@tr("...")` itt: `apps/localsend-gui-slint/ui` |
| angol katalógus | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust üzenetek) |
| Rozsdaoldali üzenetek | `crates/localsend-i18n` (generált `lookup_generated.rs`) |
| Felfelé irányuló tükör | `translations/upstream/<locale>.json` |

Bármely UI karakterlánc megváltoztatása után generálja újra a katalógusokat:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

A `locale-machine-cache.json` elkötelezett, így a létrehozott katalógusok offline reprodukálhatók. Javítson ki egy bejegyzést, és futtassa újra a generátort a fordítás javításához.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Építés a forrásból

Követelmények: Rust `1.97.1` (`rust-toolchain.toml` rögzíti) és Linux buildekhez a GTK3 és AppIndicator fejlesztési fejléc.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platform csomagok:

```bash
# macOS (on a matching host)
cd apps/localsend-gui-slint && PACKAGE_VERSION=0.1.3 ./package-macos.sh

# Linux
./scripts/package-linux.sh x86_64      # deb + tarball
./scripts/package-rpm.sh x86_64        # rpm
./scripts/package-appimage.sh x86_64   # AppImage

# Windows (PowerShell)
./scripts/package-windows.ps1 -Architecture x64
```

Az elfogadó szkriptek a valódi binárisokat hajtják a gúnyolt állapot helyett:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentáció

| Dokumentum | Tartalom |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Működés, csomagolás és platform viselkedése |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Jelenlegi vizsgálati bizonyítékok és ismert nem ellenőrzött területek |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Helyi átvételi kapuk |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Ismétlődő regressziók és a viselkedés megváltoztatása előtt várható bizonyítékok nyomvonala |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Vizuális konvergencia állapota az OpenDesign prototípussal szemben |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | A Android gyökérmodul és korlátai |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | A szállított mag felfelé irányuló származása |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Egyes dokumentumok kínai nyelven készültek; az ellenőrzési lefutásokat és a konvenciókat azokkal a szavakkal rögzítik, amelyekben megállapodtak.

## Hatókör

Az elsődleges termék a PC-kliens. A Android támogatás egy root-csak, fej nélküli fogadási démon mindaddig, amíg kifejezetten nem léptetik elő ezen a hatókörön túl – nincs GUI, nem indít előtérbeli szolgáltatást, és nem küld állandó értesítést. A `crates/localsend-official-core` fájl byte-onként megegyezik a rögzített upstream exporttal, kivéve a generált `SHA256SUMS` fájlt.

## Kredit

A protokoll, a vezetékes formátum és a Rust mag a [localsend/localsend](https://github.com/localsend/localsend) forrásból származik; ez a projekt egy asztali klienst épít erre a munkára ahelyett, hogy újra implementálná a protokollt. Köszönet a LocalSend szerzőknek és közreműködőknek.

Az interfész egy ehhez a projekthez kifejlesztett tervezési prototípust követ; vizuális referenciája a [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) helyeken található.

## Hozzájárulás

- **Hibák és javaslatok**: nyisson meg egy [problémát](https://github.com/alkaid-ops/localsend-next/issues) a verzióval, a platformmal, a reprodukálási lépésekkel és a látottakkal. Egy napló vagy képernyőkép sokkal gyorsabbá teszi a cselekvést.
- **Kód**: várjuk a lehívási kéréseket. A daemon/frontend határt, a szállítói magot vagy a vizuális szerződést érintő változtatásokat először meg kell vitatni – a megszorításokat a [`AGENTS.md`](../../AGENTS.md) helyen kell rögzíteni.
- **Fordítások**: a katalógusokat a lefoglalt gépi fordítási gyorsítótárból állítják elő, így az ott végzett javítások a legkönnyebben áttekinthetők.

## Engedély

[Apache-2.0](../../LICENSE). A harmadik féltől származó összetevők és licenceik a [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) helyen találhatók.
