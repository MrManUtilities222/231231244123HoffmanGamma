param(
    [string]$Target = "windows"
)

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path | Split-Path -Parent
$Rp = Join-Path $Root 'rust-port'
$Data = Join-Path $Root 'data'
$Dist = Join-Path $Root 'dist'

if (-not (Test-Path $Rp)) { Write-Error "Could not find $Rp"; exit 1 }

Push-Location $Rp
cargo build --release
Pop-Location

New-Item -ItemType Directory -Force -Path $Dist | Out-Null

switch ($Target) {
    'windows' {
        $pkg = Join-Path $Dist 'rustcraft-windows-x86_64'
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $pkg
        New-Item -ItemType Directory -Path $pkg | Out-Null
        Copy-Item -Path (Join-Path $Rp 'target\release\rustcraft.exe') -Destination $pkg -ErrorAction SilentlyContinue
        if (-not (Test-Path (Join-Path $pkg 'rustcraft.exe'))) { Copy-Item -Path (Join-Path $Rp 'target\release\rustcraft') -Destination $pkg -ErrorAction SilentlyContinue }
        Copy-Item -Path $Data -Destination (Join-Path $pkg 'data') -Recurse
        $zip = Join-Path $Dist 'rustcraft-windows-x86_64.zip'
        if (Test-Path $zip) { Remove-Item $zip }
        Compress-Archive -Path (Join-Path $pkg '*') -DestinationPath $zip
        Write-Host "Packaged: $zip"
    }
    default {
        Write-Error "Unsupported target: $Target"
        exit 1
    }
}
