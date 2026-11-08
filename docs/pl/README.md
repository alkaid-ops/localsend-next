<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | **Polski** | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next to klient stacjonarny zgodny z LocalSend, napisany w Rust. Obsługuje oficjalny protokół, więc wykrywa i przesyła pliki za pomocą oficjalnej aplikacji LocalSend w obu kierunkach, działając jako natywny program komputerowy, a nie aplikacja Flutter.

Strona sieciowa to oficjalny rdzeń LocalSend Rust, sprzedawany w niezmienionej formie. Wszystko ponad nim — demon, warstwa RPC, taca, interfejs użytkownika terminala i Slint GUI — jest zbudowane wokół tego rdzenia.

## Demonstracja

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Wysłać | Ustawienia |
| --- | --- |
| ![Wyślij stronę ze źródłami plików, folderów, tekstu i schowka oraz urządzeniami w pobliżu](../../docs/images/readme-send-light.png) | ![Strona ustawień z motywem, językiem, siecią i opcjami przesyłania](../../docs/images/readme-settings-light.png) |

## Funkcje

- Wykrywanie i transfer do oficjalnych klientów LocalSend na dostarczonym rdzeniu nadrzędnym, a nie na ponownej implementacji
- Jeden właściciel stanu kanonicznego: demon przechowuje ustawienia, tożsamość, ulubione, historię, transfery i tożsamość TLS, więc każdy frontend widzi to samo
- GUI, interfejs terminala i zasobnik jako niezależne nakładki na tym samym lokalnym RPC; taca jest właścicielem demona i GUI cyklu życia procesu
- Postęp w czasie rzeczywistym dla poszczególnych plików, historia z akcjami otwierania i ujawniania, ulubione, ręczne wprowadzanie danych o urządzeniu i udostępnianie sieciowe do otrzymywania łączy
- 57 języków interfejsu odpowiadających liście LocalSend, przełączane w czasie wykonywania lub zgodnie z ustawieniami regionalnymi systemu
- Natywna integracja z pulpitem: powiadomienia tylko z zasobnika, uruchamiane przy logowaniu, minimalizowanie do zasobnika, trwałość rozmieszczenia okien i okno macOS wciągane na pasek tytułowy
- Opakowanie dla macOS (DMG/ZIP), Linux (deb, obr./min, AppImage, tarball) i Windows (ZIP), zarówno dla x86_64, jak i aarch64

## Architektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Demon jest jedynym procesem, który komunikuje się z siecią lub zapisuje stan aplikacji. Frontendy nigdy nie zależą od dostarczonego rdzenia i nigdy nie uruchamiają samego demona; taca tak. Bajty plików nie przechodzą przez RPC — demon rozpoznaje ścieżki, miesza zawartość i zapisuje historię.

| Dwójkowy | Rola |
| --- | --- |
| `localsendd` | Usługa sieciowa i stan aplikacji kanonicznej |
| `localsend-tray` | Właściciel cyklu życia demona i GUI, powiadomienia |
| `localsend-gui-slint` | Formalne GUI, Slint z rendererem FemtoVG |
| `localsend-tui` | Nakładka terminala |
| `localsend-launcher` | Jawny program uruchamiający dla środowisk bez tacy |
| `localsend-androidd` | Samodzielny demon odbioru Android, tylko dla roota (MVP) |

Katalogi `localsend-gui-egui`, `-fltk`, `-iced` i `-tauri` są portami eksperymentalnymi i pozostają poza głównym obszarem roboczym do czasu zakończenia integracji RPC.

## Pobierać

Zasoby wersji mają jeden schemat nazewnictwa — `LocalSend-<version>-<platform>-<arch>.<ext>`, z platformami `linux` / `windows` / `macos` i architekturami `x86_64` / `aarch64`:

