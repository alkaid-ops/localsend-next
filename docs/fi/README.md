<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | **Suomi** | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next on LocalSend-yhteensopiva työpöytäohjelma, joka on kirjoitettu Rust. Se puhuu virallista protokollaa, joten se löytää ja siirtää tiedostoja virallisella LocalSend-sovelluksella molempiin suuntiin, samalla kun se toimii alkuperäisenä työpöytäohjelmana Flutter-sovelluksen sijaan.

Verkkopuoli on virallinen LocalSend Rust -ydin, jota myydään ennallaan. Kaikki sen yläpuolella oleva - daemon, RPC -taso, lokero, terminaalin käyttöliittymä ja Slint GUI - on rakennettu tämän ytimen ympärille.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Lähetä | Asetukset |
| --- | --- |
| ![Lähetä sivu tiedoston, kansion, tekstin ja leikepöydän lähteillä ja lähellä olevilla laitteilla](../../docs/images/readme-send-light.png) | ![Asetussivu, jossa on teema, kieli, verkko ja siirtoasetukset](../../docs/images/readme-settings-light.png) |

## Ominaisuudet

- Löytäminen ja siirto virallisia LocalSend-asiakkaita vastaan, myydyn alkupään ytimessä uudelleenkäyttöönoton sijaan
- Yksi kanoninen tilan omistaja: demonilla on asetukset, identiteetti, suosikit, historia, siirrot ja TLS-identiteetti, joten jokainen käyttöliittymä näkee saman asian
- GUI, terminaalin käyttöliittymä ja lokero itsenäisinä käyttöliittymänä saman paikallisen RPC; lokero omistaa demonin ja GUI prosessin elinkaaren
- Reaaliaikainen tiedostokohtainen edistyminen, historia avaus- ja paljastamistoiminnoilla, suosikit, manuaalinen laitesyöttö ja Web Share -vastaanottolinkit
- 57 käyttöliittymäkieltä, jotka vastaavat ylävirran LocalSend -luetteloa, vaihdettavissa ajon aikana tai järjestelmän kieliasetuksen mukaan
- Alkuperäinen työpöytäintegraatio: ilmoitukset vain tarjottimesta, käynnistys kirjautumisen yhteydessä, minimointi lokeroon, ikkunoiden sijoittelun pysyvyys ja macOS-ikkuna, joka vetäytyy otsikkopalkkiin
- Pakkaukset macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) ja Windows (ZIP), sekä x86_64 että aarch64

## Arkkitehtuuri

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemon on ainoa prosessi, joka keskustelee verkon kanssa tai kirjoittaa sovelluksen tilan. Käyttöliittymät eivät koskaan ole riippuvaisia ​​myyjän ytimestä eivätkä koskaan synnytä demonia itse; lokero tekee. Tiedostotavut eivät kulje RPC:n yli – demoni ratkaisee polut, hajauttaa sisällön ja tallentaa historian.

| Binääri | Rooli |
| --- | --- |
| `localsendd` | Verkkopalvelun ja kanonisen sovelluksen tila |
| `localsend-tray` | Daemon ja GUI elinkaaren omistaja, ilmoitukset |
| `localsend-gui-slint` | Muodollinen GUI, Slint FemtoVG renderöijalla |
| `localsend-tui` | Terminaalin käyttöliittymä |
| `localsend-launcher` | Selkeä käynnistysohjelma ympäristöihin, joissa ei ole alustaa |
| `localsend-androidd` | Itsenäinen root-vain Android vastaanottaa demon (MVP) |

`localsend-gui-egui`-, `-fltk`-, `-iced`- ja `-tauri`-hakemistot ovat kokeellisia portteja ja pysyvät juurityötilan ulkopuolella, kunnes niiden RPC-integrointi on valmis.

## Lataa

Julkaisumateriaalit noudattavat yhtä nimeämisjärjestelmää — `LocalSend-<version>-<platform>-<arch>.<ext>`, alustoilla `linux` / `windows` / `macos` ja arkkitehtuureilla `x86_64` / `aarch64`:

