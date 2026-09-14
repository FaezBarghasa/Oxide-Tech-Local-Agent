# Pop!_OS 24.04 (LTS) Development & Hardware Execution Setup Guide

This guide details configuring **Oxide-Tech Local Agent OS** on **Pop!_OS 24.04 (GNOME Wayland / X11)** for Embedded Systems (`probe-rs`, `STM32`), deterministic sandboxing (`bubblewrap`), and local inference.

---

## 1. System Dependencies & Bubblewrap (bwrap) Sandbox

Oxide-Tech relies on `bubblewrap` for isolated atomic compilation and verification checks.

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    bubblewrap \
    ca-certificates \
    git \
    curl \
    udev
```

Verify that unprivileged user namespaces and bubblewrap are functioning:
```bash
bwrap --ro-bind / / --dev /dev --proc /proc echo "Bubblewrap sandbox active!"
```

---

## 2. Hardware Debugging & Flashing (`probe-rs` / `udev` Rules)

To enable non-root flashing and debugging for STM32 and ARM Cortex-M debuggers (ST-Link, J-Link, CMSIS-DAP, Black Magic Probe):

1. Install the official `probe-rs` udev rules:
```bash
sudo curl -fsSL https://probe.rs/files/69-probe-rs.rules -o /etc/udev/rules.d/69-probe-rs.rules
```

2. Reload and apply udev rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

3. Add your local user to the `plugdev` and `dialout` groups:
```bash
sudo usermod -aG plugdev,dialout $USER
```

---

## 3. Local Infrastructure Stack (SurrealDB + Qdrant + Ollama)

Start the local database and fallback model stack with one command:

```bash
docker compose up -d
```

Verify containers are running:
- **SurrealDB**: `http://127.0.0.1:8000`
- **Qdrant**: `http://127.0.0.1:6333`
- **Ollama**: `http://127.0.0.1:11434`

---

## 4. Building & Running Oxide-Tech Local Agent

```bash
# Build release binaries
cargo build --release -p gateway

# Start control plane gateway
./target/release/gateway --port 8080
```
