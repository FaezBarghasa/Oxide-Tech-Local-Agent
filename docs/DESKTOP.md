# Oxide-Tech Local Agent — Single-Binary Desktop Monolith (Tauri v2)

One binary runs everything; one package installs it all.

## What the binary does

`oxide-tech-local-agent` (crate `src-tauri`) is the unified desktop & headless entry point:

| Invocation | Behaviour |
| --- | --- |
| `oxide-tech-local-agent` / `oxide-tech-local-agent desktop [--config PATH]` | Tauri window (Oxide Agent Studio) + gateway embedded on a background thread (`127.0.0.1:8080`) + native `llama-server` orchestrator (`127.0.0.1:8081`) + memory via bundled `oxide-embed` sidecar |
| `oxide-tech-local-agent daemon [--config PATH]` | Headless gateway foreground process — this is what systemd runs |
| `oxide-tech-local-agent doctor [--gateway-url URL] [--json]` | Diagnostics: `oxide-embed` presence/version, `.oxide/manifest.json`, gateway probe, toolchains |
| `oxide-tech-local-agent re-forge <FILE> [--arch ARCH]` | Pure-Rust binary and PTX GPU reverse engineering and decompilation |
| `oxide-tech-local-agent verify [--workspace PATH] [--json]` | Deterministic verifier suite & cryptographic evidence bundle |
| `oxide-tech-local-agent memory <args…>` | Direct `oxide-embed` passthrough (`init`, `index`, `search`, `context`, `remember`, `recall`, `conflicts`, `explain`) |
| `oxide-tech-local-agent status [--gateway-url URL]` | Gateway liveness probe |

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

Plus: Rust ≥ 1.85, pnpm, and an `oxide-embed` binary on `PATH`.

```bash
# from the repo root
pnpm install
pnpm run desktop:build
```

Emits exactly one installer:

```text
src-tauri/target/release/bundle/deb/oxide-tech-local-agent_0.6.0_amd64.deb
```

Install:

```bash
sudo apt install ./src-tauri/target/release/bundle/deb/oxide-tech-local-agent_0.6.0_amd64.deb
oxide-tech-local-agent doctor
oxide-tech-local-agent            # desktop
```

## Headless / service mode

```bash
oxide-tech-local-agent daemon --config /etc/oxide-tech/config.toml
```

Legacy `debian/oxide-agent.service` keeps working because the binary honours the same `daemon --config …` CLI (`ExecStart=/usr/bin/oxide-tech-local-agent daemon --config /etc/oxide-tech/config.toml`).

## Dev loop (no packaging)

```bash
pnpm install
pnpm run desktop:dev     # Tauri window + vite HMR on :1420, gateway embedded
```

Browser-only fallback still works (`cd ui/oxide-agent-studio && pnpm dev`,
gateway at `:8080`); the Memory tab shows a banner there because direct
`oxide-embed` access requires the desktop runtime.

## Desktop Studio View Matrix

The desktop application includes a comprehensive engineering studio:

- **System Doctor (`DoctorTab.tsx`)**: Target probe discovery (`probe-rs`), Linux udev rules deployment, and database connectivity.
- **Model Catalog & Manager (`CatalogTab.tsx`)**: Recursive GGUF weight scanning (`~/models`), GPU offload parameter controls (`-ngl 99`), and active `llama-server` lifecycle supervision.
- **Universal Gateway (`GatewayTab.tsx`)**: 19 routing strategy policies, browser-authenticated cloud account management, and real-time token bucket gauges.
- **RE-Forge Studio (`ReForgeTab.tsx`)**: Zero-copy binary parsing, ARM Cortex-M Vector Table decoding, Shannon entropy graphing, and neural safe-Rust decompilation.
- **Verification Matrix (`VerificationTab.tsx`)**: Real-time test suite execution, atomic git stash checkpointer rollback, and signed cryptographic evidence bundle export.
- **Project Memory & GraphRAG (`MemoryTab.tsx`)**: Integrated `oxide-embed` semantic memory fabric, STAIR Code-ToC leaf search, contradiction detection, and 2-hop topological call graphs.
- **Configuration & Profiles (`SettingsTab.tsx`)**: Dynamic TOML profile switching (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`).

## Files

- `src-tauri/` — Tauri v2 app:
  - `src/main.rs`: Entry point and registered IPC commands.
  - `src/model_ipc.rs`: Dedicated `llama-server` process management and real local GGUF discovery.
  - `src/doctor.rs`: Hardware and system diagnostics.
  - `src/reforge_ipc.rs`: Binary reverse engineering and decompilation.
  - `src/verifier_ipc.rs`: Deterministic verification and evidence bundles.
  - `src/config_ipc.rs`: Dynamic TOML profile persistence.
  - `src/memory.rs`: `oxide-embed` bridge, STAIR search, and conflict auditor.
  - `src/gateway_rt.rs`: Embedded Actix-Web + Quinn QUIC runtime.
- `src/` — React 19 / Vite UI (packaged via `rust-embed` into the desktop binary):
  - `src/components/DoctorTab.tsx`: Interactive hardware & permissions panel.
  - `src/components/CatalogTab.tsx`: Model catalog and local GGUF manager.
  - `src/components/ReForgeTab.tsx`: Disassembly, vector table, and entropy viewer.
  - `src/components/VerificationTab.tsx`: Test runner & evidence exporter.
  - `src/components/MemoryTab.tsx`: Semantic memory & GraphRAG browser.
  - `src/components/SettingsTab.tsx`: Real-time configuration editor.
