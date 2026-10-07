"""Validate and archive the public project without installed tools or personal saves."""
from datetime import date
import hashlib
import json
import os
from pathlib import Path
import re
import stat
from urllib.parse import unquote
import zipfile

root = Path(__file__).resolve().parents[1]
public_dirs = {'crates', 'assets', 'preview', 'tools', 'docs', 'LICENSES'}
root_files = {
    '.gitignore', '.gitattributes', 'README.md', 'README.it.md', 'LEGGIMI.md',
    'CONTRIBUTING.md', 'LICENSE', 'THIRD_PARTY_NOTICES.md',
    'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml',
    'package.json', 'package-lock.json', 'requirements-dev.txt',
    'Avvia-Anteprima.cmd', 'Avvia-PPSSPP.cmd', 'Compila-PSP.ps1',
}
optional_root_files = {'Sky Hopper_ Oltre le Nuvole.png'}
private_dirs = {'.git', '.tools', 'target', 'node_modules', '.venv', 'venv', '__pycache__'}
def excluded(path):
    rel = path.relative_to(root)
    return (any(part in private_dirs for part in rel.parts)
            or path.name == 'sky-hopper-save.bin' or path.name.startswith('.env')
            or path.name in {'.DS_Store', 'Thumbs.db'}
            or path.suffix.lower() in {'.pyc', '.pyo', '.log', '.ppst'}
            or (rel.parts[0] == 'docs' and path.suffix == '.rgba'))

files = [root / name for name in root_files if (root / name).is_file()]
assert len(files) == len(root_files), f'Missing public root files: {sorted(name for name in root_files if not (root / name).is_file())}'
files += [root / name for name in optional_root_files if (root / name).is_file()]
for folder in sorted(public_dirs):
    assert (root / folder).is_dir(), folder
    for directory, dirs, names in os.walk(root / folder):
        dirs[:] = sorted(d for d in dirs if d not in private_dirs)
        for name in sorted(names):
            file = Path(directory) / name
            if file.is_symlink():
                raise AssertionError(f'Symbolic links need explicit review: {file.relative_to(root)}')
            if not excluded(file):
                files.append(file)
files = sorted(files, key=lambda p: p.relative_to(root).as_posix())
paths = {file.relative_to(root).as_posix() for file in files}
required = {'crates/psp/Psp.toml', 'crates/game/src/lib.rs', 'crates/wasm/src/lib.rs',
            'assets/runtime/sky.rgba', 'assets/runtime/robot.rgba', 'assets/runtime/world.rgba',
            'assets/runtime/font.bin', 'LICENSES/rust-psp.txt', 'LICENSES/dependencies.json',
            'preview/sky-hopper.wasm', 'docs/showcase/cover.png',
            'docs/DEVELOPMENT.md', 'docs/PROJECT_MAP.md'}
assert required <= paths, sorted(required - paths)
for file in files:
    assert not excluded(file)
    assert file.stat().st_size < 100 * 1024 * 1024, f'Large file: {file.name}'

# Check relative Markdown links outside fenced code blocks against public files.
checked_links = 0
for file in files:
    if file.suffix != '.md':
        continue
    text = re.sub(r'```.*?```', '', file.read_text(encoding='utf-8-sig'), flags=re.S)
    for raw in re.findall(r'\]\(([^)]+)\)', text):
        destination = unquote(raw.strip().strip('<>')).split('#')[0]
        if not destination or re.match(r'^[a-zA-Z][\w+.-]*:', destination):
            continue
        target = (file.parent / destination).resolve()
        assert target.is_relative_to(root), f'External local link in {file.name}: {raw}'
        relative = target.relative_to(root).as_posix()
        assert relative in paths or (target.is_dir() and relative.split('/')[0] in public_dirs), f'Broken/publicly missing link in {file.relative_to(root)}: {raw}'
        checked_links += 1

for dependency in json.loads((root / 'LICENSES/dependencies.json').read_text()):
    for text in dependency['texts']:
        assert text in paths, text

records = []
for file in files:
    data = file.read_bytes()
    records.append({'path': file.relative_to(root).as_posix(), 'bytes': len(data),
                    'sha256': hashlib.sha256(data).hexdigest()})
manifest = {'project': 'Sky Hopper PSP', 'release': '0.2', 'packagedOn': date.today().isoformat(),
            'license': 'MIT for original project materials; dependencies retain upstream licenses',
            'includes': sorted(public_dirs), 'filesCount': len(files), 'checkedLocalLinks': checked_links,
            'excludes': ['installed tools', 'build caches', 'personal saves', 'local settings',
                         'derived screenshot buffers', 'generated dist archives'],
            'files': records}
payload = (json.dumps(manifest, indent=2) + '\n').encode()
dist = root / 'dist'
dist.mkdir(exist_ok=True)
archive = dist / 'SkyHopper-Source-v0.2.zip'
with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED, compresslevel=9) as package:
    for file in files:
        relative = file.relative_to(root).as_posix()
        entry = zipfile.ZipInfo('SkyHopper/' + relative, date_time=(2026, 10, 8, 0, 0, 0))
        entry.create_system = 3
        permissions = 0o755 if file.suffix == '.sh' else 0o644
        entry.external_attr = (stat.S_IFREG | permissions) << 16
        entry.compress_type = zipfile.ZIP_DEFLATED
        package.writestr(entry, file.read_bytes())
    package.writestr('SkyHopper/SOURCE-MANIFEST.json', payload)
with zipfile.ZipFile(archive) as package:
    assert package.testzip() is None
    assert len(package.namelist()) == len(files) + 1
    for record in records:
        assert hashlib.sha256(package.read('SkyHopper/' + record['path'])).hexdigest() == record['sha256']
(dist / 'source-v0.2-manifest.json').write_bytes(payload)
print(json.dumps({'archive': str(archive), 'publicFiles': len(files),
                  'checkedLocalLinks': checked_links, 'bytes': archive.stat().st_size,
                  'sha256': hashlib.sha256(archive.read_bytes()).hexdigest(), 'checks': 'passed'}, indent=2))
