# localsend-next repository rules

- Before debugging or changing behavior, read
  `docs/agent-regression-playbook.zh-CN.md`. It records recurring regressions,
  false-positive tests, forbidden workarounds, and the expected evidence trail.

- The primary product is a PC LocalSend-compatible client. Android support is a
  root-only, headless receive MVP until explicitly promoted beyond that scope.
- The only formal GUI is `apps/localsend-gui-slint` using Slint winit and FemtoVG.
- The egui, FLTK, Iced, and Tauri directories are experimental and excluded from
  the root workspace until migrated to `localsend-rpc`.
- Runtime architecture is frontends -> `localsend-rpc` -> `localsendd` ->
  vendored official LocalSend core.
- Frontends must not depend on the vendored core and must not spawn `localsendd`.
- `localsend-tray` owns daemon and GUI process lifecycle.
- `apps/localsend-androidd` is a standalone Android native boot daemon. It is
  not a GUI, must not start a Foreground Service, and must not create a
  persistent service notification. Its Magisk/KernelSU supervisor owns startup
  and restart.
- The Android MVP uses SAF-free root storage at
  `/storage/emulated/0/Download/LocalSend`. Never place its config inside the
  desktop app data directory.
- Do not add an Android `init.rc` or custom SELinux domain until a rooted device
  is available to verify boot-stage loading, policy, and uninstall behavior.
- System notifications are emitted only by `localsend-tray`. Windows and Linux use
  `notify-rust`; Windows notification identity and history cleanup are owned by
  `localsend-platform`. Do not add a second notification path in the GUI.
- The daemon owns settings, identity, favorites, history, transfers, and TLS
  identity persistence.
- Keep `crates/localsend-official-core` byte-for-byte identical to the recorded
  upstream export except for its generated `SHA256SUMS` file.
- Do not copy generated output, local databases, app bundles, `target/`, `tmp/`,
  or graphify output into the repository.
- Test fixtures that spawn an isolated `localsendd` must record its PID in the
  fixture manifest and explicitly terminate that PID during cleanup, including
  aborted, failed, and interrupted runs. After every E2E run, verify that no
  `target/debug/localsendd`, `native_e2e_fixture`, or test GUI process remains.
- Isolated test daemons must not join or announce on the real LocalSend multicast
  groups unless the test is specifically validating discovery. Test-only daemons
  otherwise set `LOCALSEND_TEST_DISABLE_DISCOVERY=1` so they cannot appear as
  extra devices in official clients.
- Never start LocalSend app processes with `launchctl submit`: it creates a
  keep-alive job and relaunches the tray after Quit. Use the app bundle or a
  foreground/development process instead.
- Tray quit is one instance-level operation: terminate the GUI recorded in the
  endpoint-scoped `gui.pid`, stop the owned daemon with a bounded timeout, and
  exit. It must not leave a GUI, daemon, tray, or launchd keep-alive job behind.
