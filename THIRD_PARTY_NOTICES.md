# Third-party notices

The [MIT project license](LICENSE) covers Sky Hopper's original code,
documentation and project materials. It does not replace upstream licenses.
Original notices are preserved in `LICENSES/`. Exact dependency versions are
fixed by `Cargo.lock`; the machine-readable inventory is
[LICENSES/dependencies.json](LICENSES/dependencies.json).

## Rust code linked or used during compilation

| Dependency | Version | Upstream license expression |
| --- | --- | --- |
| rust-psp | 1.0.0, revision ad92131218406308170a3344786865415aef2995 | MIT; includes PSPSDK reference notice |
| libm | 0.2.16 | MIT |
| bitflags | 2.13.2 | MIT OR Apache-2.0 |
| linked_list_allocator | 0.10.6 | Apache-2.0/MIT |
| num_enum / num_enum_derive | 0.7.6 | BSD-3-Clause OR MIT OR Apache-2.0 |
| paste | 1.0.15 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| rustversion | 1.0.23 | MIT OR Apache-2.0 |
| spin | 0.9.9 | MIT |
| syn | 2.0.119 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.26 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| unstringify | 0.1.4 | Zlib OR MIT OR Apache-2.0 |

`rust-psp` credits Marko Mijalkovic, Andrew Showers and its other contributors.
Its full [license and PSPSDK notice](LICENSES/rust-psp.txt) are included.
Dependency subfolders preserve all supplied license alternatives and Unicode
notices. Some entries are procedural macros used on the build host rather
than runtime libraries. Rust's core libraries and toolchain are external
Rust distribution components under their upstream licenses.

The source archive does not vendor dependency source. Cargo fetches it from
the locked registry and SDK revision. `tools/collect_licenses.py` refreshes
original license texts from fetched packages when the lock changes.

## Optional development tools

| Tool | Role | License / distribution |
| --- | --- | --- |
| [PPSSPP](https://github.com/hrydgard/ppsspp) | Emulator and PSP debugger | GPL-2.0-or-later; downloaded separately |
| [Playwright](https://github.com/microsoft/playwright) | Browser checks and showcase layouts | Apache-2.0; installed through npm |
| [Pillow](https://github.com/python-pillow/Pillow) | Asset packing and image-file validation | MIT-CMU / HPND notices in its distribution; installed through pip |
| Chromium / Chrome | Browser used by Playwright | Browser and component licenses in the browser distribution |

No emulator, browser binary, Node modules, Python packages or Windows font
files are included in the public source archive. Their use as external tools
does not make them part of Sky Hopper's MIT code. The font-export script uses
a local font or Pillow's fallback; rendered XMB text is supplied as an image,
with no external font file embedded.

An initial visual concept image was supplied as a generation reference.
Generated project graphics and prompts are documented in
[docs/ASSETS.md](docs/ASSETS.md). Screenshots depict those graphics in the game;
audio is synthesized by original project code.

Upstream sources: [MIT text](https://opensource.org/license/mit) and
[pinned rust-psp notice](https://raw.githubusercontent.com/AndrewAltimit/rust-psp/ad92131218406308170a3344786865415aef2995/LICENSE).
