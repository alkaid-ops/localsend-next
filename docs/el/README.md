<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | **Ελληνικά** | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



Το localsend-next είναι ένα πρόγραμμα-πελάτης επιτραπέζιου υπολογιστή συμβατό με LocalSend γραμμένο σε Rust. Μιλάει το επίσημο πρωτόκολλο, επομένως ανακαλύπτει και μεταφέρει αρχεία με την επίσημη εφαρμογή LocalSend και προς τις δύο κατευθύνσεις, ενώ εκτελείται ως εγγενές πρόγραμμα επιτραπέζιου υπολογιστή αντί για εφαρμογή Flutter.

Η πλευρά του δικτύου είναι ο επίσημος πυρήνας LocalSend Rust, που πωλείται αμετάβλητος. Όλα πάνω από αυτό — δαίμονας, στρώμα RPC, δίσκος, διεπαφή χρήστη τερματικού και το Slint GUI — είναι χτισμένα γύρω από αυτόν τον πυρήνα.

## Διαδήλωση

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Στέλνω | Ρυθμίσεις |
| --- | --- |
| ![Αποστολή σελίδας με το αρχείο, το φάκελο, το κείμενο και τις πηγές του προχείρου και τις κοντινές συσκευές](../../docs/images/readme-send-light.png) | ![Σελίδα ρυθμίσεων με θέμα, γλώσσα, δίκτυο και επιλογές μεταφοράς](../../docs/images/readme-settings-light.png) |

## Χαρακτηριστικά

- Ανακάλυψη και μεταφορά σε σχέση με επίσημους πελάτες LocalSend, στον προμηθευτή upstream πυρήνα αντί για εκ νέου εφαρμογή
- Ένας κανονικός κάτοχος πολιτείας: ο δαίμονας κατέχει ρυθμίσεις, ταυτότητα, αγαπημένα, ιστορικό, μεταφορές και την ταυτότητα TLS, επομένως κάθε frontend βλέπει το ίδιο πράγμα
- GUI, διεπαφή χρήστη τερματικού και δίσκος ως ανεξάρτητες διεπαφές μέσω του ίδιου τοπικού RPC. ο δίσκος κατέχει τον δαίμονα και τον κύκλο ζωής της διαδικασίας GUI
- Πρόοδος ανά αρχείο σε πραγματικό χρόνο, ιστορικό με ανοιχτές ενέργειες και ενέργειες αποκάλυψης, αγαπημένα, μη αυτόματη εισαγωγή συσκευής και σύνδεσμοι λήψης κοινής χρήσης Ιστού
- 57 γλώσσες διεπαφής που ταιριάζουν με τη λίστα upstream LocalSend, με δυνατότητα εναλλαγής κατά το χρόνο εκτέλεσης ή ακολουθώντας τις τοπικές ρυθμίσεις του συστήματος
- Ενσωμάτωση εγγενούς επιφάνειας εργασίας: ειδοποιήσεις μόνο από το δίσκο, εκκίνηση κατά τη σύνδεση, ελαχιστοποίηση στο δίσκο, εμμονή τοποθέτησης παραθύρου και παράθυρο macOS που μπαίνει στη γραμμή τίτλου
- Συσκευασία για macOS (@4@@/ZIP), Linux (deb, rpm, AppImage, tarball) και Windows (ZIP), τόσο για x86_64 και aarch64

## Αρχιτεκτονική

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Ο δαίμονας είναι η μόνη διεργασία που μιλά στο δίκτυο ή γράφει την κατάσταση εφαρμογής. Τα Frontend δεν εξαρτώνται ποτέ από τον πωλούμενο πυρήνα και ποτέ δεν γεννούν τον ίδιο τον δαίμονα. ο δίσκος κάνει. Τα byte αρχείων δεν ταξιδεύουν πάνω από RPC — ο δαίμονας επιλύει διαδρομές, κατακερματίζει το περιεχόμενο και καταγράφει το ιστορικό.

| Δυάδικος | Ρόλος |
| --- | --- |
| `localsendd` | Υπηρεσία δικτύου και κατάσταση κανονικής εφαρμογής |
| `localsend-tray` | Daemon και κάτοχος κύκλου ζωής GUI, ειδοποιήσεις |
| `localsend-gui-slint` | Το επίσημο GUI, Slint με την απόδοση FemtoVG |
| `localsend-tui` | Πρόσοψη τερματικού |
| `localsend-launcher` | Σαφής εκκινητής για περιβάλλοντα χωρίς δίσκο |
| `localsend-androidd` | Αυτόνομο root-only Android λαμβάνουν δαίμονα (MVP) |

