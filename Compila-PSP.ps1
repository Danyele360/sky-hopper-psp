param([ValidateSet('psp', 'wasm', 'all')][string]$Target = 'psp')
$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath $PSScriptRoot
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    python tools/sync_power_rules.py --check
    if ($LASTEXITCODE -ne 0) { throw 'Tabelle bonus non sincronizzate' }
    python tools/sync_design.py --check
    if ($LASTEXITCODE -ne 0) { throw 'Tabelle design non sincronizzate' }
    cargo test -p sky-hopper-game --locked
    if ($LASTEXITCODE -ne 0) { throw 'Test non riusciti' }
    if ($Target -in @('wasm', 'all')) {
    cargo build -p sky-hopper-preview --target wasm32-unknown-unknown --release --locked
    if ($LASTEXITCODE -ne 0) { throw 'Build anteprima non riuscita' }
    Copy-Item -LiteralPath target/wasm32-unknown-unknown/release/sky_hopper_preview.wasm -Destination preview/sky-hopper.wasm
    }
    if ($Target -in @('psp', 'all')) {
    Push-Location crates/psp
    try { cargo psp --release --locked; if ($LASTEXITCODE -ne 0) { throw 'Build PSP non riuscita' } }
    finally { Pop-Location }
    New-Item -ItemType Directory -Force dist/PSP/GAME/SKYHOPPER | Out-Null
    Copy-Item -LiteralPath target/mipsel-sony-psp/release/sky-hopper.EBOOT.PBP -Destination dist/PSP/GAME/SKYHOPPER/EBOOT.PBP
    Copy-Item -LiteralPath target/mipsel-sony-psp/release/sky-hopper.EBOOT.PBP -Destination dist/SkyHopper.EBOOT.PBP
    python tools/package_psp.py
    if ($LASTEXITCODE -ne 0) { throw 'Pacchetto PSP non riuscito' }
    }
} else {
    wsl -d Ubuntu -- bash -lc ('source ~/.cargo/env && bash tools/build.sh ' + $Target)
    if ($LASTEXITCODE -ne 0) { throw 'Build WSL non riuscita' }
}