| Platforma | Zaleta |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Dowolny Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` obejmuje każdy zasób w wydaniu. Kompilacja macOS wyprodukowana przez CI jest podpisana ad hoc, a nie podpisana lub potwierdzona notarialnie identyfikatorem programisty.

## Szybki start

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

Pakiety Linux instalują wpis na pulpicie, który uruchamia `localsend-tray`, który jest właścicielem cyklu życia demona. Zamiast tego na komputerach stacjonarnych bez paska stanu taca GUI pozostaje otwarta; `LocalSend --launcher` wybiera ten tryb jawnie z AppImage.

### Windows

Rozpakuj i uruchom `LocalSend.exe`. Archiwum zawiera GUI, demona, tacę i program uruchamiający obok siebie; trzymaj je razem.

## Zachowanie, które warto poznać

- **Zakończenie to jedna operacja.** Wyjście z zasobnika kończy GUI zapisane w `gui.pid` o zasięgu punktu końcowego, zatrzymuje posiadanego demona w określonym czasie i kończy działanie bez pozostawiania procesu lub uruchomionego zadania utrzymywania aktywności.
- **Powiadomienia pochodzą z zasobnika.** GUI nigdy nie publikuje własnych powiadomień, więc każda platforma ma jedną ścieżkę dostawy.
- **Stan okna dotyczy punktu końcowego.** Umiejscowienie jest przechowywane obok konfiguracji demona dla tego punktu końcowego, a nie we współdzielonej lokalizacji globalnej, więc dwie instancje nie kłócą się o geometrię okna.
- **Demon jest autorytatywny.** Ustawienia zmienione w GUI są przesyłane do demona i odbijane echem w migawce; frontend nie przechowuje żadnych prywatnych kopii, które mogłyby dryfować.

## Języki

Interfejs jest dostępny w 57 lokalizacjach, zgodnych z listą LocalSend, z chińskim uproszczonym jako językiem msgid. Wybierz jedno w **Ustawienia → Język** lub pozostaw opcję „Śledź system”, aby używać ustawień regionalnych systemu operacyjnego.

| | |
| --- | --- |
| Katalogi w pakiecie | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Ciągi źródłowe interfejsu użytkownika | `@tr("...")` w `apps/localsend-gui-slint/ui` |
| Katalog angielski | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust wiadomości) |
| Wiadomości po stronie rdzy | `crates/localsend-i18n` (wygenerowano `lookup_generated.rs`) |
| Lustro górne | `translations/upstream/<locale>.json` |

Wygeneruj ponownie katalogi po zmianie dowolnego ciągu interfejsu użytkownika:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` jest zatwierdzona, więc wygenerowane katalogi można odtwarzać w trybie offline. Popraw tam wpis i uruchom ponownie generator, aby naprawić tłumaczenie.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Budowanie ze źródła

Wymagania: Rust `1.97.1` (przypięte przez `rust-toolchain.toml`) oraz, w przypadku kompilacji Linux, nagłówki rozwojowe GTK3 i AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pakiety platformy:

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

Skrypty akceptacji sterują prawdziwymi plikami binarnymi zamiast wyśmiewanym stanem:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentacja

| Dokument | Zawartość |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Działanie, opakowanie i zachowanie platformy |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktualne dowody testowe i znane, niezweryfikowane obszary |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokalne bramki akceptacji |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Powtarzające się regresje i ślady dowodów oczekiwane przed zmianą zachowania |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Stan zbieżności wizualnej względem prototypu OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Moduł główny Android i jego ograniczenia |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Pochodzenie dostarczonego rdzenia z góry |

Ten plik README jest opublikowany we wszystkich 57 językach interfejsu — wybierz jeden z przełącznika na górze. Tłumaczenia są generowane z `README.md` przez `scripts/translate-readme.mjs`; poprawki wchodzą w [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Niektóre dokumenty są napisane w języku chińskim; zapisują przebiegi weryfikacyjne i konwencje słowami, w jakich zostały uzgodnione.

## Zakres

Podstawowym produktem jest klient PC. Wsparcie Android jest bezgłowym demonem odbierania przeznaczonym tylko dla roota, dopóki nie zostanie wyraźnie awansowany poza ten zakres — nie ma GUI, nie uruchamia żadnej usługi na pierwszym planie i nie wysyła żadnych trwałych powiadomień. `crates/localsend-official-core` jest zachowywany bajt po bajcie identycznie jak zarejestrowany eksport w górę, z wyjątkiem wygenerowanego pliku `SHA256SUMS`.

## Podziękowania

Protokół, format przewodu i rdzeń Rust pochodzą z [localsend/localsend](https://github.com/localsend/localsend); ten projekt buduje klienta stacjonarnego na podstawie tej pracy, zamiast ponownie implementować protokół. Dziękujemy autorom i współpracownikom LocalSend.

Interfejs jest zgodny z prototypem opracowanym na potrzeby tego projektu; jego wizualne odniesienie znajduje się w [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Wkład

- **Błędy i sugestie**: otwórz [problem](https://github.com/alkaid-ops/localsend-next/issues) z wersją, platformą, etapami odtwarzania i tym, co widziałeś. Dziennik lub zrzut ekranu znacznie przyspieszają działanie.
- **Kod**: żądania ściągnięcia są mile widziane. Zmiany, które dotykają granicy daemon/frontend, dostarczonego rdzenia lub kontraktu wizualnego, należy omówić w pierwszej kolejności — ograniczenia są zapisywane w [`AGENTS.md`](../../AGENTS.md).
- **Tłumaczenia**: katalogi są generowane z zapisanej pamięci podręcznej tłumaczeń maszynowych, więc poprawki w nich zawarte są najłatwiejszym rodzajem wkładu do przeglądu.

## Licencja

[Apache-2.0](../../LICENSE). Komponenty innych firm i ich licencje są wymienione w [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
