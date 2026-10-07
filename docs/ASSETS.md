# Graphics and animation pipeline

The repository includes both editable source art and runtime build inputs.
A normal build uses the supplied runtime data; no image generation service
or private generation account is required to compile or play.

## Provenance and license

The initially supplied `Sky Hopper_ Oltre le Nuvole.png` was the visual concept
reference; that optional reference is not required to compile the game.
Three new images were generated with the integrated `image_gen` tool, using
that reference for style: `sky.png`, `robot-sheet.png` and `world-sheet.png` in
`assets/source`. The generation prompts are in [art-prompts.md](art-prompts.md).
The art was packed and resized into game atlases; skins and Nebula use runtime
color changes. Original project materials are distributed under [MIT](../LICENSE).
Dependency licenses remain separate. The project does not claim that generated
images were hand-drawn or that screenshots came from a physical PSP.

The bitmap font is authored in `tools/prepare_font.py`. Music and sound effects
are synthesized in project code. No font file, music recording or emulator is
embedded as an asset dependency.

## Files and formats

| Source / output | Format and role |
| --- | --- |
| `assets/source/sky.png` | Source sky painting; reduced to native 480 × 272 |
| `assets/source/robot-sheet.png` | 4 × 4 robot pose sheet with transparent alpha |
| `assets/source/world-sheet.png` | 4 × 4 environment/pickup sheet with alpha |
| `assets/runtime/sky.png`, `sky.rgba` | Native background PNG and embedded raw texture |
| `assets/runtime/robot.png`, `robot.rgba` | 192 × 192 atlas, 16 cells of 48 × 48 |
| `assets/runtime/world.png`, `world.rgba` | 256 × 256 atlas, 16 cells of 64 × 64 |
| `assets/runtime/font.bin` | Authored bitmap font used by software drawing |
| `assets/runtime/manifest.json` | Frame names, cell/grid sizes and preview sequences |
| `assets/runtime/icon0.png`, `pic1.png` | PSP XMB icon/background used by `Psp.toml` |
| `assets/animations/` | Sprite animation GIFs and one historical gameplay GIF |

Raw texture bytes are **RGBA8888, row-major, straight alpha** with no file
header. For example, the background has exactly `480 * 272 * 4` bytes.
Runtime dimensions and sprite indices must match the renderer; changing atlas
layout requires updating the drawing code and manifest together.

## Robot poses

| Indices | Poses |
| --- | --- |
| 0–3 | Four running phases |
| 4–7 | Takeoff, jump, double jump, fall |
| 8–11 | Idle, blink, land, hurt |
| 12–15 | Jetpack, boost, victory, defeat |

The game's renderer chooses and times these frames from movement and state.
GIFs are documentation previews, not files played by the game. Particles,
trails, shield/magnet auras, drones and coin rotations add runtime motion.
The last generated world cell contains a scenery robot rather than the
requested spaceship; it is documented as `scenery-robot` in the manifest.

## Regenerate derived assets

With optional Pillow installed:

```sh
python3 -m pip install -r requirements-dev.txt
python3 tools/prepare_assets.py
```

Packing crops transparent margins, preserves a consistent source scale for
robot poses, creates atlases, uses nearest-neighbor resizing, writes RGBA/PNG,
regenerates the font and sprite GIFs. It does not request new AI images.
To regenerate only the font:

```sh
python3 tools/prepare_font.py
```

For host-rendered reference scenes and XMB previews:

```sh
cargo run -p sky-hopper-game --example capture --locked
python3 tools/export_previews.py
```

The first writes raw frames in `docs/captures`; the second converts them,
creates the gameplay GIF and regenerates XMB images. It uses an available
local title font with a portable fallback. Review images before replacing
release art; these scenes include deliberate test power-ups and are not a
recording from a console.

For new native showcase images, use the isolated profile/debugger procedure
in [showcase/README.md](showcase/README.md). `compose_showcase.cjs` places the
unchanged native captures inside cover/gallery layouts. Keep original native
frames alongside compositions and record the binary hash and demo-profile
status. Current showcase images are from PSP 0.2; older GIFs/captures remain
historical, and the included WASM is older until rebuilt.
