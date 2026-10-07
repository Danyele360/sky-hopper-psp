#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
target="${1:-psp}"
case "$target" in psp|wasm|all) ;; *) echo 'Usage: bash tools/build.sh [psp|wasm|all]' >&2; exit 2;; esac
python3 tools/sync_power_rules.py --check
python3 tools/sync_design.py --check
cargo test -p sky-hopper-game --locked
if [[ "$target" == wasm || "$target" == all ]]; then
  cargo build -p sky-hopper-preview --target wasm32-unknown-unknown --release --locked
  cp target/wasm32-unknown-unknown/release/sky_hopper_preview.wasm preview/sky-hopper.wasm
  echo 'Ready: preview/sky-hopper.wasm'
fi
if [[ "$target" == psp || "$target" == all ]]; then
  (cd crates/psp && cargo psp --release --locked)
  mkdir -p dist/PSP/GAME/SKYHOPPER
  cp target/mipsel-sony-psp/release/sky-hopper.EBOOT.PBP dist/PSP/GAME/SKYHOPPER/EBOOT.PBP
  cp target/mipsel-sony-psp/release/sky-hopper.EBOOT.PBP dist/SkyHopper.EBOOT.PBP
  python3 tools/package_psp.py
  echo 'Ready: dist/PSP/GAME/SKYHOPPER/EBOOT.PBP'
fi