Οι κατάλογοι `localsend-gui-egui`, `-fltk`, `-iced` και `-tauri` είναι πειραματικές θύρες και παραμένουν εκτός του χώρου εργασίας root μέχρι να ολοκληρωθεί η ενσωμάτωσή τους στο RPC.

## Λήψη

Τα στοιχεία κυκλοφορίας ακολουθούν ένα σχήμα ονοματοδοσίας — `LocalSend-<version>-<platform>-<arch>.<ext>`, με πλατφόρμες `linux` / `windows` / `macos` και αρχιτεκτονικές `x86_64` / `aarch64`:

| Πλατφόρμα | Κεφάλαιο |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Οποιαδήποτε Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

Το `SHA256SUMS.txt` καλύπτει κάθε στοιχείο σε μια κυκλοφορία. Η κατασκευή macOS που παράγεται από την CI είναι υπογεγραμμένη ad-hoc, δεν έχει υπογραφεί ή συμβολαιογραφικό αναγνωριστικό προγραμματιστή.

## Γρήγορη εκκίνηση

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

Τα πακέτα Linux εγκαθιστούν μια καταχώρηση επιφάνειας εργασίας που εκκινεί το `localsend-tray`, το οποίο κατέχει τον κύκλο ζωής του δαίμονα. Σε επιτραπέζιους υπολογιστές χωρίς γραμμή κατάστασης, ο δίσκος διατηρεί το GUI ανοιχτό. Το `LocalSend --launcher` επιλέγει αυτή τη λειτουργία ρητά από το AppImage.

### Windows

Αποσυμπιέστε και εκτελέστε το `LocalSend.exe`. Το αρχείο περιέχει το GUI, τον δαίμονα, τον δίσκο και τον εκτοξευτή δίπλα-δίπλα. κρατήστε τα μαζί.

## Συμπεριφορά που αξίζει να γνωρίζεις

- **Η έξοδος είναι μία λειτουργία.** Η έξοδος από το δίσκο τερματίζει το GUI που έχει καταγραφεί στο εύρος τελικού σημείου `gui.pid`, σταματά τον ιδιόκτητο δαίμονα εντός ενός περιορισμένου χρονικού ορίου και κλείνει χωρίς να αφήνει πίσω μια διαδικασία ή μια εργασία διατήρησης ζωντανής έναρξης.
- **Οι ειδοποιήσεις προέρχονται από το δίσκο.** Το GUI δεν δημοσιεύει ποτέ τις δικές του ειδοποιήσεις, επομένως κάθε πλατφόρμα έχει μια ενιαία διαδρομή παράδοσης.
- **Η κατάσταση του παραθύρου είναι ανά τελικό σημείο.** Η τοποθέτηση αποθηκεύεται δίπλα στη διαμόρφωση του δαίμονα για αυτό το τελικό σημείο και όχι σε μια κοινόχρηστη καθολική τοποθεσία, επομένως δύο περιπτώσεις δεν τσακώνονται για τη γεωμετρία του παραθύρου.
- **Ο δαίμονας είναι έγκυρος.** Οι ρυθμίσεις που άλλαξαν στο GUI υποβάλλονται στον δαίμονα και επαναλαμβάνονται μέσω ενός στιγμιότυπου. ένα frontend δεν διατηρεί ιδιωτικό αντίγραφο που θα μπορούσε να παρασυρθεί.

## Γλώσσες

Η διεπαφή αποστέλλεται σε 57 τοπικές ρυθμίσεις, που ταιριάζουν με τη λίστα του upstream LocalSend, με τα Απλοποιημένα Κινεζικά ως γλώσσα msgid. Επιλέξτε ένα από το **Ρυθμίσεις → Γλώσσα** ή αφήστε το στο "Ακολουθήστε το σύστημα" για να χρησιμοποιήσετε τις τοπικές ρυθμίσεις του λειτουργικού συστήματος.

| | |
| --- | --- |
| Πακέτο κατάλογοι | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Συμβολοσειρές πηγής διεπαφής χρήστη | `@tr("...")` σε `apps/localsend-gui-slint/ui` |
| Αγγλικός κατάλογος | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust μηνύματα) |
| Μηνύματα από την πλευρά της σκουριάς | `crates/localsend-i18n` (δημιουργήθηκε `lookup_generated.rs`) |
| Καθρέφτης ανάντη | `translations/upstream/<locale>.json` |

Αναδημιουργήστε τους καταλόγους αφού αλλάξετε οποιαδήποτε συμβολοσειρά διεπαφής χρήστη:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

