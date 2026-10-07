# Publishing the complete open-source project

Official repository: [Danyele360/sky-hopper-psp](https://github.com/Danyele360/sky-hopper-psp).

The public distribution is a complete project: Rust source, original and
runtime art, animation previews, browser player and review models, tools,
documentation, tests, locks, dependency notices and current screenshots.
The local machine's installed tools and progress data are excluded.

## Generate the source archive

```sh
python3 tools/package_source.py
```

Outputs are `dist/SkyHopper-Source-v0.2.zip` and
`dist/source-v0.2-manifest.json`. The ZIP contains a `SkyHopper/` project root,
including a `SOURCE-MANIFEST.json` with file sizes and SHA-256 values.
The packaging check validates documentation links, locked license records,
essential build inputs and exclusion of local folders, credentials and saves.
It does not compile or publish the game.

| Included | Downloaded or generated separately |
| --- | --- |
| `crates`, `assets`, `preview`, `tools`, `docs`, `LICENSES` | `.tools`, `target`, `node_modules`, Python environments |
| Root documentation, licenses, manifests, locks and launchers | Personal saves, emulator save states, local settings and logs |
| `assets/runtime/*.rgba` used by compilation | `docs/**/*.rgba` derived screenshot buffers |
| Included older WASM and generated source graphics | PSP EBOOT and ZIP outputs in `dist` |

The empty legacy `src` folder is not used; all source lives in `crates`.
Dependency source is fetched from pinned Cargo/npm/pip metadata rather than
copied from a private cache. The Node tools and Pillow are optional; runtime
assets are ready to compile as supplied.

## Create the GitHub repository

Unzip the source bundle into a separate folder, enter `SkyHopper`, and create
a public repository from its contents. `README.md` is the English landing
page; `README.it.md` is the Italian guide. The examples below can also be used
to publish a fork in your own account.

With Git and GitHub CLI authenticated to your account, for example:

```sh
git init -b main
git add .
git status
git commit -m "Publish Sky Hopper source, assets and documentation"
gh repo create sky-hopper-psp --public --source=. --remote=origin --push
```

Choose your desired repository name. These commands publish to your account;
the local preparation scripts do not run them. When using the existing work
folder instead of the source bundle, review `git status` and keep `.gitignore`
in place. Do not force-add ignored directories.

The cover and gallery are already linked by the README. Set
`docs/showcase/social-preview.png` as the repository sharing image if desired.
`docs/showcase/GITHUB.md` remains a shorter Markdown block for other pages.

## Binary releases

Compile PSP using the documented toolchain. `tools/build.sh psp` packages the
result with installation instructions, the project license and dependency
notices. Attach `SkyHopper-PSP-v0.2.zip`, the standalone EBOOT and the source
ZIP to a GitHub Release when ready. Describe the tested target and emulator,
and distinguish emulator evidence from physical hardware testing.

Installation ZIPs must not contain demonstration or personal save files.
Keep users' existing `sky-hopper-save.bin` when updating the executable.
The reference hashes of the originally verified 0.2 binary are retained in
`docs/releases/verified-build-v0.2.json`; packaging dates and ZIP hashes may
change when notices or instructions are updated.

## Maintain the public source

- Keep JSON sources and generated Rust tables synchronized.
- Keep lockfiles and license inventory synchronized when dependencies change.
- Document the origin and license of added assets; do not copy proprietary files
  from an SDK cache or operating system.
- Update release metadata, docs and screenshots when gameplay changes.
- Rebuild WASM before calling it a preview of the current Rust gameplay; the
  initially included WASM is an older build.

Contributors can use, modify and redistribute original project materials
under MIT while retaining the license notice. The exact license text is in
[LICENSE](../LICENSE); upstream components retain the notices in
[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).
