# Setheum native Windows bootstrap.
#
# Detects the tools this repository needs and installs whatever is missing.
# Designed to be idempotent and safe to re-run.
#
# Usage:
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/bootstrap.ps1
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/bootstrap.ps1 -Check
#
# Invoked automatically by:  mise run init
#
# Exit codes:
#   0  everything present (or installed successfully)
#   1  one or more required tools could not be installed automatically

[CmdletBinding()]
param(
    [switch]$Check,
    [switch]$NoElevate
)

$ErrorActionPreference = 'Continue'
$ProgressPreference = 'SilentlyContinue'

# ---------------------------------------------------------------- output helpers
function Write-Step($m) { Write-Host "`n== $m ==" -ForegroundColor Cyan }
function Write-Ok($m)   { Write-Host "  [ok]      $m" -ForegroundColor Green }
function Write-Miss($m) { Write-Host "  [missing] $m" -ForegroundColor Yellow }
function Write-Note($m) { Write-Host "  [note]    $m" -ForegroundColor Gray }
function Write-Warn($m) { Write-Host "  [warn]    $m" -ForegroundColor Magenta }
function Write-Act($m)  { Write-Host "  [install] $m" -ForegroundColor White }

function Refresh-Path {
    $machine = [Environment]::GetEnvironmentVariable('Path', 'Machine')
    $user    = [Environment]::GetEnvironmentVariable('Path', 'User')
    $parts   = @($machine, $user) | Where-Object { $_ }
    if ($parts.Count -gt 0) { $env:Path = ($parts -join ';') }
}
Refresh-Path

function Test-Admin {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    (New-Object Security.Principal.WindowsPrincipal($id)).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator)
}
$IsAdmin = Test-Admin

function Get-Exe([string]$Name) {
    $c = Get-Command $Name -ErrorAction SilentlyContinue
    if ($c) { return $c.Source }
    return $null
}

function Test-RealPython {
    $c = Get-Command python -ErrorAction SilentlyContinue
    if ($c -and ($c.Source -notmatch 'WindowsApps')) { return $c.Source }
    $p = Get-Command py -ErrorAction SilentlyContinue
    if ($p) { return $p.Source }
    return $null
}

function Get-Msvc {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (Test-Path $vswhere) {
        $p = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
        if ($p) { return @($p)[0] }
    }
    $link = Get-Command link.exe -ErrorAction SilentlyContinue
    if ($link -and $link.Source -notmatch '\\Git\\') { return $link.Source }
    return $null
}

$Winget = Get-Exe 'winget'
$Choco  = Get-Exe 'choco'
$Missing = New-Object System.Collections.Generic.List[string]

function Install-Package {
    param(
        [string]$Name,
        [string]$WingetId,
        [string]$ChocoId
    )
    if ($Check) {
        Write-Note ("would install {0} ({1})" -f $Name, $WingetId)
        $Missing.Add($Name) | Out-Null
        return
    }
    if ($Winget) {
        Write-Act ("{0} via winget ({1})" -f $Name, $WingetId)
        & $Winget install -e --id $WingetId --accept-source-agreements --accept-package-agreements --disable-interactivity
        if ($LASTEXITCODE -eq 0) { Refresh-Path; Write-Ok "$Name installed"; return }
        Write-Warn ("winget failed for {0} (exit {1})" -f $Name, $LASTEXITCODE)
    }
    if ($Choco -and $ChocoId) {
        Write-Act ("{0} via chocolatey ({1})" -f $Name, $ChocoId)
        & $Choco install $ChocoId -y --no-progress
        if ($LASTEXITCODE -eq 0) { Refresh-Path; Write-Ok "$Name installed"; return }
        Write-Warn ("chocolatey failed for {0} (exit {1})" -f $Name, $LASTEXITCODE)
    }
    Write-Warn ("could not install {0} automatically; install it manually" -f $Name)
    $Missing.Add($Name) | Out-Null
}

# ---------------------------------------------------------------- environment
Write-Step 'Operating system'
$os = Get-CimInstance Win32_OperatingSystem
Write-Note ("{0} {1} ({2})" -f $os.Caption, $os.Version, $os.OSArchitecture)
Write-Note ("user: {0}  admin: {1}" -f [Security.Principal.WindowsIdentity]::GetCurrent().Name, $IsAdmin)
if (-not $Winget) { Write-Warn 'winget not found; only chocolatey/manual installs will be attempted' }
if ($Check) { Write-Note 'check-only mode: nothing will be installed' }

