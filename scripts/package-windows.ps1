param(
    [ValidateSet("x64", "arm64")]
    [string]$Architecture = "x64"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$Root = Split-Path -Parent $PSScriptRoot
$Version = if ([string]::IsNullOrWhiteSpace($env:PACKAGE_VERSION)) {
    "0.1.0"
} else {
    $env:PACKAGE_VERSION.Trim().TrimStart("v")
}
if ($Version -notmatch '^[0-9A-Za-z][0-9A-Za-z._-]*$') {
    throw "invalid PACKAGE_VERSION: $Version"
}
$Target = if ($Architecture -eq "x64") { "x86_64-pc-windows-msvc" } else { "aarch64-pc-windows-msvc" }
$Release = Join-Path $Root "target/$Target/release"
$Packages = Join-Path $Root "target/release/packages"
$Stage = Join-Path $Packages "LocalSend-$Version-windows-$Architecture"
$Zip = Join-Path $Packages "LocalSend-$Version-windows-$Architecture.zip"
$GuiSource = Join-Path $Release "localsend-gui-slint.exe"
$Required = @(
    $GuiSource,
    (Join-Path $Release "localsendd.exe"),
    (Join-Path $Release "localsend-tray.exe"),
    (Join-Path $Release "localsend-launcher.exe")
)

$PreviousRustFlags = $env:RUSTFLAGS
$StaticCrt = "-C target-feature=+crt-static"
$env:RUSTFLAGS = if ([string]::IsNullOrWhiteSpace($PreviousRustFlags)) {
    $StaticCrt
} else {
    "$PreviousRustFlags $StaticCrt"
}
try {
    & cargo build --locked --release --target $Target `
        -p localsendd -p localsend-tray -p localsend-launcher
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed for daemon and tray" }
    & cargo build --locked --release --target $Target `
        -p localsend-gui-slint --no-default-features --features backend-femtovg-wgpu
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed for GUI" }
} finally {
    $env:RUSTFLAGS = $PreviousRustFlags
}

foreach ($Path in $Required) {
    if (-not (Test-Path $Path)) { throw "missing build output: $Path" }
}

if (Test-Path $Stage) { Remove-Item -Recurse -Force $Stage }
if (Test-Path $Zip) { Remove-Item -Force $Zip }
New-Item -ItemType Directory -Path $Stage | Out-Null

Copy-Item $GuiSource (Join-Path $Stage "LocalSend.exe")
Copy-Item (Join-Path $Release "localsendd.exe") $Stage
Copy-Item (Join-Path $Release "localsend-tray.exe") $Stage
Copy-Item (Join-Path $Release "localsend-launcher.exe") $Stage
Copy-Item (Join-Path $Root "LICENSE") $Stage
Copy-Item (Join-Path $Root "THIRD_PARTY_NOTICES.md") $Stage
Copy-Item (Join-Path $Root "apps/localsend-gui-slint/packaging/windows/README.txt") $Stage
New-Item -ItemType Directory -Path (Join-Path $Stage "assets") | Out-Null
Copy-Item (Join-Path $Root "apps/localsend-gui-slint/packaging/windows/app.ico") (Join-Path $Stage "assets/app.ico")

function Get-PeMetadata([string]$Path) {
    $stream = [System.IO.File]::OpenRead($Path)
    try {
        $reader = [System.Reflection.PortableExecutable.PEReader]::new($stream)
        return @{
            Machine = $reader.PEHeaders.CoffHeader.Machine
            Subsystem = $reader.PEHeaders.PEHeader.Subsystem
        }
    } finally {
        if ($null -ne $reader) { $reader.Dispose() }
        $stream.Dispose()
    }
}

$ExpectedMachine = if ($Architecture -eq "x64") {
    [System.Reflection.PortableExecutable.Machine]::Amd64
} else {
    [System.Reflection.PortableExecutable.Machine]::Arm64
}
foreach ($Name in @("LocalSend.exe", "localsendd.exe", "localsend-tray.exe", "localsend-launcher.exe")) {
    $Metadata = Get-PeMetadata (Join-Path $Stage $Name)
    if ($Metadata.Machine -ne $ExpectedMachine) {
        throw "$Name has PE machine $($Metadata.Machine), expected $ExpectedMachine"
    }
}

foreach ($Name in @("LocalSend.exe", "localsend-tray.exe", "localsend-launcher.exe")) {
    $Metadata = Get-PeMetadata (Join-Path $Stage $Name)
    if ($Metadata.Subsystem -ne [System.Reflection.PortableExecutable.Subsystem]::WindowsGui) {
        throw "$Name must use the Windows GUI subsystem"
    }
}

Compress-Archive -Path (Join-Path $Stage "*") -DestinationPath $Zip -CompressionLevel Optimal
if (-not (Test-Path $Zip)) { throw "ZIP was not created" }
Write-Output $Zip
