LocalSend for Windows
=====================

Run localsend-tray.exe to start LocalSend. The tray starts the daemon and opens
the GUI when requested. The GUI can also be started directly when a daemon is
already running. Keep all helpers next to LocalSend.exe:

  LocalSend.exe
  localsend-tray.exe
  localsendd.exe
  localsend-launcher.exe

The package is portable. Settings and TLS identity are stored in the current
user's application data directory. Allow LocalSend through Windows Defender
Firewall when prompted so nearby devices can discover this computer.

This build is ad-hoc/unsigned and is intended for local testing.
