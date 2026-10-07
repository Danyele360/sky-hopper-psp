"""Pack the generated artwork, without repainting it, for PSP and the WASM preview."""
from pathlib import Path
import json
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'assets/runtime'
OUT.mkdir(parents=True, exist_ok=True)
NEAREST = Image.Resampling.NEAREST

def save(image, name):
    image = image.convert('RGBA')
    image.save(OUT / f'{name}.png')
    (OUT / f'{name}.rgba').write_bytes(image.tobytes())

sky = Image.open(ROOT / 'assets/source/sky.png').convert('RGBA')
save(sky.resize((480, 272), NEAREST), 'sky')

def pack(name, size, player=False):
    src = Image.open(ROOT / f'assets/source/{name}-sheet.png').convert('RGBA')
    atlas = Image.new('RGBA', (size * 4, size * 4))
    frames = []
    for i in range(16):
        x, y = i % 4, i // 4
        tile = src.crop((x * src.width // 4, y * src.height // 4,
                         (x+1) * src.width // 4, (y+1) * src.height // 4))
        bbox = tile.getchannel('A').point(lambda a: 255 if a > 50 else 0).getbbox()
        if not bbox:
            raise ValueError(f'Missing frame {name}:{i}')
        tile = tile.crop(bbox)
        if player:
            # Same source-to-runtime scale throughout the sheet: no per-pose stretching.
            scale = 0.16
            w, h = max(1, round(tile.width * scale)), max(1, round(tile.height * scale))
            if max(w, h) > size - 4:
                scale *= (size - 4) / max(w, h)
                w, h = round(tile.width * scale), round(tile.height * scale)
            tile = tile.resize((w, h), NEAREST)
            px, py = (size-w)//2, size-2-h
        else:
            scale = (size-4) / max(tile.size)
            w, h = max(1, round(tile.width*scale)), max(1, round(tile.height*scale))
            tile = tile.resize((w,h), NEAREST)
            px, py = (size-w)//2, 0 if i < 3 else (size-h)//2
        frame = Image.new('RGBA', (size,size))
        frame.paste(tile, (px,py))
        frames.append(frame)
        atlas.paste(frame, (x*size,y*size))
    save(atlas, name)
    return frames

robot = pack('robot', 48, True)
world = pack('world', 64)
sequences = {
    'run': ([0,1,2,3], 90), 'idle': ([8,8,8,9], 220),
    'jump': ([4,5,5,7,10], 100), 'double-jump': ([5,6,6,7,10], 100),
    'fall': ([7,7,7,10], 120), 'land': ([10,4,8], 90),
    'hurt': ([11,8,11,8], 110), 'jetpack': ([12,12,5,12], 90),
    'boost': ([13,13,1,13], 70), 'victory': ([14,8,14,14], 200),
    'defeat': ([11,15,15,15], 240),
}
ANIM = ROOT / 'assets/animations'
ANIM.mkdir(exist_ok=True)
for name, (indices, duration) in sequences.items():
    frames=[]
    for i in indices:
        frame=Image.new('RGBA',(144,144),(7,20,42,255))
        frame.alpha_composite(robot[i].resize((144,144),NEAREST))
        frames.append(frame.convert('RGB'))
    frames[0].save(ANIM/f'{name}.gif',save_all=True,append_images=frames[1:],duration=duration,loop=0)
for name, indices in {'coin':[4,5,6,5], 'drone':[14], 'spring':[3],
                      'magnet':[8], 'shield':[9], 'boost-pickup':[10],
                      'super-jump':[11], 'jetpack-pickup':[12], 'slow-motion':[13]}.items():
    frames=[]
    for step in range(8):
        frame=Image.new('RGBA',(128,128),(7,20,42,255))
        im=world[indices[step%len(indices)]].resize((96,96),NEAREST)
        frame.alpha_composite(im,(16,12+(step%4)*2))
        frames.append(frame.convert('RGB'))
    frames[0].save(ANIM/f'{name}.gif',save_all=True,append_images=frames[1:],duration=100,loop=0)

manifest = {'resolution':[480,272], 'format':'RGBA8888, row-major, straight alpha',
 'robot':{'image':'robot.png','cell':[48,48],'grid':[4,4],
 'frames':['run-0','run-1','run-2','run-3','takeoff','jump','double-jump','fall','idle','blink','land','hurt','jetpack','boost','victory','defeat']},
 'world':{'image':'world.png','cell':[64,64],'grid':[4,4],
 'frames':['island','tech-platform','ramp','spring','coin-front','coin-quarter','coin-edge','spikes','magnet','shield','boost','super-jump','jetpack','slow-motion','drone','scenery-robot']},
 'sequences':{k:{'frames':v[0],'duration_ms':v[1]} for k,v in sequences.items()}}
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')

# The authored font can also be regenerated without repacking artwork.
from prepare_font import build_font
build_font(OUT/'font.bin')
print('Prepared 3 runtime textures, font, manifest and', len(list(ANIM.glob('*.gif'))), 'animation GIFs')