# ---------------------------------------------------------------- core tooling
Write-Step 'Core tooling'

if (Get-Exe 'mise') {
    Write-Ok ("mise {0}" -f (& mise --version 2>$null))
} else {
    Write-Miss 'mise'
    Install-Package -Name 'mise' -WingetId 'jdx.mise'
}

if (Get-Exe 'rustup') {
    Write-Ok ("rustup {0}" -f ((& rustup --version 2>$null) -split ' ')[1])
} else {
    Write-Miss 'rustup'
    Install-Package -Name 'rustup' -WingetId 'Rustlang.Rustup' -ChocoId 'rustup'
}

$msvc = Get-Msvc
if ($msvc) {
    Write-Ok "MSVC toolchain ($msvc)"
} else {
    Write-Miss 'Visual Studio / MSVC (C++ build tools)'
    Write-Note 'install "Desktop development with C++" (winget id: Microsoft.VisualStudio.2022.BuildTools)'
    $Missing.Add('MSVC build tools') | Out-Null
}

$llvmBin = 'C:\Program Files\LLVM\bin'
$clang = Get-Exe 'clang'
if (-not $clang -and (Test-Path (Join-Path $llvmBin 'clang.exe'))) { $clang = Join-Path $llvmBin 'clang.exe' }
if ($clang) {
    Write-Ok "LLVM ($clang)"
    $paths = @([Environment]::GetEnvironmentVariable('Path','Machine'), [Environment]::GetEnvironmentVariable('Path','User')) -join ';'
    if ($paths -notlike "*$llvmBin*") {
        if (-not $Check) {
            $userPath = [Environment]::GetEnvironmentVariable('Path','User')
            [Environment]::SetEnvironmentVariable('Path', (($userPath.TrimEnd(';')) + ';' + $llvmBin), 'User')
            Refresh-Path
            Write-Ok "added $llvmBin to user PATH"
        } else {
            Write-Note "would add $llvmBin to user PATH"
        }
    }
} else {
    Write-Miss 'LLVM (clang / libclang / lld-link)'
    Install-Package -Name 'LLVM' -WingetId 'LLVM.LLVM' -ChocoId 'llvm'
}

if (Get-Exe 'protoc') {
    Write-Ok ("protoc {0}" -f (& protoc --version 2>$null))
} else {
    Write-Miss 'protoc (Protocol Buffers compiler)'
    Install-Package -Name 'protobuf' -WingetId 'Google.Protobuf' -ChocoId 'protoc'
}

$buildTools = @(
    @{ Name = 'cmake'; Winget = 'Kitware.CMake';       Choco = 'cmake' },
    @{ Name = 'nasm';  Winget = 'NASM.NASM';           Choco = 'nasm'  },
    @{ Name = 'perl';  Winget = 'StrawberryPerl.StrawberryPerl'; Choco = 'strawberryperl' }
)
foreach ($t in $buildTools) {
    if (Get-Exe $t.Name) {
        Write-Ok "$($t.Name) present"
    } else {
        Write-Miss $t.Name
        Install-Package -Name $t.Name -WingetId $t.Winget -ChocoId $t.Choco
    }
}

# ---------------------------------------------------------------- rust targets
Write-Step 'Rust toolchains and WASM targets'
if (Get-Exe 'rustup') {
    $toolchains = @((& rustup toolchain list 2>$null) | ForEach-Object { ($_ -split ' ')[0] })

    $rustWanted = @(
        @{ Toolchain = '1.88.0';             Components = 'rustfmt,clippy,rust-src'; Targets = @('wasm32-unknown-unknown','wasm32v1-none') },
        @{ Toolchain = 'nightly-2024-02-14'; Components = 'rustfmt,clippy,rust-src'; Targets = @('wasm32-unknown-unknown') }
    )
    foreach ($r in $rustWanted) {
        $installed = @($toolchains | Where-Object { $_ -eq $r.Toolchain -or $_ -like "$($r.Toolchain)-*" })
        if (-not $installed) {
            if ($Check) {
                Write-Miss ("toolchain {0}" -f $r.Toolchain)
                $Missing.Add("rust $($r.Toolchain)") | Out-Null
                continue
            }
            & rustup toolchain install $r.Toolchain --profile minimal --component $r.Components
            if ($LASTEXITCODE -ne 0) {
                Write-Warn ("toolchain {0} install failed" -f $r.Toolchain)
                $Missing.Add("rust $($r.Toolchain)") | Out-Null
                continue
            }
        }
        $haveTargets = @((& rustup target list --installed --toolchain $r.Toolchain 2>$null))
        $missingTargets = @($r.Targets | Where-Object { $haveTargets -notcontains $_ })
        if ($missingTargets.Count -eq 0) {
            Write-Ok ("toolchain {0} + wasm targets" -f $r.Toolchain)
        } elseif ($Check) {
            Write-Miss ("toolchain {0} targets: {1}" -f $r.Toolchain, ($missingTargets -join ', '))
            $Missing.Add("rust $($r.Toolchain) targets") | Out-Null
        } else {
            & rustup target add @($missingTargets) --toolchain $r.Toolchain
            Write-Ok ("toolchain {0} + wasm targets" -f $r.Toolchain)
        }
    }
    if (Test-Path 'rust-toolchain.toml') { Write-Note 'repository pins Rust 1.88.0 via rust-toolchain.toml' }
} else {
    Write-Miss 'rustup (cannot configure toolchains)'
    $Missing.Add('rustup') | Out-Null
}

