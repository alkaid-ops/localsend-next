<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | **Français** | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next est un client de bureau compatible LocalSend écrit en Rust. Il parle le protocole officiel, il découvre et transfère donc des fichiers avec l'application officielle LocalSend dans les deux sens, tout en s'exécutant comme un programme de bureau natif au lieu d'une application Flutter.

Le côté réseau est le noyau officiel LocalSend Rust, vendu inchangé. Tout ce qui se trouve au-dessus - le démon, la couche RPC, la barre d'état, l'interface utilisateur du terminal et le Slint GUI - est construit autour de ce noyau.

## Démo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Envoyer | Paramètres |
| --- | --- |
| ![Envoyer la page avec les sources du fichier, du dossier, du texte et du presse-papiers et les appareils à proximité] (docs/images/readme-send-light.png) | ![Page de paramètres avec options de thème, de langue, de réseau et de transfert] (docs/images/readme-settings-light.png) |

## Fonctionnalités

- Découverte et transfert vers des clients LocalSend officiels, sur le noyau amont vendu plutôt qu'une ré-implémentation
- Un propriétaire d'état canonique : le démon détient les paramètres, l'identité, les favoris, l'historique, les transferts et l'identité TLS, donc chaque interface voit la même chose
- GUI, interface utilisateur du terminal et plateau en tant qu'interfaces indépendantes sur le même RPC local ; le plateau possède le cycle de vie du démon et du processus GUI
- Progression en temps réel par fichier, historique avec actions d'ouverture et de révélation, favoris, saisie manuelle de l'appareil et liens de réception Web Share
- 57 langues d'interface correspondant à la liste de LocalSend en amont, commutables au moment de l'exécution ou en fonction des paramètres régionaux du système
- Intégration native du bureau : notifications depuis la barre d'état uniquement, lancement à la connexion, réduction dans la barre d'état, persistance du placement de la fenêtre et fenêtre macOS qui s'affiche dans sa barre de titre
- Packaging pour macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) et Windows (ZIP), pour x86_64 et aarch64

## Architecture

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Le démon est le seul processus qui communique avec le réseau ou écrit l'état de l'application. Les frontaux ne dépendent jamais du noyau vendu et ne génèrent jamais le démon eux-mêmes ; le plateau le fait. Les octets du fichier ne parcourent pas RPC — le démon résout les chemins, hache le contenu et enregistre l'historique.

| Binaire | Rôle |
| --- | --- |
| `localsendd` | Service réseau et état canonique de l’application |
| `localsend-tray` | Démon et propriétaire du cycle de vie GUI, notifications |
| `localsend-gui-slint` | Le formel GUI, Slint avec le moteur de rendu FemtoVG |
| `localsend-tui` | Interface du terminal |
| `localsend-launcher` | Lanceur explicite pour les environnements sans plateau |
| `localsend-androidd` | Démon de réception Android autonome root uniquement (MVP) |

Les répertoires `localsend-gui-egui`, `-fltk`, `-iced` et `-tauri` sont des ports expérimentaux et restent en dehors de l'espace de travail racine jusqu'à ce que leur intégration RPC soit terminée.

## Télécharger

Les ressources de version suivent un schéma de dénomination — `LocalSend-<version>-<platform>-<arch>.<ext>`, avec les plateformes `linux` / `windows` / `macos` et les architectures `x86_64` / `aarch64` :

| Plate-forme | Actif |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| N'importe quel Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` couvre tous les actifs d'une version. La version macOS produite par CI est signée ad hoc, et non signée ou notariée par l'ID de développeur.

## Démarrage rapide

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

Les packages Linux installent une entrée de bureau qui lance `localsend-tray`, qui possède le cycle de vie du démon. Sur les ordinateurs de bureau sans barre d'état, la barre d'état maintient le GUI ouvert à la place ; `LocalSend --launcher` sélectionne explicitement ce mode à partir du AppImage.

### Windows

Décompressez et exécutez `LocalSend.exe`. L'archive contient côte à côte le GUI, le démon, le plateau et le lanceur ; gardez-les ensemble.

## Comportements à connaître

- **Quitter est une opération.** Quitter depuis la barre d'état met fin au GUI enregistré dans le `gui.pid` de portée du point de terminaison, arrête le démon détenu dans un délai d'attente limité et se termine sans laisser de processus ou de tâche keep-alive lancée derrière.
- **Les notifications proviennent du bac.** Le GUI ne publie jamais ses propres notifications, chaque plateforme dispose donc d'un seul chemin de livraison.
- **L'état de la fenêtre est par point de terminaison.** L'emplacement est stocké à côté de la configuration du démon pour ce point de terminaison plutôt que dans un emplacement global partagé, de sorte que deux instances ne se disputent pas la géométrie de la fenêtre.
- **Le démon fait autorité.** Les paramètres modifiés dans GUI sont soumis au démon et renvoyés via un instantané ; un frontend ne conserve aucune copie privée qui pourrait dériver.

