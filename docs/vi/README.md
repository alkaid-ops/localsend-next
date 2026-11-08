<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | **Tiếng Việt** | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next là ứng dụng khách dành cho máy tính để bàn tương thích với LocalSend được viết bằng Rust. Nó sử dụng giao thức chính thức, do đó, nó phát hiện và truyền các tệp bằng ứng dụng LocalSend chính thức theo cả hai hướng, trong khi chạy như một chương trình máy tính để bàn gốc thay vì ứng dụng Flutter.

Phía mạng là lõi LocalSend Rust chính thức, được bán không thay đổi. Mọi thứ phía trên nó — daemon, lớp RPC, khay, giao diện người dùng đầu cuối và Slint GUI — đều được xây dựng xung quanh lõi đó.

## Thử nghiệm

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Gửi | Cài đặt |
| --- | --- |
| ![Gửi trang có chứa tệp, thư mục, văn bản, nguồn clipboard và các thiết bị lân cận](../../docs/images/readme-send-light.png) | ![Trang cài đặt với các tùy chọn chủ đề, ngôn ngữ, mạng và truyền tải](../../docs/images/readme-settings-light.png) |

## Tính năng

- Khám phá và chuyển giao cho các khách hàng LocalSend chính thức, trên lõi ngược dòng được cung cấp thay vì triển khai lại
- Một chủ sở hữu trạng thái chuẩn: daemon chứa các cài đặt, danh tính, mục yêu thích, lịch sử, chuyển khoản và danh tính TLS, vì vậy mọi giao diện người dùng đều thấy điều giống nhau
- GUI, giao diện người dùng đầu cuối và khay dưới dạng các giao diện người dùng độc lập trên cùng một địa phương RPC; khay sở hữu daemon và vòng đời quy trình GUI
- Tiến trình theo thời gian thực trên mỗi tệp, lịch sử với các hành động mở và hiển thị, mục yêu thích, mục nhập thiết bị thủ công và Chia sẻ web nhận liên kết
- 57 ngôn ngữ giao diện phù hợp với danh sách ngược dòng của LocalSend, có thể chuyển đổi khi chạy hoặc theo ngôn ngữ hệ thống
- Tích hợp máy tính để bàn gốc: chỉ thông báo từ khay, khởi chạy khi đăng nhập, thu nhỏ vào khay, duy trì vị trí cửa sổ và cửa sổ macOS kéo vào thanh tiêu đề của nó
- Bao bì cho macOS (DMG/ZIP), Linux (deb, vòng/phút, AppImage, tarball) và Windows (ZIP), cho cả x86_64 và aarch64

## Ngành kiến ​​​​trúc

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemon là tiến trình duy nhất giao tiếp với mạng hoặc ghi trạng thái ứng dụng. Giao diện người dùng không bao giờ phụ thuộc vào lõi được cung cấp và không bao giờ tự sinh ra daemon; cái khay thì có. Các byte tệp không di chuyển qua RPC — daemon phân giải các đường dẫn, băm nội dung và ghi lại lịch sử.

| nhị phân | Vai trò |
| --- | --- |
| `localsendd` | Dịch vụ mạng và trạng thái ứng dụng chuẩn |
| `localsend-tray` | Daemon và chủ sở hữu vòng đời GUI, thông báo |
| `localsend-gui-slint` | GUI, Slint chính thức với trình kết xuất FemtoVG |
| `localsend-tui` | Giao diện đầu cuối |
| `localsend-launcher` | Trình khởi chạy rõ ràng cho môi trường không có khay |
| `localsend-androidd` | Chỉ root độc lập Android nhận daemon (MVP) |

Các thư mục `localsend-gui-egui`, `-fltk`, `-iced` và `-tauri` là các cổng thử nghiệm và nằm ngoài không gian làm việc gốc cho đến khi quá trình tích hợp RPC của chúng hoàn tất.

## Tải xuống

Nội dung phát hành tuân theo một sơ đồ đặt tên — `LocalSend-<version>-<platform>-<arch>.<ext>`, với nền tảng `linux` / `windows` / `macos` và kiến ​​trúc `x86_64` / `aarch64`:

