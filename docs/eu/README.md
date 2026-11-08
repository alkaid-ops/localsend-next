<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | **Euskara** | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next Rust-n idatzitako LocalSend-ekin bateragarria den mahaigaineko bezero bat da. Protokolo ofiziala hitz egiten du, beraz, LocalSend aplikazio ofizialarekin fitxategiak deskubritzen eta transferitzen ditu bi noranzkoetan, Flutter aplikazioaren ordez mahaigaineko jatorrizko programa gisa exekutatzen den bitartean.

Sarearen aldea LocalSend Rust nukleo ofiziala da, saldu gabe. Haren gainean dagoen guztia — deabrua, RPC geruza, erretilua, terminaleko interfazea eta Slint GUI — nukleo horren inguruan eraikita dago.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Bidali | Ezarpenak |
| --- | --- |
| ![Bidali orria fitxategi, karpeta, testu eta arbel iturriekin eta inguruko gailuekin](../../docs/images/readme-send-light.png) | ![Ezarpen orria gaia, hizkuntza, sare eta transferentzia aukerak dituena](../../docs/images/readme-settings-light.png) |

## Ezaugarriak

- Aurkitzea eta transferitzea LocalSend bezero ofizialen aurka, saldutako upstream nukleoan berriro inplementatu beharrean
- Egoera-jabe kanoniko bat: deabruak ezarpenak, identitatea, gogokoak, historia, transferentziak eta TLS identitatea ditu, beraz frontend guztiek gauza bera ikusten dute
- GUI, terminaleko interfazea eta bandeja frontend independente gisa RPC lokal berean; erretiluak deabruaren eta GUI prozesuaren bizi-zikloaren jabe dira
- Fitxategi bakoitzeko denbora errealeko aurrerapena, irekitako eta erakusteko ekintzekin historia, gogokoak, gailu eskuz sartzea eta Web Share jasotzeko estekak
- 57 interfaze-hizkuntza bat datozen LocalSend-ren zerrendarekin batera, exekuzioan alda daitezkeenak edo sistemaren lokalizazioa jarraituz
- Mahaigaineko integrazio natiboa: erretilutik soilik jakinarazpenak, saioa hasten denean abiarazi, erretiluan minimizatzea, leihoak kokatzeko iraunkortasuna eta izenburu-barran sartzen den macOS leihoa
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) eta Windows (ZIP), bai x86_64 eta bai aarch64rentzat

## Arkitektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Deabrua sarearekin hitz egiten duen edo aplikazioaren egoera idazten duen prozesu bakarra da. Frontendak ez dira inoiz saltzaileen nukleoaren menpekoak eta ez dute inoiz deabrua sortzen; erretiluak egiten du. Fitxategiaren byteek ez dute RPC baino gehiago ibiltzen — deabruak bideak ebazten ditu, edukia hashak eta historia erregistratzen ditu.

| Binarioa | Rola |
| --- | --- |
| `localsendd` | Sare-zerbitzua eta aplikazio-egoera kanonikoa |
| `localsend-tray` | Daemon eta GUI bizi-zikloaren jabea, jakinarazpenak |
| `localsend-gui-slint` | GUI formala, Slint FemtoVG errendatzailearekin |
| `localsend-tui` | Terminal frontend |
| `localsend-launcher` | Erretilurik gabeko inguruneetarako abiarazle esplizitua |
| `localsend-androidd` | Erro autonomoa soilik Android jaso deabrua (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` eta `-tauri` direktorioak ataka esperimentalak dira eta erroko lan-eremutik kanpo geratzen dira RPC integrazioa osatu arte.

## Deskargatu

Kaleratze-aktiboek izen-eskema bat jarraitzen dute — `LocalSend-<version>-<platform>-<arch>.<ext>`, `linux` / `windows` / `macos` eta `x86_64` / `aarch64` plataformekin:

| Plataforma | Aktiboa |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Edozein Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt`-k bertsio bateko aktibo guztiak estaltzen ditu. CIk ekoiztutako macOS eraikuntza ad-hoc sinatuta dago, ez Garatzaile ID sinatuta edo notariodun.

## Hasiera azkarra

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

Linux paketeek `localsend-tray` abiarazten duen mahaigaineko sarrera bat instalatzen dute, deabruaren bizi-zikloaren jabea dena. Egoera-barrarik gabeko mahaigainetan, erretiluak GUI irekita mantentzen du ordez; `LocalSend --launcher`-k modu hori esplizituki hautatzen du AppImage-tik.

### Windows

Deskonprimitu eta exekutatu `LocalSend.exe`. Artxiboak GUI, deabrua, erretilua eta abiarazlea elkarren ondoan ditu; mantendu elkarrekin.

## Jakitea merezi duen jokabidea

- **Irten eragiketa bat da.** Erretilutik irteteak amaiera-puntu-esparruko `gui.pid`-n grabatutako GUI amaitzen du, jabetzako deabrua gelditzen da mugatutako denbora-muga batean eta irtengo da prozesu bat edo abian jarritako mantentze-lan bat atzean utzi gabe.
- **Jakinarazpenak erretilutik datoz.** GUI-k ez du inoiz bere jakinarazpenik argitaratzen, beraz plataforma bakoitzak bidalketa bide bakarra du.
- **Leihoaren egoera amaiera-puntu bakoitzeko da.** Kokapena amaiera-puntu horren deabru-konfigurazioaren ondoan gordetzen da, partekatutako kokapen global batean beharrean, beraz, bi instantziek ez dute leihoaren geometriaren aurka borrokatzen.
- **Deabrua autoritarioa da.** GUI-n aldatutako ezarpenak deabruari bidaltzen zaizkio eta argazki baten bidez errepikatzen dira; frontend batek ez du desbideratu litekeen kopia pribaturik gordetzen.

## Hizkuntzak

Interfazea 57 lokaletan bidaltzen da, goranzko LocalSend-ren zerrendarekin bat datorrena, txinera sinplifikatua msgid hizkuntza gisa. Aukeratu bat **Ezarpenak → Hizkuntza** atalean, edo utzi "Jarraitu sistema" aukeran OS-ko lokalizazioa erabiltzeko.

| | |
| --- | --- |
| Katalogo sortak | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI iturburu-kateak | `@tr("...")` `apps/localsend-gui-slint/ui`-n |
| Ingelesezko katalogoa | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mezuak) |
| Herdoilaren aldeko mezuak | `crates/localsend-i18n` (sortutako `lookup_generated.rs`) |
| Gora ispilua | `translations/upstream/<locale>.json` |

