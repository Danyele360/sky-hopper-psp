"""Validate existing captures and package the GitHub showcase; no image edits."""
from pathlib import Path
from PIL import Image
import hashlib
import json
import re
import zipfile

root = Path(__file__).resolve().parents[1]
showcase = root / 'docs/showcase'
sha256 = lambda data: hashlib.sha256(data).hexdigest()
build_hash = sha256((root / 'dist/SkyHopper.EBOOT.PBP').read_bytes())
assert build_hash == '20578c2ad1bb01ac88ffbf5ca6e5e4e6c0a23f9f0d7af675f316b7b387ab42f8'
assert sha256((root / 'preview/sky-hopper.wasm').read_bytes()) == 'da4781e10a53ae5ab66cb4daecbed46e6c0ac3d5b61aedb1f26583e95356211f'

compositions = {'cover': (1600, 1000), 'social-preview': (1280, 640), 'gallery': None}
images = []
for file in sorted(showcase.glob('*.png')):
    with Image.open(file) as image:
        size = image.size
        image.verify()
    composed = file.stem in compositions
    if not composed:
        assert size == (480, 272), (file.name, size)
    elif compositions[file.stem]:
        assert size == compositions[file.stem], (file.name, size)
    else:
        assert size[0] == 2080
    images.append({'file': file.name, 'width': size[0], 'height': size[1],
                   'kind': 'presentation layout' if composed else 'native PSP framebuffer',
                   'bytes': file.stat().st_size, 'sha256': sha256(file.read_bytes())})
assert len(images) == 21 and sum(i['kind'] == 'native PSP framebuffer' for i in images) == 18

manifest = {'capturedOn': '2026-10-08', 'release': 'PSP 0.2', 'gameId': 'SKYH-00001',
            'emulator': 'PPSSPP 1.20.4', 'buildSha256': build_hash,
            'source': 'Actual emulated PSP framebuffer; the game was not rebuilt',
            'demoProfile': 'Isolated save with staged coins, levels, stock and unlocks. Gameplay and run rewards captured live.',
            'nativeImages': 18, 'presentationLayouts': 3,
            'checks': ['All PNGs decode', 'Native captures are 480x272',
                       'Composition dimensions match', 'README links resolve',
                       'PSP EBOOT and previous WASM hashes unchanged'],
            'images': images}
(showcase / 'capture-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')

# Check local Markdown destinations after creating the linked manifest.
text = (showcase / 'README.md').read_text(encoding='utf-8')
for destination in re.findall(r'\]\(([^)]+)\)', text):
    assert (showcase / destination).is_file(), destination

archive = root / 'dist/SkyHopper-GitHub-screens-v0.2.zip'
files = sorted(p for p in showcase.iterdir() if p.suffix in {'.png', '.md', '.json'})
with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED, compresslevel=9) as package:
    for file in files:
        package.write(file, file.relative_to(root).as_posix())
with zipfile.ZipFile(archive) as package:
    assert package.testzip() is None
    assert len(package.namelist()) == len(files)
    assert all(name.startswith('docs/showcase/') for name in package.namelist())
print(json.dumps({'nativeScreenshots': 18, 'compositions': 3,
                  'gallerySize': next([i['width'], i['height']] for i in images if i['file'] == 'gallery.png'),
                  'archive': str(archive), 'files': len(files), 'bytes': archive.stat().st_size,
                  'sha256': sha256(archive.read_bytes()), 'checks': 'passed'}, indent=2))
