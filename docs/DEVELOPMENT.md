# Development / Sviluppo

Run commands from the project root. Examples use Linux shell; on Windows use
Ubuntu under WSL2, or the PowerShell wrapper where indicated. The tested
environment is Ubuntu + Rust **nightly-2026-04-13**. macOS and native Windows
are alternatives, but were not verified for the PSP build.

## 1. Play without compiling

Python 3.11+ is enough for the included browser preview:

```sh
python3 tools/preview_server.py
```

The playable `preview/sky-hopper.wasm` is the older build. Its JavaScript wrapper
has current language support; the binary still has the older game rules.
`http://127.0.0.1:8787/preview/camera.html` is the current UI/design review using
JavaScript models. It is useful for layout and economics, not Rust physics.

The server binds only to localhost. `--no-browser` avoids opening a browser;
`--port 8788` selects another port.

## 2. Install Rust and the matching PSP packager

Install Git, Python, Rustup and host build prerequisites. On Ubuntu the usual
host prerequisites are `build-essential`, `pkg-config` and `libssl-dev`.
Make sure Cargo is on PATH; a standard Rustup installation provides it through
`~/.cargo/env`.

```sh
rustup toolchain install nightly-2026-04-13 --profile minimal --component rust-src --component rustfmt --target wasm32-unknown-unknown
cargo +nightly-2026-04-13 install --git https://github.com/AndrewAltimit/rust-psp --rev ad92131218406308170a3344786865415aef2995 cargo-psp --locked
```

Alternative installation of the same packager:

```sh
git clone https://github.com/AndrewAltimit/rust-psp .tools/rust-psp
git -C .tools/rust-psp checkout ad92131218406308170a3344786865415aef2995
cargo +nightly-2026-04-13 install --path .tools/rust-psp/cargo-psp --locked
```

The clone is optional and ignored by Git. Cargo fetches the game SDK dependency
itself using `crates/psp/Cargo.toml`. Keep `Cargo.lock` in the repository.
The SDK builds its target support through Rust; no private files from the
maintainer's `.tools` directory are required.

The pinned SDK does not compile with the tested newer nightlies 2026-08-26 and
2026-09-30 because `core::panic::PanicPayload` was removed. Keep the pinned
toolchain until the SDK and project are deliberately migrated together.

## 3. Build

```sh
bash tools/build.sh psp
bash tools/build.sh wasm
bash tools/build.sh all
```

Each mode checks the generated rule tables and runs core tests. PSP mode
produces `target/mipsel-sony-psp/release/sky-hopper.EBOOT.PBP`, installation and
standalone copies in `dist`, and `dist/SkyHopper-PSP-v0.2.zip` with notices.
WASM mode replaces `preview/sky-hopper.wasm` with the current shared game.
The default mode is `psp`; it preserves the existing WASM.

Windows equivalents are `Compila-PSP.ps1 -Target psp`, `wasm` or `all`.
The wrapper uses local Cargo if found, otherwise `wsl -d Ubuntu`.
Invoke it with your normal PowerShell execution policy; no system policy
change is required for the Linux commands. npm examples can use `npm.cmd`
and `npx.cmd` in PowerShell when `.ps1` launchers are disabled.

Direct builds, useful when diagnosing failures:

```sh
cargo test -p sky-hopper-game --locked
cargo build -p sky-hopper-preview --target wasm32-unknown-unknown --release --locked
# For PSP, run from crates/psp:
cargo psp --release --locked
```

The direct WASM command does not copy its output into `preview`; the build
script does. Direct PSP compilation does not package `dist`; the build script
copies the outputs and calls `tools/package_psp.py`.

## 4. Test and review

Core Rust and generated tables:

```sh
python3 tools/sync_power_rules.py --check
python3 tools/sync_design.py --check
cargo fmt --all -- --check
cargo test -p sky-hopper-game --locked
```

The core tests cover generation, movement, input edges, pause, compatible and
conflicting bonuses, shield consumption, progression, migration and clipping.
They run on the host and do not require a PSP emulator.

Optional browser/model tools require **Node 22+**:

```sh
npm ci
npx playwright install chromium
npm test
```

On Linux a browser host may also need Playwright's system dependencies:
`npx playwright install --with-deps chromium`. Start the local preview server
in another terminal, then run:

```sh
npm run check:browser
npm run check:preview
```

The first checks the current JavaScript design models and UI; the second checks
the playable WASM file actually present. Both save reports/screenshots. The
browser helper uses Playwright Chromium, or local Chrome on Windows when
present. Set `PLAYWRIGHT_CHROMIUM_EXECUTABLE` to choose another executable.
Set `PREVIEW_ORIGIN` (for example `http://127.0.0.1:8788`) when testing a server
on another port.

## 5. Test the native PSP executable

Install [PPSSPP](https://www.ppsspp.org/download/) separately. Use its standalone
EBOOT copy. The portable Windows launcher `Avvia-PPSSPP.cmd` expects an optional
emulator at `.tools/ppsspp/PPSSPPWindows64.exe`; otherwise open your installed
PPSSPP and choose the EBOOT manually.

Create a reproducible legacy-save fixture:

```sh
python3 tools/prepare_psp_check.py
```

This writes only `.tools/psp-v02-check`, with a six-key demonstration save, the
new EBOOT and debugger config. Do not point the test at a personal game folder.
Launch PPSSPP using the configuration and standalone executable, for example:

```sh
PPSSPP --config=".tools/psp-v02-check/ppsspp.ini" ".tools/psp-v02-check/SkyHopper.EBOOT.PBP"
node tools/check_psp_release.cjs
```

Use your emulator executable name/path. On Windows use its full path and
absolute config/game paths. Enable the local remote debugger if your build
ignores the config; the scripts connect to `ws://127.0.0.1:9234/debugger`.
`PYTHON` can select the interpreter used for frame export. Wait for boot before
running the script. Native images are read from emulated VRAM, not from browser
models; input timing and hardware speed can affect gameplay frames.

## 6. Change rules and translations

Edit the JSON sources, then regenerate:

```sh
python3 tools/sync_power_rules.py
python3 tools/sync_design.py
```

Do not hand-edit `power_rules.rs` or `design_rules.rs`. Commit JSON changes and
generated files together. Update the JavaScript models if behavior changes;
they mirror the design but execute independently from Rust.

## 7. Graphics and packaging

For optional art/export tools:

```sh
python3 -m pip install -r requirements-dev.txt
```

See [ASSETS.md](ASSETS.md) before regenerating graphics. Asset generation is
not part of a normal build. To refresh licenses after dependency changes,
fetch the lock and run `tools/collect_licenses.py` in the same Rust environment.
`--sdk` explicitly selects the pinned SDK checkout.

`python3 tools/package_source.py` creates the public source bundle, and
`python3 tools/package_psp.py` packages an existing EBOOT without compiling it.
`tools/package_showcase.py` validates the historical 0.2 screenshots against
the exact original release hashes; it is not a generic check for later builds.
