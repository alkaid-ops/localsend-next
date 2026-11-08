<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | **한국어** | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next은 Rust로 작성된 LocalSend 호환 데스크톱 클라이언트입니다. 공식 프로토콜을 사용하므로 Flutter 앱 대신 기본 데스크톱 프로그램으로 실행되는 동안 공식 LocalSend 앱을 사용하여 양방향으로 파일을 검색하고 전송합니다.

네트워크 측은 공식 LocalSend Rust 코어이며 변경되지 않은 상태로 공급됩니다. 그 위의 모든 것(데몬, RPC 레이어, 트레이, 터미널 UI 및 Slint GUI)은 해당 코어를 중심으로 구축됩니다.

## 데모

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| 보내다 | 설정 |
| --- | --- |
| ![파일, 폴더, 텍스트, 클립보드 소스 및 주변 장치가 포함된 페이지 보내기](../../docs/images/readme-send-light.png) | ![테마, 언어, 네트워크, 전송 옵션이 포함된 설정 페이지](../../docs/images/readme-settings-light.png) |

## 기능

- 재구현이 아닌 벤더 업스트림 코어에서 공식 LocalSend 클라이언트에 대한 검색 및 전송
- 하나의 표준 상태 소유자: 데몬은 설정, ID, 즐겨찾기, 기록, 전송 및 TLS ID를 보유하므로 모든 프런트엔드에서 동일한 내용을 볼 수 있습니다.
- GUI, 동일한 로컬 RPC에 대한 독립적인 프런트엔드로 터미널 UI 및 트레이; 트레이는 데몬과 GUI 프로세스 수명주기를 소유합니다.
- 파일별 실시간 진행, 열기 및 표시 작업 기록, 즐겨찾기, 수동 장치 입력 및 웹 공유 수신 링크
- 업스트림 LocalSend의 목록과 일치하는 57개 인터페이스 언어, 런타임 시 전환 가능 또는 시스템 로케일을 따름
- 기본 데스크탑 통합: 트레이에서만 알림, 로그인 시 실행, 트레이로 최소화, 창 배치 지속성, 제목 표시줄에 표시되는 macOS 창
- x86_64 및 aarch64 모두에 대해 macOS(DMG/ZIP), Linux(deb, rpm, AppImage, tarball) 및 Windows(ZIP)에 대한 패키징

## 아키텍처

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

데몬은 네트워크와 통신하거나 애플리케이션 상태를 쓰는 유일한 프로세스입니다. 프런트엔드는 벤더 코어에 의존하지 않으며 데몬 자체를 생성하지도 않습니다. 트레이는 그렇습니다. 파일 바이트는 RPC을 통해 이동하지 않습니다. 데몬은 경로를 확인하고 콘텐츠를 해시하며 기록을 기록합니다.