| Alusta | Omaisuus |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Mikä tahansa Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` kattaa jokaisen julkaisun sisällön. CI:n tuottama macOS-koontiversio on tilapäisesti allekirjoitettu, ei Developer ID -allekirjoitettu tai notaarin vahvistama.

## Pika aloitus

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

Linux-paketit asentavat työpöytämerkinnän, joka käynnistää `localsend-tray`, joka omistaa demonin elinkaaren. Pöytäkoneissa ilman tilapalkkia lokero pitää GUI auki sen sijaan; `LocalSend --launcher` valitsee kyseisen tilan nimenomaisesti AppImage:sta.

### Windows

Pura ja suorita `LocalSend.exe`. Arkisto sisältää GUI, demonin, lokero ja kantoraketti vierekkäin; pitää ne yhdessä.

## Käyttäytyminen kannattaa tietää

- **Quit on yksi toiminto.** Lopetus lokerosta lopettaa GUI:n, joka on tallennettu päätepisteen laajuiseen `gui.pid`, pysäyttää omistetun demonin rajoitetun aikakatkaisun sisällä ja poistuu jättämättä prosessia tai käynnistettyä säilytystyötä.
- **Ilmoitukset tulevat lokerosta.** GUI ei koskaan lähetä omia ilmoituksiaan, joten jokaisella alustalla on yksi toimituspolku.
- **Ikkunan tila on päätepistekohtaisesti.** Sijoitus tallennetaan kyseisen päätepisteen demonmääritysten viereen eikä jaettuun globaaliin sijaintiin, joten kaksi ilmentymää eivät riitele ikkunan geometriasta.
- **Daemon on arvovaltainen.** GUI:ssa muutetut asetukset lähetetään daemonille ja palautetaan tilannekuvan kautta; käyttöliittymä ei pidä yksityistä kopiota, joka voisi ajautua.

## Kielet

Käyttöliittymä toimitetaan 57 kielellä, jotka vastaavat ylävirran LocalSend -luetteloa, ja msgid-kielenä on yksinkertaistettu kiina. Valitse yksi kohdasta **Asetukset → Kieli** tai jätä se "Seuraa järjestelmää" -kohtaan käyttääksesi käyttöjärjestelmän kieliasetusta.

| | |
| --- | --- |
| Mukana olevat luettelot | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Käyttöliittymän lähdemerkkijonot | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| Englanninkielinen luettelo | `translations/locale-en.json` (käyttöliittymä), `translations/locale-en-runtime.json` (Rust viestit) |
| Ruosteen puoleiset viestit | `crates/localsend-i18n` (luettu `lookup_generated.rs`) |
| Ylävirran peili | `translations/upstream/<locale>.json` |

Luo luettelot uudelleen minkä tahansa käyttöliittymän merkkijonon muuttamisen jälkeen:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` on sitoutunut, joten luodut luettelot ovat toistettavissa offline-tilassa. Korjaa merkintä siellä ja suorita generaattori uudelleen käännöksen korjaamiseksi.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Rakentaminen lähteestä

Vaatimukset: Rust `1.97.1` (kiinnitetty `rust-toolchain.toml`) ja Linux -koontiversioille GTK3 ja AppIndicator -kehitysotsikot.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Alustapaketit:

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

Hyväksymisskriptit ohjaavat todellisia binaaritiedostoja pilkatun tilan sijaan:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentaatio

| Asiakirja | Sisällys |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Toiminta, pakkaus ja alustan käyttäytyminen |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Nykyiset testitodisteet ja tunnetut vahvistamattomat alueet |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Paikalliset vastaanottoportit |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Toistuvia regressioita ja todisteita, joita odotetaan ennen käyttäytymisen muuttumista |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visuaalinen konvergenssitila verrattuna OpenDesign-prototyyppiin |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android juurimoduuli ja sen rajat |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Myyjän ytimen alkupää |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Jotkut asiakirjat on kirjoitettu kiinaksi; he kirjaavat varmennusajot ja käytännöt sanoilla, joista ne on sovittu.

## Laajuus

Ensisijainen tuote on PC-asiakas. Android support is a root-only, headless receive daemon until explicitly promoted beyond that scope — it has no GUI, starts no foreground service and posts no persistent notification. `crates/localsend-official-core` pidetään tavuittain identtisenä sen tallennettujen alkupään viennin kanssa, lukuun ottamatta luotua `SHA256SUMS`-tiedostoa.

## Krediitit

Protokolla, lankamuoto ja Rust -ydin ovat peräisin [localsend/localsend](https://github.com/localsend/localsend); tämä projekti rakentaa työpöytäasiakkaan tälle työlle sen sijaan, että ottaisi protokollaa uudelleen käyttöön. Kiitos LocalSend kirjoittajille ja avustajille.

Käyttöliittymä noudattaa tätä projektia varten kehitettyä suunnitteluprototyyppiä; sen visuaalinen viite sijaitsee [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Osallistuminen

- **Virheet ja ehdotukset**: avaa [ongelma](https://github.com/alkaid-ops/localsend-next/issues), jossa kerrotaan versio, alusta, kopiointivaiheet ja näkemäsi. Loki tai kuvakaappaus nopeuttaa toimintaa paljon.
- **Koodi**: vetopyynnöt ovat tervetulleita. Muutokset, jotka koskettavat daemon/frontend-rajaa, toimittajan ydintä tai visuaalista sopimusta, tulee keskustella ensin – rajoitukset kirjataan [`AGENTS.md`](../../AGENTS.md).
- **Käännökset**: luettelot luodaan sitoutuneesta konekäännösvälimuistista, joten korjaukset ovat helpoin tarkistaa.

## Lisenssi

[Apache-2.0](../../LICENSE). Kolmannen osapuolen komponentit ja niiden lisenssit on lueteltu osoitteessa [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
