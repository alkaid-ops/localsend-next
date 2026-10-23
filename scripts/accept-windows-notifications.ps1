param(
    [ValidateSet("x64", "arm64")]
    [string]$Architecture = "x64",
    [string]$AppPath = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

if ($env:OS -ne "Windows_NT") {
    throw "This acceptance script must run natively on Windows 10/11."
}

if (-not $AppPath) {
    $AppPath = Join-Path $PSScriptRoot "../target/release/packages/LocalSend-windows-$Architecture/LocalSend.exe"
}
$AppPath = (Resolve-Path $AppPath).Path
$TrayPath = Join-Path (Split-Path $AppPath) "localsend-tray.exe"
$DaemonPath = Join-Path (Split-Path $AppPath) "localsendd.exe"
foreach ($Path in @($AppPath, $TrayPath, $DaemonPath)) {
    if (-not (Test-Path $Path)) { throw "missing executable: $Path" }
}

Start-Process -FilePath $AppPath
$deadline = (Get-Date).AddSeconds(10)
do {
    $Tray = Get-Process -Name "localsend-tray" -ErrorAction SilentlyContinue
    $Daemon = Get-Process -Name "localsendd" -ErrorAction SilentlyContinue
    if ($Tray -and $Daemon) { break }
    Start-Sleep -Milliseconds 250
} while ((Get-Date) -lt $deadline)
if (-not $Tray -or -not $Daemon) {
    throw "tray or daemon did not start"
}

$Identity = Get-ItemProperty -Path "HKCU:\Software\Classes\AppUserModelId\org.localsend.pc.prototype" -ErrorAction Stop
if ($Identity.DisplayName -ne "LocalSend") {
    throw "AppUserModelId DisplayName is invalid"
}
$Shortcut = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\LocalSend.lnk"
if (-not (Test-Path $Shortcut)) { throw "notification Start Menu shortcut is missing" }

Write-Host "Send one text message and one file from LocalSend 1.18.2 or another peer."
Write-Host "Verify that both toasts show the LocalSend identity and play a sound."
Write-Host "Click each toast and verify the message/folder opens through the GUI."
$answer = Read-Host "Type PASS only if every check succeeded"
if ($answer -cne "PASS") {
    throw "manual notification acceptance was not confirmed"
}

Write-Output "result: passed"
