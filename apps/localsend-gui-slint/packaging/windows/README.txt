LocalSend for Windows
=====================

Run LocalSend.exe to start the GUI. The tray process owns the daemon, and both
helpers must remain next to LocalSend.exe:

  LocalSend.exe
  localsend-tray.exe
  localsendd.exe

The package is portable. Settings and TLS identity are stored in the current
user's application data directory. Allow LocalSend through Windows Defender
Firewall when prompted so nearby devices can discover this computer.

This build is ad-hoc/unsigned and is intended for local testing.