# ---------------------------------------------------------------- dev settings
Write-Step 'Developer settings'
$lpKey = 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem'
$dmKey = 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock'
$longPaths = (Get-ItemProperty $lpKey -Name LongPathsEnabled -ErrorAction SilentlyContinue).LongPathsEnabled
$devMode   = (Get-ItemProperty $dmKey -Name AllowDevelopmentWithoutDevLicense -ErrorAction SilentlyContinue).AllowDevelopmentWithoutDevLicense

$needReg = @()
if ($longPaths -ne 1) { $needReg += 'LongPathsEnabled' }
if ($devMode -ne 1)   { $needReg += 'AllowDevelopmentWithoutDevLicense' }

if ($needReg.Count -eq 0) {
    Write-Ok 'long paths + developer mode enabled'
} elseif ($Check) {
    Write-Miss ("would enable: {0}" -f ($needReg -join ', '))
    $Missing.Add('long paths / developer mode') | Out-Null
} elseif ($NoElevate) {
    Write-Warn ("run elevated to enable: {0}" -f ($needReg -join ', '))
    $Missing.Add('long paths / developer mode') | Out-Null
} else {
    $tmp = Join-Path $env:TEMP 'setheum-devsettings.ps1'
    $body = @'
$ErrorActionPreference = 'Stop'
New-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem' -Name 'LongPathsEnabled' -Value 1 -PropertyType DWord -Force | Out-Null
New-Item -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock' -Force | Out-Null
New-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock' -Name 'AllowDevelopmentWithoutDevLicense' -Value 1 -PropertyType DWord -Force | Out-Null
'@
    Set-Content -Path $tmp -Value $body -Encoding ASCII
    Write-Act 'enabling long paths + developer mode (UAC prompt expected)'
    try {
        Start-Process -FilePath 'powershell.exe' -Verb RunAs -Wait `
            -ArgumentList '-NoProfile','-ExecutionPolicy','Bypass','-File',$tmp
        $longPaths = (Get-ItemProperty $lpKey -Name LongPathsEnabled -ErrorAction SilentlyContinue).LongPathsEnabled
        $devMode   = (Get-ItemProperty $dmKey -Name AllowDevelopmentWithoutDevLicense -ErrorAction SilentlyContinue).AllowDevelopmentWithoutDevLicense
        if ($longPaths -eq 1) { Write-Ok 'long paths enabled' } else { Write-Warn 'long paths not enabled' }
        if ($devMode -eq 1)   { Write-Ok 'developer mode enabled' } else { Write-Warn 'developer mode not enabled' }
    } catch {
        Write-Warn ("elevation declined or failed: {0}" -f $_.Exception.Message)
        $Missing.Add('long paths / developer mode') | Out-Null
    }
}

if (Get-Exe 'git') {
    if (-not $Check) {
        & git config --global core.longpaths true
        Write-Ok 'git core.longpaths=true'
    } else {
        Write-Note 'would set git core.longpaths=true'
    }
}

# ---------------------------------------------------------------- summary
Write-Step 'Summary'
if ($Missing.Count -eq 0) {
    Write-Ok 'all required tooling is present'
    exit 0
}
Write-Warn ("missing / manual action required: {0}" -f (($Missing | Select-Object -Unique) -join ', '))
exit 1
