# Contributing to Sky Hopper

Read [the development guide](docs/DEVELOPMENT.md) and
[project map](docs/PROJECT_MAP.md) before changing the shared core.

## Propose a change

For a bug, include the target (PSP, PPSSPP, playable WASM or JavaScript review),
the version, steps to reproduce and the expected result. Include a screenshot
or emulator version when relevant. Share a personal save only when necessary
and when you intend to share its progress data.

For a contribution, explain the concrete problem, resulting behavior and
checks you ran. Keep changes focused. Contributions are made under this
repository's MIT license; preserve upstream notices and document the
origin/license of any new asset or dependency.

## Checks

```sh
python3 tools/sync_power_rules.py --check
python3 tools/sync_design.py --check
cargo fmt --all -- --check
cargo test -p sky-hopper-game --locked
npm test
```

For JSON changes, regenerate Rust tables before checking them. For UI changes,
run the browser design review against the local server and inspect the native
480 × 272 layout. Gameplay changes need checks on the Rust core and relevant
target: JavaScript models do not verify Rust physics. Save changes must preserve
migration from the original six-key and current 32-key formats.

New mechanics should have meaningful behavior and interaction tests. Text,
documentation and low-impact visuals can be checked directly. Do not commit
installed tools, credentials, personal saves or build caches. Binary releases
and screenshots should identify the exact build and whether they came from
an emulator, a browser or physical hardware.
