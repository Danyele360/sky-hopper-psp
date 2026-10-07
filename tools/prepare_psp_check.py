"""Prepare an isolated legacy-save migration check, without a private emulator folder."""
from pathlib import Path
import shutil
import struct

root = Path(__file__).resolve().parents[1]
session = root / '.tools/psp-v02-check'
pbp = root / 'dist/SkyHopper.EBOOT.PBP'
if not pbp.is_file():
    raise SystemExit('Build PSP first, or copy the standalone release to dist/SkyHopper.EBOOT.PBP')
session.mkdir(parents=True, exist_ok=True)
shutil.copyfile(pbp, session / 'SkyHopper.EBOOT.PBP')
values = {'best': 62, 'bank': 0, 'owned': 0, 'skin': 0, 'world': 0, 'upgrade': 0}
data = bytearray(b'RCFG' + struct.pack('<HH', 1, len(values)))
for key, value in values.items():
    encoded = key.encode('utf-8')
    data += bytes([len(encoded)]) + encoded + bytes([2]) + struct.pack('<HI', 4, value)
for filename in ('legacy-save.bin', 'sky-hopper-save.bin'):
    (session / filename).write_bytes(data)
(session / 'ppsspp.ini').write_text('''[General]
FirstRun=False
RemoteDebuggerOnStartup=True
RemoteDebuggerLocal=True
RemoteISOPort=9234
PauseWhenMinimized=False
[Graphics]
GraphicsBackend=0
InternalResolution=2
ShowFPSCounter=0
[Sound]
Enable=False
''', encoding='utf-8')
print('Prepared .tools/psp-v02-check; start PPSSPP with its EBOOT and ppsspp.ini')
