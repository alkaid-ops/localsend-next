<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | **日本語** | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next は、Rust で記述された LocalSend 互換のデスクトップ クライアントです。公式プロトコルを使用するため、Flutter アプリではなくネイティブ デスクトップ プログラムとして実行しながら、公式 LocalSend アプリを使用してファイルを両方向に検出して転送します。

ネットワーク側は、変更されていない公式の LocalSend Rust コアです。その上のすべて (デーモン、RPC レイヤー、トレイ、ターミナル UI、および Slint GUI) は、そのコアを中心に構築されています。

## デモ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| 送信 | 設定 |
| --- | --- |
| ![ファイル、フォルダー、テキスト、クリップボードのソースと近くのデバイスを含むページを送信](../../docs/images/readme-send-light.png) | ![テーマ、言語、ネットワーク、転送オプションを含む設定ページ](../../docs/images/readme-settings-light.png) |

## 機能

- 再実装ではなくベンダーのアップストリーム コア上で、公式 LocalSend クライアントに対する検出と転送
- 1 つの正規状態所有者: デーモンは設定、ID、お気に入り、履歴、転送、および TLS ID を保持するため、すべてのフロントエンドが同じものを認識します。
- GUI、同じローカル RPC 上の独立したフロントエンドとしての端末 UI とトレイ。トレイはデーモンと GUI プロセスのライフサイクルを所有します
- ファイルごとのリアルタイムの進行状況、開くアクションと公開アクションの履歴、お気に入り、手動デバイス入力、Web 共有受信リンク
- 上流の LocalSend のリストに一致する 57 のインターフェイス言語。実行時またはシステム ロケールに従って切り替え可能
- ネイティブ デスクトップ統合: トレイからのみの通知、ログイン時に起動、トレイへの最小化、ウィンドウ配置の永続化、タイトル バーに引き込まれる macOS ウィンドウ
- macOS (DMG/ZIP)、Linux (deb、rpm、AppImage、tarball)、および Windows (ZIP) (x86_64 と aarch64 の両方) のパッケージ化

## アーキテクチャ

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

デーモンは、ネットワークと通信したり、アプリケーションの状態を書き込んだりする唯一のプロセスです。フロントエンドはベンダーのコアに依存したり、デーモン自体を生成したりすることはありません。トレイはそうします。ファイルのバイトは RPC を超えて移動しません。デーモンはパスを解決し、コンテンツをハッシュし、履歴を記録します。

| バイナリ | 役割 |
| --- | --- |
| `localsendd` | ネットワーク サービスと正規のアプリケーションの状態 |
| `localsend-tray` | デーモンと GUI ライフサイクル所有者、通知 |
| `localsend-gui-slint` | FemtoVG レンダラーを使用した正式な GUI、Slint |
| `localsend-tui` | ターミナルフロントエンド |
| `localsend-launcher` | トレイのない環境用の明示的なランチャー |
| `localsend-androidd` | スタンドアロンの root 専用 Android 受信デーモン (MVP) |

`localsend-gui-egui`、`-fltk`、`-iced`、`-tauri` ディレクトリは実験用ポートであり、RPC の統合が完了するまでルート ワークスペースの外に置かれます。

## ダウンロード

リリース アセットは、プラットフォーム `linux` / `windows` / `macos` およびアーキテクチャ `x86_64` / `aarch64` の 1 つの命名スキームに従います — `LocalSend-<version>-<platform>-<arch>.<ext>`:

| プラットフォーム | 資産 |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`、`LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`、`LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`、`LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`、`LocalSend-0.1.3-linux-aarch64.rpm` |
| 任意の Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`、`LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` は、リリース内のすべてのアセットをカバーします。 CI によって生成された macOS ビルドは、開発者 ID による署名や公証ではなく、アドホック署名されています。

## クイックスタート

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

Linux パッケージは、デーモンのライフサイクルを所有する `localsend-tray` を起動するデスクトップ エントリをインストールします。ステータス バーのないデスクトップでは、代わりにトレイが GUI を開いたままにします。 `LocalSend --launcher` は AppImage からそのモードを明示的に選択します。

### Windows

解凍して `LocalSend.exe` を実行します。アーカイブには、GUI、デーモン、トレイ、およびランチャーが並べて保存されます。それらを一緒に保ちます。

## 知っておきたい動作

