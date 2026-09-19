# Oxide Agent — single-binary desktop (Tauri) + single `.deb`

One binary runs everything; one `.deb` installs it all.

## What the binary does

`oxide-agent` (crate `src-tauri`) is the only production artifact:

| Invocation | Behaviour |
|---|---|
| `oxide-agent` / `oxide-agent desktop [--config PATH]` | Tauri window (Oxide Agent Studio) + gateway embedded on a background thread (`127.0.0.1:8080`) + memory via bundled `oxide-embed` sidecar |
| `oxide-agent daemon [--config PATH]` | Headless gateway foreground process — this is what systemd runs |
| `oxide-agent doctor [--gateway-url URL]` | Diagnostics: `oxide-embed` presence/version, `.oxide/manifest.json`, gateway probe |
| `oxide-agent memory <args…>` | Direct `oxide-embed` passthrough (`init`, `index`, `search`, `context`, `remember`, `recall`, `conflicts`, `explain`) |
| `oxide-agent status [--gateway-url URL]` | Gateway liveness probe |

All project memory flows through `oxide-embed` — the desktop UI calls the
`memory_*` Tauri commands (`src-tauri/src/memory.rs`), which resolve the
sidecar in this order: `OXIDE_EMBED_BIN` → dir next to the app binary
(Tauri `externalBin` staging) → `/usr/lib/oxide-agent/oxide-embed` →
`/usr/bin/oxide-embed` → `PATH`.

## Build the `.deb` (one file)

Prerequisites (Debian/Ubuntu/Pop!_OS):

```bash
sudo apt update && sudo apt install -y \
  build-essential curl pkg-config libssl-dev \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf xdg-utils
```

Plus: Rust ≥ 1.85, pnpm, and an `oxide-embed` 0.3.x binary
(`~/.local/bin/oxide-embed`, `./bin/oxide-embed`, or `$OXIDE_EMBED_BIN`).

```bash
# from the repo root
pnpm install
pnpm run desktop:build
```

This stages the sidecar (`scripts/fetch-sidecar.sh` →
`src-tauri/binaries/oxide-embed-<triple>`), builds the studio frontend, and
emits exactly one installer, e.g.:

```text
src-tauri/target/release/bundle/deb/oxide-agent_0.5.0_amd64.deb
```

Install:

```bash
sudo apt install ./src-tauri/target/release/bundle/deb/oxide-agent_0.5.0_amd64.deb
oxide-agent doctor
oxide-agent            # desktop
```

## Headless / service mode

```bash
oxide-agent daemon --config /etc/oxide-tech/config.toml
```

Legacy `debian/oxide-agent.service` keeps working unchanged because the new
binary honours the same `daemon --config …` CLI (`ExecStart=/usr/bin/oxide-agent
daemon --config /etc/oxide-tech/config.toml`).

## Dev loop (no packaging)

```bash
pnpm install
pnpm run desktop:dev     # Tauri window + vite HMR on :1420, gateway embedded
```

Browser-only fallback still works (`cd ui/oxide-agent-studio && pnpm dev`,
gateway at `:8080`); the Memory tab shows a banner there because direct
`oxide-embed` access requires the desktop runtime.

## Project memory quickstart (inside the app)

1. Open the **Project Memory** tab, set the project directory (the one with `.oxide/`).
2. `Re-index` (runs `oxide-embed index --force`), then search / recall / remember.
3. Same operations from a terminal: `oxide-agent memory search "dma init" --stair`.

## Files

- `src-tauri/` — Tauri app: `src/main.rs` (single binary), `src/memory.rs`
  (oxide-embed bridge + `memory_*` commands), `src/gateway_rt.rs` (embedded
  gateway), `tauri.conf.json` (bundle → single `.deb`), `capabilities/`.
- `scripts/fetch-sidecar.sh` — stages `oxide-embed` for the bundler.
- `ui/oxide-agent-studio/src/lib/desktop.ts` — typed Tauri `invoke` wrappers
  with gateway-HTTP fallback in browsers.
- `ui/oxide-agent-studio/src/components/MemoryTab.tsx` — studio Memory tab.
