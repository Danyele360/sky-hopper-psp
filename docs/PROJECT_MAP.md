# Project map / Mappa del progetto

This page explains the public project and local working folders. The source
bundle contains everything needed to understand and rebuild the game;
external tools and installed caches are obtained separately.

## Root files

| File | Role |
| --- | --- |
| `README.md`, `README.it.md` | Public overview, play instructions, features and entry points |
| `LEGGIMI.md` | Italian documentation index, retained for existing links |
| `LICENSE` | MIT license for original project materials |
| `THIRD_PARTY_NOTICES.md`, `LICENSES/` | Dependency attribution, original license texts and version inventory |
| `CONTRIBUTING.md` | Contribution and verification expectations |
| `Cargo.toml`, `Cargo.lock` | Rust workspace, build profiles and exact dependency graph |
| `rust-toolchain.toml` | Matching nightly, components and WASM target |
| `package.json`, `package-lock.json` | Optional Node checks and pinned Playwright |
| `requirements-dev.txt` | Optional Pillow version for packing/export |
| `.gitignore`, `.gitattributes` | Public/local separation and text/binary handling |
| `Compila-PSP.ps1` | Windows entry point for PSP, WASM or both |
| `Avvia-Anteprima.cmd` | Local browser preview launcher |
| `Avvia-PPSSPP.cmd` | Optional portable Windows emulator launcher |

## Rust workspace

| File or folder | Role |
| --- | --- |
| `crates/game/src/lib.rs` | Platform generation, physics, collisions, inputs, game modes and tests |
| `crates/game/src/powers.rs` | Independent bonus timers and compatibility behavior |
| `crates/game/src/progression.rs` | Permanent upgrades, purchases, capsules, missions and save migration |
| `crates/game/src/render.rs` | Software drawing, world camera and native-resolution UI |
| `crates/game/src/power_rules.rs` | Generated bonus tables from `assets/power-rules.json` |
| `crates/game/src/design_rules.rs` | Generated economy/catalog and ITA/ENG strings |
| `crates/game/examples/capture.rs` | Deterministic host-rendered reference scenes and raw frame exports |
| `crates/psp/src/main.rs` | PSP startup, fixed-step loop, controller, framebuffer and save I/O |
| `crates/psp/src/audio.rs` | Music and effects synthesized on a separate PSP thread |
| `crates/psp/Psp.toml` | XMB title, version, game ID, icon and background metadata |
| `crates/wasm/src/lib.rs` | Browser ABI exposing the shared game and framebuffer |

## Graphics, browser and documentation

`assets/source` stores three generated source PNGs; `assets/runtime` stores
embedded textures, original bitmap font, XMB pictures and sprite metadata.
`assets/animations` contains GIF previews. JSON sources define bonuses,
progression and localization. [Asset pipeline](ASSETS.md).

`preview/index.html`, `game.js` and `style.css` are the WASM player.
`i18n.mjs` provides translations. `camera.html`, `camera.js` and `review.css`
provide the visual review; `power-state.mjs` and `progression.mjs` are separate
JavaScript rule models. `sky-hopper.wasm` is the included older binary.

| Documentation folder/file | Role |
| --- | --- |
| `docs/DEVELOPMENT.md` | Setup, builds, tests and troubleshooting |
| `docs/ARCHITECTURE.md` | Simulation, rendering, progression and platform boundaries |
| `docs/ASSETS.md`, `art-prompts.md` | Provenance and graphics/animation generation |
| `docs/PUBLISHING.md` | Public source package and GitHub/release instructions |
| `docs/OPEN_SOURCE_CHECK.md` | Verification of the extracted public source distribution |
| `docs/replayability.md` | Progression design decisions and researched sources |
| `docs/verification.md`, `captures/` | Historical first-version checks and captures |
| `docs/camera-review/`, `power-review/`, `design-review/` | Historical browser model/layout reviews |
| `docs/psp-v0.2/` | Native PSP 0.2 test evidence |
| `docs/showcase/` | Updated real PSP images and GitHub layouts |
| `docs/releases/` | Public installation instructions and original verified build hashes |

## Every tool

| Tool | Purpose / prerequisites |
| --- | --- |
| `build.sh` | Test and build `psp`, `wasm` or `all`; Rust + Python, PSP packager for PSP |
| `sync_power_rules.py` | Generate bonus Rust tables; standard Python, `--check` supported |
| `sync_design.py` | Generate catalog/economy/localization Rust tables; standard Python, `--check` supported |
| `preview_server.py` | Serve the project locally; standard Python |
| `prepare_font.py` | Regenerate the project's authored bitmap font |
| `prepare_assets.py` | Pack source art, font, manifest and sprite GIFs; Pillow |
| `export_previews.py` | Convert host frame exports into PNG/GIF and XMB images; Pillow |
| `check_powers.mjs` | Test JavaScript bonus behavior; Node |
| `check_progression.mjs` | Test JavaScript economy, migration, missions and capsules; Node |
| `browser.cjs` | Shared portable Playwright browser selection |
| `check_preview.cjs` | Check the included playable WASM in a browser; Playwright + local server |
| `check_design_review.cjs` | Current model/UI checks, including language and mobile layout; Playwright + server |
| `check_power_review.cjs` | Compatibility entry point delegating to the current design review |
| `prepare_psp_check.py` | Create an isolated legacy save/config/EBOOT fixture for native tests |
| `check_psp_release.cjs` | Native 0.2 input, UI, migration and pause check; PPSSPP debugger + Node + Pillow |
| `check_psp.cjs` | Basic native boot/gameplay/pause/hangar capture; debugger + Node + Pillow |
| `capture_showcase.cjs` | Isolated demonstration profile and native screenshot bursts; PSP release + debugger |
| `compose_showcase.cjs` | Compose cover, gallery and sharing image around native captures; Playwright |
| `package_showcase.py` | Validate original 0.2 capture hashes and archive the showcase; Pillow |
| `package_psp.py` | Validate and package an existing EBOOT with instructions and license notices |
| `collect_licenses.py` | Copy original license texts from fetched locked Rust dependencies; Python 3.11+ |
| `package_source.py` | Check public files/links and generate the complete open-source ZIP and manifest |

## Local folders outside the public package

| Location | Why it stays local |
| --- | --- |
| `.tools/` | Downloaded SDK/emulator, debugger fixtures, scratch files and demonstration saves |
| `target/` | Recreated Rust compilation output/cache |
| `node_modules/` | Recreated installed npm dependencies |
| `.venv/`, `venv/`, `__pycache__/` | Recreated Python environments/cache |
| `dist/` | Generated binary/source/showcase release archives |
| `docs/**/*.rgba` | Derived screenshot buffers; recreate through capture tools |
| `sky-hopper-save.bin`, `*.ppst` | Personal progress and emulator save states |
| `.env*`, logs and OS metadata | Local settings, credentials or irrelevant machine files |
| `src/` | Empty legacy folder in the initial workspace; the code lives in `crates/` |

The **RGBA files in `assets/runtime` are included**: they are embedded build
inputs, unlike derived raw screenshot buffers. If the original supplied
concept image is present at the root it is also packaged; it is optional and
not required for compilation. The generated source art is always included.