- **終了は 1 つの操作です。** トレイから終了すると、エンドポイント スコープの `gui.pid` に記録された GUI が終了し、制限付きタイムアウト内で所有デーモンが停止し、プロセスや launchd keep-alive ジョブを残さずに終了します。
- **通知はトレイから送信されます。** GUI は独自の通知を投稿しないため、各プラットフォームには単一の配信パスがあります。
- **ウィンドウの状態はエンドポイントごとです。** 配置は共有グローバルな場所ではなく、そのエンドポイントのデーモン設定の横に保存されるため、2 つのインスタンスがウィンドウ ジオメトリをめぐって競合することはありません。
- **デーモンには権限があります。** GUI で変更された設定はデーモンに送信され、スナップショットを通じてエコー バックされます。フロントエンドは、漂流する可能性のあるプライベートコピーを保持しません。

## 言語

このインターフェイスは、アップストリームの LocalSend のリストに一致する 57 のロケールで出荷され、msgid 言語として簡体字中国語が使用されます。 **[設定] → [言語]** でいずれかを選択するか、[システムに従う] のままにして OS ロケールを使用します。

| | |
| --- | --- |
| 同梱カタログ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UIソース文字列 | `@tr("...")` の `apps/localsend-gui-slint/ui` |
| 英語カタログ | `translations/locale-en.json` (UI)、`translations/locale-en-runtime.json` (Rust メッセージ) |
| Rustサイドのメッセージ | `crates/localsend-i18n` (生成された `lookup_generated.rs`) |
| アップストリームミラー | `translations/upstream/<locale>.json` |

UI 文字列を変更した後、カタログを再生成します。

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` はコミットされるため、生成されたカタログはオフラインで再現できます。そこのエントリを修正し、ジェネレータを再実行して翻訳を修正します。

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ソースからのビルド

要件: Rust `1.97.1` (`rust-toolchain.toml` によって固定)、および Linux ビルドの場合は、GTK3 および AppIndicator 開発ヘッダー。

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

プラットフォーム パッケージ:

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

受け入れスクリプトは、モック状態ではなく実際のバイナリを駆動します。

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ドキュメント

| 書類 | コンテンツ |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | 操作、パッケージ化、およびプラットフォームの動作 |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | 現在のテスト証拠と既知の未検証領域 |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | 現地の受け入れゲート |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | 再発する回帰と行動を変える前に予想される証拠の痕跡 |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign プロトタイプに対する視覚的な収束状態 |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android ルート モジュールとその制限 |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | ベンダーコアの上流の来歴 |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

一部の文書は中国語で書かれています。彼らは検証の実行と規約を合意された言葉で記録します。

## 対象範囲

主な製品は PC クライアントです。 Android サポートは、そのスコープを超えて明示的に昇格されるまでは、ルート専用のヘッドレス受信デーモンです。GUI はなく、フォアグラウンド サービスも開始されず、永続的な通知も送信されません。 `crates/localsend-official-core` は、生成された `SHA256SUMS` ファイルを除き、記録されたアップストリーム エクスポートとバイト単位で同一に保たれます。

## 謝辞

プロトコル、ワイヤ形式、および Rust コアは [localsend/localsend](https://github.com/localsend/localsend) から取得されます。このプロジェクトは、プロトコルを再実装するのではなく、その作業に基づいてデスクトップ クライアントを構築します。 LocalSend の著者と寄稿者に感謝します。

インターフェイスは、このプロジェクトのために開発されたデザイン プロトタイプに従っています。その視覚的な参照は [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) にあります。

## コントリビュート

- **バグと提案**: バージョン、プラットフォーム、再現手順、確認した内容を記載した [問題](https://github.com/alkaid-ops/localsend-next/issues) をオープンしてください。ログやスクリーンショットがあると、より迅速に行動できるようになります。
- **コード**: プル リクエストは歓迎です。 daemon/frontend の境界、ベンダーのコア、またはビジュアル コントラクトに触れる変更については、最初に議論する必要があります。制約は [`AGENTS.md`](../../AGENTS.md) に記録されます。
- **翻訳**: カタログはコミットされた機械翻訳キャッシュから生成されるため、そこでの修正はレビューするのが最も簡単な種類の貢献です。

## ライセンス

[Apache-2.0](../../LICENSE)。サードパーティのコンポーネントとそのライセンスは [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) にリストされています。
