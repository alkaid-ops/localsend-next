<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | **Português (Brasil)** | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next é um cliente de desktop compatível com LocalSend escrito em Rust. Ele fala o protocolo oficial, portanto descobre e transfere arquivos com o aplicativo oficial LocalSend em ambas as direções, enquanto é executado como um programa de desktop nativo em vez de um aplicativo Flutter.

O lado da rede é o núcleo LocalSend Rust oficial, vendido inalterado. Tudo acima dele - daemon, camada RPC, bandeja, UI do terminal e Slint GUI - é construído em torno desse núcleo.

## Demonstração

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Enviar | Configurações |
| --- | --- |
| ![Enviar página com as fontes do arquivo, pasta, texto e área de transferência e dispositivos próximos](../../docs/images/readme-send-light.png) | ![Página de configurações com opções de tema, idioma, rede e transferência](../../docs/images/readme-settings-light.png) |

## Recursos

- Descoberta e transferência contra clientes LocalSend oficiais, no núcleo upstream vendido, em vez de uma reimplementação
- Um proprietário de estado canônico: o daemon contém configurações, identidade, favoritos, histórico, transferências e a identidade TLS, para que cada frontend veja a mesma coisa
- GUI, UI do terminal e bandeja como frontends independentes no mesmo local RPC; a bandeja possui o daemon e o ciclo de vida do processo GUI
- Progresso por arquivo em tempo real, histórico com ações de abertura e revelação, favoritos, entrada manual de dispositivo e links de recebimento de compartilhamento na Web
- 57 idiomas de interface correspondentes à lista upstream LocalSend, alternáveis ​​em tempo de execução ou seguindo a localidade do sistema
- Integração nativa com a área de trabalho: notificações apenas da bandeja, inicialização no login, minimização para a bandeja, persistência no posicionamento da janela e uma janela macOS que aparece na barra de título
- Empacotamento para macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) e Windows (ZIP), para x86_64 e aarch64

## Arquitetura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

O daemon é o único processo que se comunica com a rede ou grava o estado do aplicativo. Os front-ends nunca dependem do núcleo fornecido e nunca geram o daemon por conta própria; a bandeja faz. Os bytes do arquivo não passam por RPC — o daemon resolve caminhos, faz hash de conteúdo e registra histórico.

| Binário | Papel |
| --- | --- |
| `localsendd` | Serviço de rede e estado canônico do aplicativo |
| `localsend-tray` | Daemon e GUI proprietário do ciclo de vida, notificações |
| `localsend-gui-slint` | O formal GUI, Slint com o renderizador FemtoVG |
| `localsend-tui` | Interface do terminal |
| `localsend-launcher` | Launcher explícito para ambientes sem bandeja |
| `localsend-androidd` | Daemon de recebimento independente somente root Android (MVP) |

Os diretórios `localsend-gui-egui`, `-fltk`, `-iced` e `-tauri` são portas experimentais e ficam fora do espaço de trabalho raiz até que sua integração RPC seja concluída.

## Download

Os ativos de lançamento seguem um esquema de nomenclatura — `LocalSend-<version>-<platform>-<arch>.<ext>`, com plataformas `linux` / `windows` / `macos` e arquiteturas `x86_64` / `aarch64`:

| Plataforma | Ativo |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Qualquer Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` abrange todos os ativos em uma versão. A compilação macOS produzida pela CI é assinada ad hoc, não assinada pelo ID do desenvolvedor ou autenticada.

## Início rápido

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

Os pacotes Linux instalam uma entrada de desktop que inicia `localsend-tray`, que possui o ciclo de vida do daemon. Em desktops sem barra de status, a bandeja mantém GUI aberto; `LocalSend --launcher` seleciona esse modo explicitamente em AppImage.

### Windows

Descompacte e execute `LocalSend.exe`. O arquivo contém o GUI, o daemon, a bandeja e o inicializador lado a lado; mantenha-os juntos.

## Comportamentos importantes

- **Sair é uma operação.** Sair da bandeja encerra o GUI registrado no `gui.pid` com escopo de endpoint, interrompe o daemon de propriedade dentro de um tempo limite limitado e sai sem deixar um processo ou um trabalho de manutenção de atividade iniciado para trás.
- **As notificações vêm da bandeja.** O GUI nunca publica suas próprias notificações, então cada plataforma tem um único caminho de entrega.
- **O estado da janela é por endpoint.** O posicionamento é armazenado ao lado da configuração do daemon para esse endpoint, em vez de em um local global compartilhado, para que duas instâncias não briguem pela geometria da janela.
- **O daemon é autoritativo.** As configurações alteradas em GUI são enviadas ao daemon e repetidas por meio de um instantâneo; um frontend não mantém nenhuma cópia privada que possa ser desviada.

## Idiomas

A interface é fornecida em 57 localidades, correspondendo à lista upstream LocalSend, com chinês simplificado como idioma msgid. Escolha um em **Configurações → Idioma** ou deixe-o em "Seguir sistema" para usar a localidade do sistema operacional.

| | |
| --- | --- |
| Catálogos agrupados | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Strings de origem da IU | `@tr("...")` em `apps/localsend-gui-slint/ui` |
| Catálogo inglês | `translations/locale-en.json` (IU), `translations/locale-en-runtime.json` (Rust mensagens) |
| Mensagens do lado da ferrugem | `crates/localsend-i18n` (gerado `lookup_generated.rs`) |
| Espelho a montante | `translations/upstream/<locale>.json` |

Gere novamente os catálogos após alterar qualquer string da UI:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` é confirmado, portanto os catálogos gerados podem ser reproduzidos offline. Corrija uma entrada e execute novamente o gerador para corrigir uma tradução.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construindo a partir da fonte

Requisitos: Rust `1.97.1` (fixado por `rust-toolchain.toml`) e, para compilações Linux, os cabeçalhos de desenvolvimento GTK3 e AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pacotes de plataforma:

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

Os scripts de aceitação orientam os binários reais em vez do estado simulado:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentação

| Documento | Conteúdo |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operação, embalagem e comportamento da plataforma |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Evidências de testes atuais e áreas conhecidas não verificadas |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Portões de aceitação locais |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regressões recorrentes e a trilha de evidências esperada antes da mudança de comportamento |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Estado de convergência visual em relação ao protótipo OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | O módulo raiz Android e seus limites |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Proveniência upstream do núcleo vendido |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Alguns documentos estão escritos em chinês; eles registram execuções de verificação e convenções nas palavras com as quais foram acordados.

## Escopo

O produto principal é o cliente de PC. O suporte Android é um daemon de recebimento sem cabeça e somente root até que seja explicitamente promovido além desse escopo - ele não tem GUI, não inicia nenhum serviço em primeiro plano e não publica nenhuma notificação persistente. `crates/localsend-official-core` é mantido byte por byte idêntico à sua exportação upstream gravada, exceto pelo arquivo `SHA256SUMS` gerado.

## Créditos

O protocolo, o formato do fio e o núcleo Rust vêm de [localsend/localsend](https://github.com/localsend/localsend); este projeto cria um cliente de desktop nesse trabalho, em vez de reimplementar o protocolo. Obrigado aos LocalSend autores e colaboradores.

A interface segue um protótipo de design desenvolvido para este projeto; sua referência visual reside em [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Como contribuir

- **Bugs e sugestões**: abra um [issue](https://github.com/alkaid-ops/localsend-next/issues) com a versão, plataforma, etapas de reprodução e o que você viu. Um registro ou captura de tela torna a ação muito mais rápida.
- **Código**: solicitações pull são bem-vindas. Mudanças que afetam o limite daemon/frontend, o núcleo do fornecedor ou o contrato visual devem ser discutidas primeiro — as restrições são registradas em [`AGENTS.md`](../../AGENTS.md).
- **Traduções**: os catálogos são gerados a partir do cache de tradução automática confirmado, portanto as correções são o tipo de contribuição mais fácil de revisar.

## Licença

[Apache-2.0](../../LICENSE). Os componentes de terceiros e suas licenças estão listados em [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).