| Nền tảng | Tài sản |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Bất kỳ Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` bao gồm mọi nội dung trong bản phát hành. Bản dựng macOS do CI tạo ra được ký đặc biệt, không phải ID nhà phát triển được ký hoặc công chứng.

## Bắt đầu nhanh

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

Các gói Linux cài đặt một mục trên máy tính để bàn khởi chạy `localsend-tray`, sở hữu vòng đời daemon. Trên máy tính để bàn không có thanh trạng thái, thay vào đó, khay sẽ giữ GUI mở; `LocalSend --launcher` chọn chế độ đó một cách rõ ràng từ AppImage.

### Windows

Giải nén và chạy `LocalSend.exe`. Kho lưu trữ chứa GUI, daemon, khay và trình khởi chạy cạnh nhau; giữ chúng lại với nhau.

## Hành vi đáng biết

- **Thoát là một thao tác.** Thoát khỏi khay sẽ chấm dứt GUI được ghi trong `gui.pid` ở phạm vi điểm cuối, dừng daemon sở hữu trong một khoảng thời gian chờ giới hạn và thoát mà không để lại một tiến trình hoặc công việc duy trì launchd nào phía sau.
- **Thông báo đến từ khay.** GUI không bao giờ đăng thông báo riêng, vì vậy mỗi nền tảng có một đường dẫn phân phối duy nhất.
- **Trạng thái cửa sổ dành cho mỗi điểm cuối.** Vị trí được lưu trữ bên cạnh cấu hình daemon cho điểm cuối đó thay vì ở một vị trí chung được chia sẻ, do đó hai phiên bản không tranh nhau về hình dạng cửa sổ.
- **Daemon có thẩm quyền.** Các cài đặt đã thay đổi trong GUI được gửi tới daemon và phản hồi lại qua ảnh chụp nhanh; giao diện người dùng không giữ bản sao riêng tư nào có thể trôi dạt.

## Ngôn ngữ

Giao diện có sẵn ở 57 ngôn ngữ, khớp với danh sách LocalSend ngược dòng, với tiếng Trung giản thể là ngôn ngữ msgid. Chọn một trong **Cài đặt → Ngôn ngữ** hoặc để nó ở phần "Theo dõi hệ thống" để sử dụng ngôn ngữ hệ điều hành.

| | |
| --- | --- |
| Danh mục đi kèm | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Chuỗi nguồn giao diện người dùng | `@tr("...")` ở `apps/localsend-gui-slint/ui` |
| Danh mục tiếng Anh | `translations/locale-en.json` (Giao diện người dùng), tin nhắn `translations/locale-en-runtime.json` (Rust) |
| Tin nhắn rỉ sét | `crates/localsend-i18n` (được tạo `lookup_generated.rs`) |
| Gương ngược dòng | `translations/upstream/<locale>.json` |

Tạo lại danh mục sau khi thay đổi bất kỳ chuỗi giao diện người dùng nào:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` được cam kết nên các danh mục được tạo có thể tái tạo ngoại tuyến. Sửa một mục ở đó và chạy lại trình tạo để sửa bản dịch.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Xây dựng từ nguồn

Yêu cầu: Rust `1.97.1` (được ghim bởi `rust-toolchain.toml`) và đối với các bản dựng Linux, các tiêu đề phát triển GTK3 và AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Gói nền tảng:

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

Tập lệnh chấp nhận điều khiển các tệp nhị phân thực thay vì trạng thái mô phỏng:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Tài liệu

| Tài liệu | Nội dung |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Hoạt động, đóng gói và hành vi nền tảng |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Bằng chứng thử nghiệm hiện tại và các khu vực chưa được xác minh đã biết |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Cổng chấp nhận địa phương |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Hồi quy định kỳ và dấu vết bằng chứng dự kiến ​​trước khi thay đổi hành vi |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Trạng thái hội tụ trực quan so với nguyên mẫu OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Mô-đun gốc Android và các giới hạn của nó |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Xuất xứ ngược dòng của lõi được bán |

README này được xuất bản bằng tất cả 57 ngôn ngữ giao diện — hãy chọn một ngôn ngữ từ trình chuyển đổi ở trên cùng. Các bản dịch được tạo từ `README.md` bởi `scripts/translate-readme.mjs`; các chỉnh sửa sẽ được thực hiện trong [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Một số tài liệu được viết bằng tiếng Trung Quốc; họ ghi lại các bước xác minh và quy ước bằng những từ mà họ đã thống nhất.

## Phạm vi

Sản phẩm chính là máy khách PC. Hỗ trợ Android là một daemon nhận không có đầu, chỉ có root cho đến khi được quảng bá rõ ràng ngoài phạm vi đó — nó không có GUI, không bắt đầu dịch vụ nền trước và không đăng thông báo liên tục. `crates/localsend-official-core` được giữ từng byte giống hệt với quá trình xuất ngược dòng được ghi lại, ngoại trừ tệp `SHA256SUMS` được tạo.

## Tín dụng

Giao thức, định dạng dây và lõi Rust đến từ [localsend/localsend](https://github.com/localsend/localsend); dự án này xây dựng một máy khách để bàn cho công việc đó thay vì triển khai lại giao thức. Cảm ơn các tác giả và cộng tác viên LocalSend.

Giao diện tuân theo nguyên mẫu thiết kế được phát triển cho dự án này; tham chiếu trực quan của nó nằm trong [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Đóng góp

- **Lỗi và đề xuất**: mở [vấn đề](https://github.com/alkaid-ops/localsend-next/issues) về phiên bản, nền tảng, các bước tái tạo và những gì bạn đã thấy. Nhật ký hoặc ảnh chụp màn hình giúp hành động nhanh hơn nhiều.
- **Mã**: yêu cầu kéo đều được chào đón. Những thay đổi liên quan đến ranh giới daemon/frontend, lõi được cung cấp hoặc hợp đồng trực quan nên được thảo luận trước — các ràng buộc được ghi lại trong [`AGENTS.md`](../../AGENTS.md).
- **Bản dịch**: các danh mục được tạo từ bộ nhớ đệm của bản dịch máy đã cam kết, do đó, các bản chỉnh sửa là loại đóng góp dễ dàng nhất để xem xét.

## Giấy phép

[Apache-2.0](../../LICENSE). Các thành phần của bên thứ ba và giấy phép của chúng được liệt kê trong [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
