<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | **Türkçe** | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next, Rust ile yazılmış, LocalSend uyumlu bir masaüstü istemcisidir. Resmi protokolü konuşur, bu nedenle Flutter uygulaması yerine yerel bir masaüstü programı olarak çalışırken resmi LocalSend uygulamasıyla dosyaları her iki yönde keşfeder ve aktarır.

Ağ tarafı, değişmeden satılan resmi LocalSend Rust çekirdeğidir. Bunun üzerindeki her şey - arka plan programı, RPC katmanı, tepsi, terminal kullanıcı arayüzü ve Slint GUI - bu çekirdeğin etrafında inşa edilmiştir.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Göndermek | Ayarlar |
| --- | --- |
| ![Dosya, klasör, metin ve pano kaynakları ve yakındaki cihazlarla sayfayı gönderin](../../docs/images/readme-send-light.png) | ![Tema, dil, ağ ve aktarım seçeneklerini içeren ayarlar sayfası](../../docs/images/readme-settings-light.png) |

## Özellikler

- Yeniden uygulama yerine, satıcının yukarı yöndeki çekirdeğinde resmi LocalSend istemcilerine karşı keşif ve aktarım
- Tek bir standart durum sahibi: arka plan programı; ayarları, kimliği, sık kullanılanları, geçmişi, aktarımları ve TLS kimliğini tutar; böylece her ön uç aynı şeyi görür.
- GUI, aynı yerel RPC üzerinden bağımsız ön uçlar olarak terminal kullanıcı arayüzü ve tepsi; tepsi arka plan programının ve GUI süreç yaşam döngüsünün sahibidir
- Dosya başına gerçek zamanlı ilerleme, açma ve gösterme eylemleriyle geçmiş, favoriler, manuel cihaz girişi ve Web Paylaşımı alma bağlantıları
- Yukarı akış LocalSend'nin listesiyle eşleşen, çalışma zamanında değiştirilebilen veya sistem yerel ayarını takip eden 57 arayüz dili
- Yerel masaüstü entegrasyonu: yalnızca tepsiden gelen bildirimler, oturum açarken başlatma, tepsiye simge durumuna küçültme, pencere yerleştirme kalıcılığı ve başlık çubuğuna uzanan bir macOS penceresi
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) ve Windows (ZIP) için paketleme, hem x86_64 hem de aarch64 için

## Mimarlık

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemon, ağla konuşan veya uygulama durumunu yazan tek süreçtir. Ön uçlar hiçbir zaman satılan çekirdeğe bağlı değildir ve arka plan programını asla kendileri oluşturmaz; tepsi bunu yapar. Dosya baytları RPC üzerinden seyahat etmez; arka plan programı yolları çözer, içeriği karma hale getirir ve geçmişi kaydeder.

| İkili | Rol |
| --- | --- |
| `localsendd` | Ağ hizmeti ve kurallı uygulama durumu |
| `localsend-tray` | Daemon ve GUI yaşam döngüsü sahibi, bildirimler |
| `localsend-gui-slint` | FemtoVG oluşturucusu ile resmi GUI, Slint |
| `localsend-tui` | Terminal ön ucu |
| `localsend-launcher` | Tepsisiz ortamlar için açık başlatıcı |
| `localsend-androidd` | Bağımsız yalnızca kök Android alma arka plan programı (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` ve `-tauri` dizinleri deneysel bağlantı noktalarıdır ve RPC entegrasyonu tamamlanana kadar kök çalışma alanının dışında kalır.

## İndirmek

Sürüm varlıkları tek bir adlandırma şemasını takip eder — `LocalSend-<version>-<platform>-<arch>.<ext>`, `linux` / `windows` / `macos` platformları ve `x86_64` / `aarch64` mimarileri ile:

| platformu | Varlık |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Herhangi bir Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` bir sürümdeki her varlığı kapsar. CI tarafından üretilen macOS yapısı, Geliştirici Kimliği imzalı veya noter tasdikli değil, geçici olarak imzalanmıştır.

## Hızlı başlangıç

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

Linux paketleri, daemon yaşam döngüsünün sahibi olan `localsend-tray`'yi başlatan bir masaüstü girişi yükler. Durum çubuğu olmayan masaüstü bilgisayarlarda tepsi bunun yerine GUI'yi açık tutar; `LocalSend --launcher` bu modu açıkça AppImage arasından seçer.

### Windows

Sıkıştırılmış dosyayı açın ve `LocalSend.exe` dosyasını çalıştırın. Arşiv GUI, arka plan programı, tepsi ve başlatıcıyı yan yana tutar; onları bir arada tutun.

## Bilmeye değer davranış

- **Çıkış tek bir işlemdir.** Tepsiden çıkmak, uç nokta kapsamlı `gui.pid`'de kayıtlı GUI'yi sonlandırır, sınırlı bir zaman aşımı süresi içinde sahip olunan arka plan programını durdurur ve geride bir işlem veya başlatılan canlı tutma işi bırakmadan çıkar.
- **Bildirimler tepsiden gelir.** GUI hiçbir zaman kendi bildirimlerini göndermez, dolayısıyla her platformun tek bir dağıtım yolu vardır.
- **Pencere durumu uç nokta başınadır.** Yerleştirme, paylaşılan bir küresel konum yerine söz konusu uç nokta için arka plan programı yapılandırmasının yanında depolanır; böylece iki örnek, pencere geometrisi için kavga etmez.
- **Arka plan programı yetkilidir.** GUI'da değiştirilen ayarlar arka plan programına gönderilir ve anlık görüntü aracılığıyla geri yansıtılır; bir ön uçta sürüklenebilecek hiçbir özel kopya bulunmaz.

## Diller

Arayüz, msgid dili olarak Basitleştirilmiş Çince ile yukarı akış LocalSend'nin listesiyle eşleşen 57 yerel ayarda gönderilir. **Ayarlar → Dil** altından birini seçin veya işletim sistemi yerel ayarını kullanmak için "Sistemi takip et" seçeneğinde bırakın.

| | |
| --- | --- |
| Paket kataloglar | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Kullanıcı arayüzü kaynak dizeleri | `apps/localsend-gui-slint/ui` içinde `@tr("...")` |
| İngilizce katalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mesajlar) |
| Pas tarafı mesajları | `crates/localsend-i18n` (oluşturulan `lookup_generated.rs`) |
| Yukarı akış aynası | `translations/upstream/<locale>.json` |

