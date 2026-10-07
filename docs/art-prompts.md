# Prompt delle immagini

Generatore utilizzato: tool integrato `image_gen`, senza CLI/API esterne.
Tutte e tre le generazioni usano `Sky Hopper_ Oltre le Nuvole.png` esclusivamente
come riferimento di stile. Gli originali sono in `assets/source/`; il packing
ridimensiona e ritaglia le celle senza ridisegnare l'artwork. L'alpha trasparente
degli sprite viene conservato. La risoluzione effettiva è quella restituita dal
tool, gestita automaticamente dal packing anche se diversa da quella richiesta.

## Fondale — sky.png

Use case: stylized-concept. Asset type: production background for Sky Hopper PSP,
a 480x272 side-scrolling pixel art platformer. Input image is a visual STYLE
reference only, not an edit target. Create a new wide 16:9-ish landscape background,
at 1536x864 if possible. Match the reference's beautiful saturated blue sky/space,
giant softly textured blue planet upper left, tiny stars, distant floating islands
with waterfalls and pale futuristic towers, banks of glowing white clouds in lower
half. Clean modern pixel art, hand-crafted crisp pixels and layered atmospheric
depth, richly detailed yet readable when reduced to 480x272. The main gameplay runs
horizontally toward the right and upward; leave the central horizontal band mostly
open sky for foreground platforms and characters. Background only: NO character,
NO coins, NO foreground playable platforms, NO HUD, NO text, NO PSP console,
NO border, NO collage. Full bleed standalone game background.

## Robot — robot-sheet.png

Use case: stylized-concept. Asset type: production transparent sprite sheet for Sky
Hopper PSP. Reference image is STYLE and character inspiration, create an original
consistent cute pearl-white spherical little sky robot with a large dark navy glass
face, two luminous cyan eyes, small cyan side-ear pods, single antenna, tiny
articulated legs and hands, faces RIGHT in all frames. Modern clean blue/cyan pixel
art with dark outlines, readable at 32px. Make EXACTLY a 1024x1024 transparent sprite
sheet organized as a strict 4-column by 4-row equal grid, each cell 256x256. Center
one complete character frame inside each cell with generous empty margin, no
overlap. Character identity and scale identical in all 16 frames, baseline
identical. Row 1: four phases of RUNNING cycle right (legs alternate, arm swing).
Row 2: takeoff crouch, jumping upward, double-jump curled with small cyan spark,
falling with feet down. Row 3: idle neutral, idle blink, landing squashed, hurt
startled. Row 4: jetpack flying with small orange exhaust, boost forward lean with
cyan exhaust, happy victory arms raised, defeated robot sitting with antenna
drooping. EXACTLY 16 separate sprites. NO captions, NO lettering, NO gridlines,
NO checkerboard painted background. Real transparent alpha background. Do not
include scenes or UI or platforms. Production sprite atlas.

## Mondo — world-sheet.png

Use case: stylized-concept. Asset type: production environment and collectible
sprite atlas for Sky Hopper PSP. Reference image style only. Clean vibrant pixel
art navy/cyan outline floating sky island technology, readable at 32 to 96 pixels.
TRUE transparent background. Exactly 1024x1024 image arranged strict 4 columns by
4 rows of equal 256x256 cells. Exactly one separate asset per cell, centered fully
inside cell with margins. NO text, labels, gridlines, scene, checkerboard. Row 1:
wide flat grass-topped floating rocky island platform; wide flat steel/cyan
floating tech platform; wide grass-topped sloping ramp rising toward right;
compressed red metal spring on small blue base. Row 2: golden coin front view;
golden coin rotated three-quarter; golden coin thin edge view; large triangular
dark metal spikes cluster. Row 3: cyan/red horseshoe magnet collectible; glowing
cyan shield collectible; double cyan forward-chevron boost collectible; purple
upward arrow super-jump collectible. Row 4: small white/cyan jetpack collectible;
cyan hourglass slow-motion collectible; navy spherical enemy drone with red glowing
eye and small rotor; cheerful small hovering cyan spaceship scenery. Consistent
light upper left. Island and metal platform must have FLAT tops, easy collision
surface; show full rocky underside. Production transparent sprite sheet, no UI.

Il tool ha restituito un robot di scena nella sedicesima cella del mondo;
quella cella è indicata come `scenery-robot` nel manifest e non usata come nave.