Birsortu katalogoak edozein UI kate aldatu ondoren:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` konprometituta dago, beraz, sortutako katalogoak lineaz kanpo erreproduzi daitezke. Zuzendu sarrera bat bertan eta berriro exekutatu sorgailua itzulpen bat konpontzeko.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Iturburutik eraikitzea

Baldintzak: Rust `1.97.1` (`rust-toolchain.toml`-k ainguratua) eta, Linux eraikitzeetarako, GTK3 eta AppIndicator garapen-goiburuak.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Plataforma paketeak:

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

Onarpen-scriptek benetako bitarrak gidatzen dituzte egoera burlatuaren ordez:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentazioa

| Dokumentua | Edukiak |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Funtzionamendua, ontziratzea eta plataformaren portaera |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Uneko probaren frogak eta egiaztatu gabeko eremu ezagunak |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Tokiko onarpen ateak |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Erregresio errepikakorrak eta portaera aldatu aurretik espero den ebidentziaren arrastoa |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Ikusizko konbergentzia egoera OpenDesign prototipoaren aurka |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android erro modulua eta bere mugak |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Saltzaileen nukleoaren upstream jatorria |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Dokumentu batzuk txineraz idatzita daude; egiaztapen-prozesuak eta konbentzioak adostutako hitzetan erregistratzen dituzte.

## Eremua

Produktu nagusia PC bezeroa da. Android euskarria erro-soilik, bururik gabeko harrera daemon bat da, esparru horretatik kanpo esplizituki sustatzen den arte — ez du GUI, ez du lehen planoko zerbitzurik abiarazten eta ez du jakinarazpen iraunkorrik argitaratzen. `crates/localsend-official-core` bytez-byte grabatutako upstream esportazioaren berdin mantentzen da, sortutako `SHA256SUMS` fitxategiaz gain.

## Kredituak

Protokoloa, hari formatua eta Rust nukleoa [localsend/localsend](https://github.com/localsend/localsend)-tik datoz; proiektu honek mahaigaineko bezero bat eraikitzen du lan horretan, protokoloa berriro inplementatu beharrean. Eskerrik asko LocalSend egile eta laguntzaileei.

Interfazeak proiektu honetarako garatutako diseinu-prototipo bati jarraitzen dio; bere erreferentzia bisuala [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)-n bizi da.

## Ekartzen

- **Akatsak eta iradokizunak**: ireki [arazo] (https://github.com/alkaid-ops/localsend-next/issues) bertsioa, plataforma, erreprodukzio urratsak eta ikusi duzunarekin. Erregistro edo pantaila-argazki batek askoz azkarrago egiten du jardutea.
- **Kodea**: tira eskaerak ongi etorriak dira. daemon/frontend muga, saltzaileen nukleoa edo ikusizko kontratua ukitzen duten aldaketak eztabaidatu behar dira lehenik; mugak [`AGENTS.md`](../../AGENTS.md) atalean erregistratzen dira.
- **Itzulpenak**: katalogoak konprometitutako itzulpen automatikoko cachetik sortzen dira, beraz, zuzenketak dira berrikusteko ekarpenik errazena.

## Lizentzia

[Apache-2.0] (LICENSE). Hirugarrenen osagaiak eta haien lizentziak [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) atalean daude zerrendatuta.
