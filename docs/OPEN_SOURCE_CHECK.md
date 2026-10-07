# Open-source distribution check — 8 October 2026

The project now has a root MIT license, Cargo license metadata, English and
Italian READMEs, contribution guidance, development/architecture/asset guides,
a complete tool map and publishing instructions. Original license texts for
the 14 locked external Rust dependencies are preserved in `LICENSES`.

Verification performed:

- 30 Rust core tests passed, including from an extracted public source ZIP.
- 22 JavaScript model tests passed; generated JSON/Rust tables matched.
- Rust formatting, shell build-script syntax, PowerShell syntax, Python script
  compilation and Node script syntax checks passed.
- `npm ci` installed the pinned Playwright dependencies in the extracted copy.
- The extracted project was served on a separate local port; the browser
  design review passed, including ITA/ENG, progression, mobile layout and
  loading the included older WASM.
- The isolated PSP test fixture could be generated from the extracted project.
- Packaging the existing PSP binary from the extracted project succeeded,
  including project/dependency license notices and no saves.
- Public Markdown links, required embedded assets, dependency license files,
  source ZIP integrity and per-file SHA-256 values were checked.

The PSP EBOOT and existing older WASM were not rebuilt or changed. Their hashes
remain the verified values in the native/showcase records. The installation
ZIP was repackaged to include the new project license and all dependency
notices; its hash can therefore differ from the historical ZIP record.

The source package excludes `.tools`, `target`, `node_modules`, Python caches,
local settings, personal saves and derived raw screenshot buffers. Embedded
runtime RGBA assets, original generated art, all public code/tools/docs,
animation previews and screenshots are included. Original concept-reference
files are optional; the generated source art and runtime data are included.

Public repository: [Danyele360/sky-hopper-psp](https://github.com/Danyele360/sky-hopper-psp).
Original project copyright and Git authorship identify Daniele (Danyele360).
