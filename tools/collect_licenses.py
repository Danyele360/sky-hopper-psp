"""Collect license texts from the exact Cargo.lock dependencies already fetched.

Run after cargo fetch, from the Rust environment used to build the game.
Optional: --sdk /path/to/the/pinned/rust-psp/checkout
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import tomllib

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--sdk', type=Path)
args = parser.parse_args()
cargo_home = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo')))
output = root / 'LICENSES'
output.mkdir(exist_ok=True)
records = []
lock = tomllib.loads((root / 'Cargo.lock').read_text(encoding='utf-8'))
for package in lock['package']:
    source = package.get('source', '')
    if source.startswith('registry+'):
        matches = list((cargo_home / 'registry/src').glob(f"*/{package['name']}-{package['version']}"))
        if not matches:
            raise SystemExit(f"Missing {package['name']}: run cargo fetch --locked first")
        directory = matches[0]
        metadata = tomllib.loads((directory / 'Cargo.toml').read_text(encoding='utf-8'))['package']
        texts = sorted(p for p in directory.iterdir() if p.is_file() and
                       (p.name.upper().startswith(('LICENSE', 'COPYING', 'COPYRIGHT'))))
        destination = output / f"{package['name']}-{package['version']}"
        destination.mkdir(exist_ok=True)
        if not texts:
            raise SystemExit(f'No license texts for {directory.name}')
        for text in texts:
            shutil.copyfile(text, destination / text.name)
        records.append({'name': package['name'], 'version': package['version'],
                        'license': metadata.get('license'),
                        'source': metadata.get('repository', source),
                        'texts': [str((destination / p.name).relative_to(root)).replace('\\', '/') for p in texts]})

sdk = args.sdk
if sdk is None:
    matches = list((cargo_home / 'git/checkouts').glob('rust-psp-*/ad92131*/LICENSE'))
    if not matches:
        raise SystemExit('SDK checkout not found; supply --sdk /path/to/rust-psp')
    sdk = matches[0].parent
shutil.copyfile(sdk / 'LICENSE', output / 'rust-psp.txt')
records.append({'name': 'rust-psp', 'version': '1.0.0',
                'revision': 'ad92131218406308170a3344786865415aef2995',
                'license': 'MIT; includes PSPSDK reference notice',
                'source': 'https://github.com/AndrewAltimit/rust-psp',
                'texts': ['LICENSES/rust-psp.txt']})
(output / 'dependencies.json').write_text(json.dumps(records, indent=2) + '\n', encoding='utf-8')
print(f'Collected license texts for {len(records)} locked dependencies')
