"""Verify and package the existing PSP release. Does not build PSP or WASM."""
from pathlib import Path
from datetime import date
import hashlib
import json
import struct
import shutil
import zipfile

root = Path(__file__).resolve().parents[1]
dist = root / 'dist'
pbp = dist / 'PSP/GAME/SKYHOPPER/EBOOT.PBP'
data = pbp.read_bytes()
assert data[:4] == b'\x00PBP', 'Invalid EBOOT magic'
offsets = struct.unpack_from('<8I', data, 8)
assert offsets[0] >= 40 and list(offsets) == sorted(offsets) and offsets[-1] <= len(data)
assert data[offsets[0]:offsets[0]+4] == b'\x00PSF', 'Missing XMB metadata'
assert data[offsets[1]:offsets[1]+8] == b'\x89PNG\r\n\x1a\n', 'Missing XMB icon'
assert data[offsets[4]:offsets[4]+8] == b'\x89PNG\r\n\x1a\n', 'Missing XMB background'
assert data[offsets[6]:offsets[6]+4] in (b'\x7fELF', b'~PSP'), 'Missing PSP executable'
assert (dist/'SkyHopper.EBOOT.PBP').read_bytes() == data
archive = dist/'SkyHopper-PSP-v0.2.zip'
(dist/'licenses').mkdir(parents=True, exist_ok=True)
shutil.copyfile(root/'docs/releases/ISTRUZIONI-v0.2.txt', dist/'ISTRUZIONI.txt')
shutil.copyfile(root/'LICENSE', dist/'LICENSE')
shutil.copyfile(root/'THIRD_PARTY_NOTICES.md', dist/'THIRD_PARTY_NOTICES.md')
for source in (root/'LICENSES').rglob('*'):
    if source.is_file():
        destination = dist/'licenses'/source.relative_to(root/'LICENSES')
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
files = [pbp,dist/'ISTRUZIONI.txt',dist/'LICENSE',dist/'THIRD_PARTY_NOTICES.md']
license_files = sorted(p for p in (root/'LICENSES').rglob('*') if p.is_file())
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
    for path in files:z.write(path,path.relative_to(dist).as_posix())
    for path in license_files:z.write(path,'LICENSES/'+path.relative_to(root/'LICENSES').as_posix())
with zipfile.ZipFile(archive) as z:
    assert z.testzip() is None
    assert z.read('PSP/GAME/SKYHOPPER/EBOOT.PBP') == data
    assert all('save.bin' not in name for name in z.namelist())
manifest = {
    'release':'0.2','packaged_on':date.today().isoformat(),'target':'PSP',
    'pbp_bytes':len(data),'pbp_sha256':hashlib.sha256(data).hexdigest(),
    'zip_bytes':archive.stat().st_size,'zip_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),
    'wasm_sha256':hashlib.sha256((root/'preview/sky-hopper.wasm').read_bytes()).hexdigest(),
    'verification':'PBP structure, matching executable copy, ZIP integrity and absence of saves; run tests separately',
}
(dist/'release-v0.2.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8')
print(json.dumps(manifest,indent=2))