Herhangi bir kullanıcı arayüzü dizesini değiştirdikten sonra katalogları yeniden oluşturun:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` taahhüt edilmiştir, dolayısıyla oluşturulan kataloglar çevrimdışı olarak çoğaltılabilir. Buradaki girişi düzeltin ve çeviriyi düzeltmek için oluşturucuyu yeniden çalıştırın.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Kaynaktan inşa etmek

Gereksinimler: Rust `1.97.1` (`rust-toolchain.toml` ile sabitlenmiştir) ve Linux derlemeleri için GTK3 ve AppIndicator geliştirme başlıkları.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platform paketleri:

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

Kabul komut dosyaları, sahte durum yerine gerçek ikili dosyaları yönlendirir:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokümantasyon

| Belge | İçindekiler |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operasyon, paketleme ve platform davranışı |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Güncel test kanıtları ve bilinen doğrulanmamış alanlar |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Yerel kabul kapıları |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Davranışı değiştirmeden önce tekrarlanan regresyonlar ve beklenen kanıt izi |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign prototipine karşı görsel yakınsama durumu |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android kök modülü ve sınırları |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Satılan çekirdeğin yukarı akış kaynağı |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Bazı belgeler Çince yazılmıştır; doğrulama işlemlerini ve sözleşmeleri üzerinde anlaştıkları kelimelerle kaydederler.

## Kapsam

Birincil ürün PC istemcisidir. Android desteği, bu kapsamın ötesinde açıkça tanıtılana kadar yalnızca root'a yönelik, başsız bir alma arka plan programıdır; GUI içermez, ön plan hizmetini başlatmaz ve kalıcı bir bildirim göndermez. `crates/localsend-official-core`, oluşturulan `SHA256SUMS` dosyası dışında, kaydedilen yukarı akış dışa aktarımıyla bayt bayt aynı tutulur.

## Kredi

Protokol, kablo formatı ve Rust çekirdeği [localsend/localsend](https://github.com/localsend/localsend)'den gelir; bu proje, protokolü yeniden uygulamak yerine söz konusu çalışma üzerine bir masaüstü istemcisi oluşturur. LocalSend yazarlara ve katkıda bulunanlara teşekkür ederiz.

Arayüz, bu proje için geliştirilen bir tasarım prototipini takip ediyor; görsel referansı [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) içinde bulunur.

## Katkıda Bulunmak

- **Hatalar ve öneriler**: Sürüm, platform, çoğaltma adımları ve gördüklerinizi içeren bir [sayı](https://github.com/alkaid-ops/localsend-next/issues) açın. Bir günlük veya ekran görüntüsü, harekete geçmeyi çok daha hızlı hale getirir.
- **Kod**: çekme istekleri kabul edilir. daemon/frontend sınırına, tedarikçi çekirdeğine veya görsel sözleşmeye dokunan değişiklikler ilk önce tartışılmalıdır; kısıtlamalar [`AGENTS.md`](../../AGENTS.md) içine kaydedilir.
- **Çeviriler**: Kataloglar, taahhüt edilen makine çevirisi önbelleğinden oluşturulur, dolayısıyla oradaki düzeltmeler, gözden geçirilmesi en kolay katkı türüdür.

## Lisans

[Apache-2.0](../../LICENSE). Üçüncü taraf bileşenler ve bunların lisansları [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) adresinde listelenmiştir.