Το `locale-machine-cache.json` δεσμεύεται, επομένως οι δημιουργημένοι κατάλογοι μπορούν να αναπαραχθούν εκτός σύνδεσης. Διορθώστε μια καταχώριση εκεί και εκτελέστε ξανά τη γεννήτρια για να διορθώσετε μια μετάφραση.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Κτίριο από την πηγή

Απαιτήσεις: Rust `1.97.1` (καρφιτσώθηκε από `rust-toolchain.toml`) και, για τις κατασκευές Linux, οι κεφαλίδες ανάπτυξης GTK3 και AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Πακέτα πλατφόρμας:

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

Τα σενάρια αποδοχής οδηγούν τα πραγματικά δυαδικά αρχεία αντί για την κατάσταση κοροϊδίας:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Απόδειξη με έγγραφα

| Εγγραφο | Περιεχόμενα |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Λειτουργία, συσκευασία και συμπεριφορά πλατφόρμας |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Τρέχοντα στοιχεία δοκιμών και γνωστές μη επαληθευμένες περιοχές |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Τοπικές πύλες αποδοχής |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Επαναλαμβανόμενες παλινδρομήσεις και το ίχνος αποδεικτικών στοιχείων που αναμένεται πριν από την αλλαγή συμπεριφοράς |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Κατάσταση οπτικής σύγκλισης έναντι του πρωτοτύπου OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Η ρίζα Android και τα όριά της |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Ανοδική προέλευση του πωλούμενου πυρήνα |

Αυτό το README δημοσιεύεται και στις 57 γλώσσες διεπαφής — επιλέξτε μία από το πρόγραμμα εναλλαγής στο επάνω μέρος. Οι μεταφράσεις δημιουργούνται από `README.md` από `scripts/translate-readme.mjs`; οι διορθώσεις γίνονται στο [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Ορισμένα έγγραφα είναι γραμμένα στα κινέζικα. καταγράφουν τις διαδικασίες επαλήθευσης και τις συμβάσεις με τις λέξεις στις οποίες συμφωνήθηκαν.

## Εύρος

Το κύριο προϊόν είναι ο υπολογιστής-πελάτης. Η υποστήριξη Android είναι ένας δαίμονας λήψης μόνο root, χωρίς κεφάλι μέχρι να προωθηθεί ρητά πέρα ​​από αυτό το εύρος — δεν έχει GUI, δεν ξεκινά υπηρεσία προσκηνίου και δεν δημοσιεύει επίμονη ειδοποίηση. Το `crates/localsend-official-core` διατηρείται byte-for-byte πανομοιότυπο με την καταγεγραμμένη εξαγωγή του, εκτός από το αρχείο `SHA256SUMS` που δημιουργήθηκε.

## Πιστώσεις

Το πρωτόκολλο, η μορφή καλωδίου και ο πυρήνας Rust προέρχονται από [localsend/localsend](https://github.com/localsend/localsend); Αυτό το έργο δημιουργεί έναν υπολογιστή-πελάτη επιτραπέζιου υπολογιστή σε αυτήν την εργασία αντί για την εκ νέου υλοποίηση του πρωτοκόλλου. Ευχαριστούμε τους συγγραφείς και τους συντελεστές LocalSend.

Η διεπαφή ακολουθεί ένα πρωτότυπο σχεδιασμού που αναπτύχθηκε για αυτό το έργο. Η οπτική του αναφορά βρίσκεται στο [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Συμβάλλοντας

- **Σφάλματα και προτάσεις**: ανοίξτε ένα [θέμα](https://github.com/alkaid-ops/localsend-next/issues) με την έκδοση, την πλατφόρμα, τα βήματα αναπαραγωγής και τι είδατε. Ένα αρχείο καταγραφής ή στιγμιότυπο οθόνης καθιστά πολύ πιο γρήγορη την ενέργεια.
- **Κωδικός**: Τα αιτήματα έλξης είναι ευπρόσδεκτα. Οι αλλαγές που αγγίζουν το όριο daemon/frontend, τον πυρήνα πωλητή ή το οπτικό συμβόλαιο θα πρέπει να συζητηθούν πρώτα — οι περιορισμοί καταγράφονται στο [`AGENTS.md`](../../AGENTS.md).
- **Μεταφράσεις**: οι κατάλογοι δημιουργούνται από τη δεσμευμένη κρυφή μνήμη μηχανικής μετάφρασης, επομένως οι διορθώσεις εκεί είναι το πιο εύκολο είδος συνεισφοράς για έλεγχο.

## Αδεια

[Apache-2.0](../../LICENSE). Τα στοιχεία τρίτων και οι άδειές τους παρατίθενται στο [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
