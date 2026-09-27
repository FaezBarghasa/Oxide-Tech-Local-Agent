#!/usr/bin/env python3
"""
Verify Tauri IPC Bindings Integrity
Validates that every command invoked in the frontend (TS/TSX) is implemented and registered in src-tauri/src/
"""
import re
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
SRC_DIR = ROOT_DIR / "src" / "src"
TAURI_SRC_DIR = ROOT_DIR / "src-tauri" / "src"

INVOKE_PATTERNS = [
    re.compile(r"invoke(?:<[^>]+>)?\(\s*['\"]([a-zA-Z0-9_]+)['\"]"),
    re.compile(r"tauriInvoke(?:<[^>]+>)?\(\s*['\"]([a-zA-Z0-9_]+)['\"]"),
]

def find_frontend_invocations():
    invocations = set()
    for ext in ("*.ts", "*.tsx"):
        for path in SRC_DIR.rglob(ext):
            try:
                content = path.read_text(encoding="utf-8")
                for pattern in INVOKE_PATTERNS:
                    for match in pattern.finditer(content):
                        invocations.add(match.group(1))
            except Exception as e:
                print(f"Error reading {path}: {e}", file=sys.stderr)
    return invocations

def find_tauri_registered_handlers():
    main_rs = TAURI_SRC_DIR / "main.rs"
    if not main_rs.exists():
        print(f"Missing {main_rs}", file=sys.stderr)
        return set()
    
    content = main_rs.read_text(encoding="utf-8")
    handler_match = re.search(r"generate_handler!\[(.*?)\]", content, re.DOTALL)
    if not handler_match:
        print("Could not find generate_handler! in main.rs", file=sys.stderr)
        return set()
    
    handlers_raw = handler_match.group(1)
    handlers = set()
    for raw_line in handlers_raw.splitlines():
        # Remove comment if any
        line = raw_line.split("//")[0].strip()
        if not line:
            continue
        for item in line.split(","):
            item = item.strip()
            if not item:
                continue
            cmd_name = item.split("::")[-1].strip()
            if cmd_name:
                handlers.add(cmd_name)
    return handlers

def main():
    print("[+] Auditing Frontend Tauri IPC Bindings against Rust Backend...")
    invoked = find_frontend_invocations()
    registered = find_tauri_registered_handlers()

    print(f"  • Found {len(invoked)} unique IPC invocations in frontend.")
    print(f"  • Found {len(registered)} registered IPC handlers in src-tauri/src/main.rs.")

    missing = invoked - registered
    if missing:
        print("\n[!] FATAL: The following frontend IPC commands are NOT registered in Rust backend:")
        for m in sorted(missing):
            print(f"    - {m}")
        sys.exit(1)
    else:
        print("\n[✓] All frontend IPC commands are properly registered and verified in Rust backend.")
        for cmd in sorted(invoked):
            print(f"    ✓ {cmd}")
        sys.exit(0)

if __name__ == "__main__":
    main()
