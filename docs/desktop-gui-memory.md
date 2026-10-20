# PC GUI memory measurements

This records the idle RSS of the desktop GUI prototypes. The daemon is not
included. Each frontend connects to one isolated `localsendd` over a temporary
Unix socket; that daemon is configured with `port = 0` and
`start_server_on_launch = false`.

## Environment and method

- Date: 2026-10-18
- Host: macOS 14.8.7, Apple Silicon arm64
- Build: `cargo build --release`
- State: first window open, no transfer activity
- Sample time: 5 seconds after launch; Tauri uses 8 seconds
- Samples: three runs per configuration, median reported
- Metric: resident set size (`ps -o rss`)
- Tauri: the main process plus WebKit `WebContent`, `Networking`, and `GPU`
  XPC processes created by that launch

RSS counts shared pages and is not the same as private physical memory. These
numbers are useful for comparing implementations on this machine, not as a
cross-platform guarantee.

## Results

| Configuration | Median RSS | Relative to TUI |
|---|---:|---:|
| TUI | 4.5 MiB | 1.0x |
| Iced `wgpu` | 48.3 MiB | 10.7x |
| Slint `winit-femtovg` | 50.9 MiB | 11.2x |
| egui `wgpu`, no CJK font | 62.5 MiB | 13.8x |
| egui `glow`, no CJK font | 64.1 MiB | 14.1x |
| Tauri main process only | 72.0 MiB | 15.9x |
| Slint `winit-software` | 85.3 MiB | 18.8x |
| Iced `tiny-skia` | 86.0 MiB | 19.0x |
| egui `wgpu`, Hiragino CJK | 88.0 MiB | 19.4x |
| egui `glow`, Hiragino CJK | 94.9 MiB | 20.9x |
| FLTK bundled | 96.4 MiB | 21.3x |
| egui `wgpu`, PingFang CJK | 138.3 MiB | 30.5x |
| egui `glow`, PingFang CJK | 144.6 MiB | 31.9x |
| Tauri main plus WebKit | 156.5 MiB | 34.5x |
| Slint `winit-skia` | 229.8 MiB | 50.7x |
| Slint `winit-skia-opengl` | 229.6 MiB | 50.7x |

The egui PingFang result is dominated by loading the complete 74 MiB
`PingFang.ttc`. A production app should embed a subset or use a smaller system
font strategy. The no-CJK rows isolate toolkit overhead and are not a complete
Chinese UI.

## Slint backend sensitivity

The same Slint UI produced materially different results depending on renderer:

| Renderer | Median RSS |
|---|---:|
| `winit-femtovg` | 50.9 MiB |
| `winit-software` | 85.3 MiB |
| `winit-skia` | 229.8 MiB |
| `winit-skia-opengl` | 229.6 MiB |

This means "Slint memory" is not a single number. The renderer and platform must
be selected and measured separately.

An older community report on Windows measured roughly:

| Renderer | Reported memory |
|---|---:|
| `winit-software` | 14 MB |
| FemtoVG | 51.1 MB |
| Skia OpenGL | 52.6 MB |
| Skia Direct3D | 17.5 MB |

Those values are from a different OS, Slint version, toolchain, and measurement
method, so they are not directly comparable with the table above. They reinforce
the same conclusion: the backend can change memory by tens or hundreds of MiB.

## Final selection

`Slint` with the `winit-femtovg` backend is the selected GUI implementation.

The decision is not based on a single lowest number. Iced `wgpu` measured
slightly lower on this macOS host, but Slint wins on the overall desktop path:

- FemtoVG is within roughly 2.6 MiB of the lowest measured GUI configuration.
- Slint stays a single native Rust process and does not require a WebView stack.
- The renderer is a build feature, so Windows can be measured and switched to
  `winit-software` or Skia Direct3D if the reported low-memory behavior
  reproduces.
- The same Slint UI code remains usable across renderer experiments without
  changing the daemon/RPC architecture.

`apps/localsend-gui-slint` therefore defaults to `backend-femtovg`. The migration to
`localsend-next` keeps this renderer choice; measurements above remain historical
evidence from the predecessor workspace.

## Practical reading

- TUI remains far smaller than every GUI.
- Iced `wgpu` and Slint FemtoVG are the only tested GUI configurations near
  50 MiB on this macOS host.
- Software rendering is not automatically the low-memory choice on macOS.
- Tauri must include its WebKit processes; counting only the Rust main process
  understates it by roughly 85 MiB.
- FLTK was not the lightest result here because the bundled build and font stack
  still bring a substantial native toolkit footprint.
- Cross-platform selection needs a backend matrix. A backend that wins on
  macOS may lose on Windows or Linux.
