from pathlib import Path
import shutil
import subprocess
from http.server import HTTPServer, SimpleHTTPRequestHandler

class Targets:
    Debug = "Debug"
    Release = "Release"

WASM_BINARY_NAME = "absolute_defence.wasm"
TARGET_DIR = Path("target")
TARGET_WASM_DIR = TARGET_DIR / "wasm32-unknown-unknown"
DEBUG_TARGET = TARGET_WASM_DIR / f"debug/{WASM_BINARY_NAME}"
RELEASE_TARGET = TARGET_WASM_DIR / f"release/{WASM_BINARY_NAME}"
SRC_WEB_DIR = Path("docs")
SRC_WEB_ASSET_DIR = SRC_WEB_DIR / "assets"

BROWSER = "explorer"
COMPILE_TARGET = Targets.Debug

def main():
    {
        Targets.Debug: run_debug,
        Targets.Release: run_release,
    }[COMPILE_TARGET]()

def run_debug():
    run_process(make_compile_commands() + ["--dev"])
    sanitize_web_files()
    start_server()
def run_release():
    run_process(make_compile_commands() + ["--release", "--", "--no-default-features"])
    sanitize_web_files()
    start_server()

def make_compile_commands():
    return [
        "wasm-pack",
        "build",
        "--out-dir", str(SRC_WEB_ASSET_DIR),
        "--target", "web",
    ]

def run_process(command):
    subprocess.run(command, check=True)

def sanitize_web_files():
    # We don't want a couple of files that get created
    to_delete = [path for path in SRC_WEB_ASSET_DIR.glob("*.d.ts")]
    to_delete += [
        SRC_WEB_ASSET_DIR / ".gitignore",
        SRC_WEB_ASSET_DIR / "package.json",
    ]
    for path in to_delete: path.unlink()

def start_server():
    class OurHandler(SimpleHTTPRequestHandler):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, directory=str(SRC_WEB_DIR), **kwargs)
    static_server = HTTPServer(("localhost", 8080), OurHandler)

    # Apparently Windows Explorer returns a 1 here
    subprocess.run([BROWSER, "http://localhost:8080"])

    print("Starting the static server")
    try:
        static_server.serve_forever()
    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    main()
