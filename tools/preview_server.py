"""Local-only server. Run with python tools/preview_server.py."""
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import argparse
import functools
import webbrowser

parser=argparse.ArgumentParser()
parser.add_argument('--no-browser',action='store_true')
parser.add_argument('--port',type=int,default=8787)
args=parser.parse_args()
root=Path(__file__).resolve().parents[1]
url=f'http://127.0.0.1:{args.port}/preview/'
handler=functools.partial(SimpleHTTPRequestHandler,directory=str(root))
try:
    server=ThreadingHTTPServer(('127.0.0.1',args.port),handler)
except OSError:
    print(f'La porta {args.port} e gia occupata. Usa --port 8788.')
    raise SystemExit(1)
print(f'Sky Hopper: {url}\nCtrl+C per chiudere il server.',flush=True)
if not args.no_browser:webbrowser.open(url)
try:server.serve_forever()
except KeyboardInterrupt:server.server_close()