## Langues

L'interface est livrée dans 57 paramètres régionaux, correspondant à la liste de LocalSend en amont, avec le chinois simplifié comme langue msgid. Choisissez-en un sous **Paramètres → Langue** ou laissez-le sur « Suivre le système » pour utiliser les paramètres régionaux du système d'exploitation.

| | |
| --- | --- |
| Catalogues groupés | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Chaînes sources de l'interface utilisateur | `@tr("...")` dans `apps/localsend-gui-slint/ui` |
| catalogue anglais | `translations/locale-en.json` (interface utilisateur), `translations/locale-en-runtime.json` (Rust messages) |
| Messages côté rouille | `crates/localsend-i18n` (généré `lookup_generated.rs`) |
| Miroir en amont | `translations/upstream/<locale>.json` |

Régénérez les catalogues après avoir modifié une chaîne d'interface utilisateur :

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` est validé, les catalogues générés sont donc reproductibles hors ligne. Corrigez une entrée ici et réexécutez le générateur pour corriger une traduction.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construire à partir de la source

Exigences : Rust `1.97.1` (épinglé par `rust-toolchain.toml`) et, pour les builds Linux, les en-têtes de développement GTK3 et AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Forfaits plateforme :

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

Les scripts d'acceptation pilotent les vrais binaires au lieu de l'état simulé :

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentation

| Document | Contenu |
| --- | --- |
| [`docs/desktop-pc.md`] (docs/desktop-pc.md) | Fonctionnement, packaging et comportement de la plateforme |
| [`docs/verification.zh-CN.md`] (docs/verification.zh-CN.md) | Preuves de tests actuelles et zones non vérifiées connues |
| [`docs/acceptance-automation.zh-CN.md`] (docs/acceptance-automation.zh-CN.md) | Portes d'acceptation locales |
| [`docs/agent-regression-playbook.zh-CN.md`] (docs/agent-regression-playbook.zh-CN.md) | Régressions récurrentes et preuves attendues avant de changer de comportement |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`] (docs/HANDOFF_CLAUDE.zh-CN.md) | État de convergence visuelle par rapport au prototype OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`] (docs/android-root-mvp.zh-CN.md) | Le module racine Android et ses limites |
| [`VENDORED_CORE.md`] (VENDORED_CORE.md) | Provenance amont du noyau vendu |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Certains documents sont rédigés en chinois ; ils enregistrent les cycles de vérification et les conventions dans les mots dans lesquels ils ont été convenus.

## Périmètre

Le produit principal est le client PC. La prise en charge de Android est un démon de réception sans tête uniquement root jusqu'à ce qu'il soit explicitement promu au-delà de cette portée - il n'a pas de GUI, ne démarre aucun service de premier plan et ne publie aucune notification persistante. `crates/localsend-official-core` est conservé octet par octet identique à son exportation amont enregistrée, à l'exception de son fichier `SHA256SUMS` généré.

## Remerciements

Le protocole, le format filaire et le noyau Rust proviennent de [localsend/localsend] (https://github.com/localsend/localsend) ; ce projet construit un client de bureau sur ce travail plutôt que de réimplémenter le protocole. Merci aux LocalSend auteurs et contributeurs.

L'interface suit un prototype de conception développé pour ce projet ; sa référence visuelle réside dans [`apps/localsend-gui-slint/reference/open-design/`] (apps/localsend-gui-slint/reference/open-design/).

## Contribuer

- **Bogues et suggestions** : ouvrez un [issue] (https://github.com/alkaid-ops/localsend-next/issues) avec la version, la plateforme, les étapes de reproduction et ce que vous avez vu. Un journal ou une capture d’écran permet d’agir beaucoup plus rapidement.
- **Code** : les demandes de tirage sont les bienvenues. Les modifications qui touchent la limite daemon/frontend, le noyau vendu ou le contrat visuel doivent être discutées en premier — les contraintes sont enregistrées dans [`AGENTS.md`] (AGENTS.md).
- **Traductions** : les catalogues sont générés à partir du cache de traduction automatique validé, les corrections constituent donc le type de contribution le plus simple à examiner.

## Licence

[Apache-2.0] (LICENSE). Les composants tiers et leurs licences sont répertoriés dans [THIRD_PARTY_NOTICES.md] (THIRD_PARTY_NOTICES.md).