| 바이너리 | 역할 |
| --- | --- |
| `localsendd` | 네트워크 서비스 및 정식 애플리케이션 상태 |
| `localsend-tray` | 데몬 및 GUI 수명 주기 소유자, 알림 |
| `localsend-gui-slint` | FemtoVG 렌더러를 사용한 공식 GUI, Slint |
| `localsend-tui` | 터미널 프런트엔드 |
| `localsend-launcher` | 트레이가 없는 환경을 위한 명시적 실행 프로그램 |
| `localsend-androidd` | 독립형 루트 전용 Android 수신 데몬(MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` 및 `-tauri` 디렉터리는 실험용 포트이며 RPC 통합이 완료될 때까지 루트 작업 공간 외부에 유지됩니다.

## 다운로드

릴리스 자산은 `LocalSend-<version>-<platform>-<arch>.<ext>`, 플랫폼 `linux` / `windows` / `macos` 및 아키텍처 `x86_64` / `aarch64`과 같은 하나의 명명 체계를 따릅니다.

| 플랫폼 | 유산 |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| 모든 Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt`은 릴리스의 모든 자산을 다룹니다. CI에서 생성된 macOS 빌드는 개발자 ID가 서명되거나 공증되지 않고 임시로 서명됩니다.

## 빠른 시작

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

Linux 패키지는 데몬 수명 주기를 소유하는 `localsend-tray`을 시작하는 데스크탑 항목을 설치합니다. 상태 표시줄이 없는 데스크톱에서는 트레이가 대신 GUI을 열어 둡니다. `LocalSend --launcher`은 AppImage에서 해당 모드를 명시적으로 선택합니다.

### Windows

압축을 풀고 `LocalSend.exe`을 실행하세요. 아카이브에는 GUI, 데몬, 트레이 및 실행 프로그램이 나란히 보관되어 있습니다. 함께 유지하십시오.

## 알아둘 만한 동작

- **종료는 하나의 작업입니다.** 트레이를 종료하면 엔드포인트 범위 `gui.pid`에 기록된 GUI이 종료되고, 제한된 시간 초과 내에 소유한 데몬이 중지되며, 프로세스나 실행된 연결 유지 작업을 남기지 않고 종료됩니다.
- **알림은 트레이에서 제공됩니다.** GUI은 자체 알림을 게시하지 않으므로 각 플랫폼에는 단일 전달 경로가 있습니다.
- **창 상태는 엔드포인트별로 다릅니다.** 공유된 전역 위치가 아닌 해당 엔드포인트에 대한 데몬 구성 옆에 배치가 저장되므로 두 인스턴스가 창 형상을 놓고 싸우지 않습니다.
- **데몬은 권한이 있습니다.** GUI에서 변경된 설정은 데몬에 제출되고 스냅샷을 통해 다시 반영됩니다. 프런트엔드는 표류할 수 있는 개인 복사본을 보관하지 않습니다.

## 언어

인터페이스는 업스트림 LocalSend의 목록과 일치하는 57개 로케일로 제공되며 중국어 간체는 msgid 언어입니다. **설정 → 언어**에서 하나를 선택하거나 "시스템 따르기"에 그대로 두어 OS 로케일을 사용하세요.

| | |
| --- | --- |
| 번들 카탈로그 | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI 소스 문자열 | `@tr("...")` `apps/localsend-gui-slint/ui`에서 |
| 영어 카탈로그 | `translations/locale-en.json`(UI), `translations/locale-en-runtime.json`(Rust 메시지) |
| 러스트 측 메시지 | `crates/localsend-i18n`(`lookup_generated.rs` 생성) |
| 업스트림 미러 | `translations/upstream/<locale>.json` |

UI 문자열을 변경한 후 카탈로그를 다시 생성합니다.

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json`이 커밋되므로 생성된 카탈로그를 오프라인에서 재현할 수 있습니다. 해당 항목을 수정하고 생성기를 다시 실행하여 번역을 수정하세요.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## 소스에서 빌드

요구 사항: Rust `1.97.1`(`rust-toolchain.toml`으로 고정) 및 Linux 빌드의 경우 GTK3 및 AppIndicator 개발 헤더.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

플랫폼 패키지:

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

승인 스크립트는 모의 상태 대신 실제 바이너리를 구동합니다.

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## 선적 서류 비치

| 문서 | 내용물 |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | 운영, 패키징 및 플랫폼 동작 |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | 현재 테스트 증거 및 알려진 미확인 영역 |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | 현지 접수 게이트 |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | 행동을 바꾸기 전에 예상되는 반복 회귀 및 증거 추적 |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign 프로토타입에 대한 시각적 수렴 상태 |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android 루트 모듈과 그 한계 |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | 벤더 코어의 업스트림 출처 |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

일부 문서는 중국어로 작성되어 있습니다. 그들은 동의한 단어로 검증 실행과 규칙을 기록합니다.

## 적용 범위

주요 제품은 PC 클라이언트입니다. Android 지원은 해당 범위를 넘어 명시적으로 승격될 때까지 루트 전용, 헤드리스 수신 데몬입니다. GUI이 없고 포그라운드 서비스를 시작하지 않으며 지속적인 알림을 게시하지 않습니다. `crates/localsend-official-core`은 생성된 `SHA256SUMS` 파일을 제외하고 기록된 업스트림 내보내기와 바이트 단위로 동일하게 유지됩니다.

## 감사의 글

프로토콜, 유선 형식 및 Rust 코어는 [localsend/localsend](https://github.com/localsend/localsend)에서 나옵니다. 이 프로젝트는 프로토콜을 다시 구현하는 대신 해당 작업에 데스크톱 클라이언트를 구축합니다. LocalSend 작성자와 기여자에게 감사드립니다.

인터페이스는 이 프로젝트를 위해 개발된 디자인 프로토타입을 따릅니다. 시각적 참조는 [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)에 있습니다.

## 기여하기

- **버그 및 제안**: 버전, 플랫폼, 재현 단계 및 본 내용이 포함된 [문제](https://github.com/alkaid-ops/localsend-next/issues)를 엽니다. 로그나 스크린샷을 사용하면 조치를 취하는 것이 훨씬 더 빨라집니다.
- **코드**: 풀 요청을 환영합니다. daemon/frontend 경계, 벤더 코어 또는 시각적 계약에 관련된 변경 사항을 먼저 논의해야 합니다. 제약 조건은 [`AGENTS.md`](../../AGENTS.md)에 기록됩니다.
- **번역**: 카탈로그는 커밋된 기계 번역 캐시에서 생성되므로 수정 사항이 가장 쉽게 검토할 수 있습니다.

## 특허

[아파치-2.0](../../LICENSE). 타사 구성 요소 및 해당 라이선스는 [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md)에 나열되어 있습니다.
