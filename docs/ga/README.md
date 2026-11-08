<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | **Gaeilge** | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



Is cliant deisce atá comhoiriúnach le LocalSend é localsend-next agus atá scríofa i Rust. Labhraíonn sé an prótacal oifigiúil, mar sin aimsíonn sé agus aistríonn sé comhaid leis an app oifigiúil @ @ 1@@ sa dá threo, agus é ag rith mar chlár deisce dúchais in ionad aip Flutter.

Is é an taobh líonra an croí oifigiúil @ @ 0@@ @ 2@@, díoltóir gan athrú. Tá gach rud os a chionn - deamhan, ciseal @ 4 @ @, tráidire, UI críochfoirt agus an @ @ 1@@ @ 3@@ - tógtha timpeall ar an gcroí sin.

## Taispeántas

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Seol | Socruithe |
| --- | --- |
| ![Seol leathanach leis an gcomhad, fillteán, téacs agus foinsí gearrthaisce agus gléasanna in aice láimhe](../../docs/images/readme-send-light.png) | ![Leathanach socruithe le téama, teanga, roghanna líonra agus aistrithe](../../docs/images/readme-settings-light.png) |

## Gnéithe

- Fionnachtain agus aistriú i gcoinne cliant oifigiúil @ 0@@, ar an gcroí in aghaidh an tsrutha díoltóra seachas a chur i bhfeidhm
- Úinéir stáit chanónach amháin: coinníonn an deamhan socruithe, céannacht, ceanáin, stair, aistrithe agus aitheantas TLS, mar sin feiceann gach tosaigh an rud céanna
- GUI, UI críochfoirt agus tráidire mar aghaidheanna neamhspleácha thar an gcéanna áitiúil RPC; is leis an tráidire deamhan agus saolré próisis GUI
- Dul chun cinn in aghaidh an chomhaid i bhfíor-am, stair le gníomhartha oscailte agus nochta, ceanáin, iontráil gléas láimhe, agus naisc gheobhaidh Web Share
- 57 teanga comhéadain ag teacht le liosta in aghaidh an tsrutha LocalSend, inaistrithe ag am rite nó ag leanúint locale an chórais
- Comhtháthú deisce dúchais: fógraí ón tráidire amháin, seol ag logáil isteach, íoslaghdú go dtí an tráidire, marthanacht socrúcháin fuinneoige, agus fuinneog macOS a tharraingíonn isteach ina bharra teidil
- Pacáistiú le haghaidh macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) agus Windows (ZIP), don dá x86_64 agus aarch64

## Ailtireacht

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Is é an deamhan an t-aon phróiseas a labhraíonn leis an líonra nó a scríobhann staid iarratais. Ní bhraitheann aghaidheanna ar an gcroí díoltóra agus ní sceithfidh siad an deamhan iad féin; a dhéanann an tráidire. Ní thaistealaíonn beart comhaid thar @ @ 0@@ - ​​réitíonn an deamhan cosáin, hashes ábhar agus taifeadann an stair.

| Dénártha | Ról |
| --- | --- |
| `localsendd` | Seirbhís líonra agus staid iarratais chanónach |
| `localsend-tray` | Úinéir saolré Deamhan agus GUI, fógraí |
| `localsend-gui-slint` | An GUI, Slint foirmiúil leis an rindreálaí FemtoVG |
| `localsend-tui` | Aghaidh críochfoirt |
| `localsend-launcher` | Tosaitheoir soiléir le haghaidh timpeallachtaí gan tráidire |
| `localsend-androidd` | Fréamh standalone amháin Android gheobhaidh deamhan (MVP) |

Is calafoirt thurgnamhacha iad na heolairí `localsend-gui-egui`, `-fltk`, `-iced` agus @ 4@@ agus fanann siad lasmuigh den fhréamhspás oibre go dtí go mbeidh a gcomhtháthú RPC críochnaithe.

## Íosluchtaigh

Leanann sócmhainní scaoileadh scéim ainmniúcháin amháin - `LocalSend-<version>-<platform>-<arch>.<ext>`, le hardáin `linux` / `windows` / `macos` agus ailtireachtaí `x86_64` / `aarch64`:

| Ardán | Sócmhainn |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ar bith Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

Clúdaíonn `SHA256SUMS.txt` gach sócmhainn in eisiúint. Tá an tógáil macOS arna tháirgeadh ag CI sínithe ad-hoc, ní sínithe nó notaraithe le hAitheantas Forbróra.

## Tús tapa

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

Suiteálann pacáistí Linux iontráil deisce a sheolann `localsend-tray`, ar leis an saolré deamhan. Ar deasc gan barra stádais coinníonn an tráidire an GUI oscailte ina ionad; Roghnaíonn `LocalSend --launcher` an modh sin go sainráite ón AppImage.

### Windows

Unzip agus rith `LocalSend.exe`. Coinníonn an chartlann an GUI, an deamhan, an tráidire agus an tosaitheoir taobh le taobh; coinnigh le chéile iad.

## Iompar is fiú a fhios

- **Is oibríocht amháin é éirí as an tráidire.** Nuair a éiríonn sé as an tráidire cuirtear deireadh leis an GUI a thaifeadtar sa chríochphointe @ 1@@, stopann sé an deamhan faoi úinéireacht laistigh de theorainn ama teorantach, agus téann sé amach gan próiseas ná post coinnithe beo seolta a fhágáil taobh thiar de.
- **Tagann fógraí ón tráidire.** Ní phostálann an GUI a chuid fógraí féin riamh, mar sin tá cosán seachadta amháin ag gach ardán.
- **Tá staid na fuinneoige in aghaidh an chríochphointe.** Stóráiltear an socrúchán in aice le cumraíocht an deamhan don chríochphointe sin seachas i suíomh domhanda comhroinnte, mar sin ní bhíonn dhá chás ag dul i ngleic le céimseata na bhfuinneog.
- **Tá an deamhan údarásach.** Cuirtear socruithe a athraíodh sa GUI faoi bhráid an deamhan agus cuirtear macalla ar ais trí léargas; ní choinníonn tosaigh aon chóip phríobháideach a d'fhéadfadh a bheith ag imeacht.

## Teangacha

Seoltar an comhéadan i 57 logán, ag teacht le liosta LocalSend in aghaidh an tsrutha, le Sínis Simplithe mar an teanga msgid. Roghnaigh ceann faoi **Socruithe → Teanga**, nó fág ar "Follow system" é chun an locale OS a úsáid.

| | |
| --- | --- |
| Catalóga cuachta | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Teaghráin foinse Chomhéadain | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| catalóg Béarla | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust teachtaireachtaí) |
| Teachtaireachtaí meirge-taobh | `crates/localsend-i18n` (gineadh `lookup_generated.rs`) |
| Scáthán in aghaidh srutha | `translations/upstream/<locale>.json` |

Athghin na catalóga tar éis aon teaghrán Chomhéadain a athrú:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

Tá `locale-machine-cache.json` tiomanta, mar sin tá na catalóga ginte in-atáirgthe as líne. Ceartaigh iontráil ansin agus athrith an gineadóir chun aistriúchán a shocrú.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Tógáil ón bhfoinse

Riachtanais: Rust `1.97.1` (pinned by `rust-toolchain.toml`) agus, le haghaidh tógálaithe Linux, na ceanntásca forbartha GTK3 agus AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pacáistí ardán:

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

Tiomáineann scripteanna glactha na fíordhénártha in ionad stát magaidh:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Doiciméadúchán

| Doiciméad | Clár ábhair |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Oibríocht, pacáistiú agus iompar ardán |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Fianaise tástála reatha agus réimsí anaithnide nach bhfuil fíoraithe |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Geataí glactha áitiúla |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Aischéimniú athfhillteach agus an rian fianaise a bhfuiltear ag súil leis sula n-athraítear iompar |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Staid an chóineasaithe amhairc i gcoinne an fhréamhshamhail OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | An modúl fréimhe Android agus a theorainneacha |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Bunús in aghaidh srutha an chroí díoltóra |

Tá an README seo foilsithe i ngach ceann de na 57 teanga comhéadain - roghnaigh ceann amháin ón lascóir ag an mbarr. Gintear na haistriúcháin ó `README.md` le `scripts/translate-readme.mjs`; cuirtear ceartúcháin isteach [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Tá roinnt doiciméad scríofa i Sínis; taifeadann siad ritheanna agus gnásanna fíoraithe sna focail ar comhaontaíodh iad.

## Raon feidhme

Is é an príomhtháirge an cliant PC. Is deamhan fréamh-amháin amháin í tacaíocht @ 2@@ go dtí go gcuirtear chun cinn go sainráite í thar an raon feidhme sin - níl aon @ @ 0@@ aige, ní thosaíonn aon seirbhís tulra agus ní chuireann sé post ar fhógra leanúnach. Coinnítear `crates/localsend-official-core` byte-for-byte atá comhionann lena onnmhairiú réamhtheachtacha taifeadta, seachas a chomhad @ 4@@ ginte.

## Creidmheasanna

Tagann an prótacal, an fhormáid sreang agus an croí Rust [localsend/localsend](https://github.com/localsend/localsend); tógann an tionscadal seo cliant deisce ar an obair sin seachas an prótacal a chur i bhfeidhm arís. Buíochas leis na húdair agus na rannpháirtithe LocalSend.

Leanann an comhéadan fréamhshamhail dearaidh a forbraíodh don tionscadal seo; tá a thagairt amhairc ina chónaí i [@@@@](../../apps/localsend-gui-slint/reference/open-design/).

## Ag cur leis

- **Bugs agus moltaí**: oscail [saincheist] (https://github.com/alkaid-ops/localsend-next/issues) leis an leagan, an t-ardán, na céimeanna atáirgeadh agus an méid a chonaic tú. Fágann logáil nó gabháil scáileáin go bhfuil sé i bhfad níos tapúla gníomhú air.
- **Cód**: tá fáilte roimh iarratais ar tharraingt. Ba cheart athruithe a bhaineann le teorainn daemon/frontend, leis an gcroí díoltóra, nó leis an gconradh amhairc a phlé ar dtús — déantar na srianta a thaifeadadh in [@@@](../../AGENTS.md).
- **Aistriúcháin**: gintear na catalóga ón taisce tiomnaithe meaisín-aistriúcháin, mar sin is iad na ceartúcháin is fusa le hathbhreithniú a dhéanamh.

## Ceadúnas

[Apache-2.0](../../LICENSE). Tá comhpháirteanna tríú páirtí agus a gceadúnais liostaithe in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
