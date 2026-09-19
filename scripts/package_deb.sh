#!/usr/bin/env bash
# ==============================================================================
# Oxide-Tech Local Agent OS — Debian (.deb) Package Builder
# Single-binary delivery with embedded Tauri Desktop GUI + oxide-embed memory
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

PKG_NAME="oxide-tech-local-agent"
PKG_VERSION="0.5.0"
TARGET_DIR="${ROOT_DIR}/target"
OUTPUT_DIR="${TARGET_DIR}/debian"
STAGE_DIR="${OUTPUT_DIR}/stage"

# Detect System Architecture for Debian
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")

echo "======================================================================"
echo " Packaging ${PKG_NAME} v${PKG_VERSION} (${ARCH})"
echo "======================================================================"

# 1. Clean previous staging workspace
rm -rf "${STAGE_DIR}"
mkdir -p "${OUTPUT_DIR}"

# 2. Build Frontend UI Studio
echo "[+] Step 1: Compiling Frontend UI Studio..."
UI_DIR="${ROOT_DIR}/ui/oxide-agent-studio"
if [[ -d "${UI_DIR}" ]]; then
    if command -v pnpm &>/dev/null; then
        echo "[*] Building UI with pnpm in ${UI_DIR}..."
        (cd "${UI_DIR}" && pnpm run build:tauri)
    elif command -v npm &>/dev/null; then
        echo "[*] Building UI with npm in ${UI_DIR}..."
        (cd "${UI_DIR}" && npm run build:tauri)
    fi
fi

# 3. Build Universal Rust Desktop Binary (Tauri v2 + CLI engines)
echo "[+] Step 2: Compiling universal release binary (oxide-tech-local-agent)..."
cd "${ROOT_DIR}"
cargo build --release -p oxide-tech-local-agent

AGENT_BIN="${TARGET_DIR}/release/oxide-tech-local-agent"
if [[ ! -f "${AGENT_BIN}" ]]; then
    AGENT_BIN="${TARGET_DIR}/release/oxide_tech_local_agent"
fi

if [[ ! -f "${AGENT_BIN}" ]]; then
    echo "[-] Error: Failed to find compiled binary at ${AGENT_BIN}"
    exit 1
fi

# 4. Construct Debian Staging Hierarchy
echo "[+] Step 3: Staging file hierarchy..."
mkdir -p "${STAGE_DIR}/DEBIAN"
mkdir -p "${STAGE_DIR}/usr/bin"
mkdir -p "${STAGE_DIR}/usr/lib/oxide-tech-local-agent"
mkdir -p "${STAGE_DIR}/usr/share/applications"
mkdir -p "${STAGE_DIR}/usr/share/icons/hicolor/128x128/apps"
mkdir -p "${STAGE_DIR}/usr/share/icons/hicolor/32x32/apps"
mkdir -p "${STAGE_DIR}/usr/share/icons/hicolor/scalable/apps"
mkdir -p "${STAGE_DIR}/etc/oxide-tech"
mkdir -p "${STAGE_DIR}/lib/systemd/system"
mkdir -p "${STAGE_DIR}/usr/share/oxide-tech/studio"
mkdir -p "${STAGE_DIR}/var/lib/oxide-tech"
mkdir -p "${STAGE_DIR}/var/log/oxide-tech"

# Copy binary & create compatibility symlinks
cp "${AGENT_BIN}" "${STAGE_DIR}/usr/bin/oxide-tech-local-agent"
chmod 755 "${STAGE_DIR}/usr/bin/oxide-tech-local-agent"
ln -sf "/usr/bin/oxide-tech-local-agent" "${STAGE_DIR}/usr/bin/oxide-agent"
ln -sf "/usr/bin/oxide-tech-local-agent" "${STAGE_DIR}/usr/bin/oxide-gateway"

# Copy oxide-embed sidecar
EMBED_SRC="${ROOT_DIR}/src-tauri/binaries/oxide-embed-x86_64-unknown-linux-gnu"
if [[ -f "${EMBED_SRC}" ]]; then
    echo "[+] Staging bundled oxide-embed sidecar..."
    cp "${EMBED_SRC}" "${STAGE_DIR}/usr/lib/oxide-tech-local-agent/oxide-embed"
    chmod 755 "${STAGE_DIR}/usr/lib/oxide-tech-local-agent/oxide-embed"
    ln -sf "/usr/lib/oxide-tech-local-agent/oxide-embed" "${STAGE_DIR}/usr/bin/oxide-embed"
fi

# Copy Desktop entry & icons
if [[ -f "${ROOT_DIR}/debian/oxide-tech-local-agent.desktop" ]]; then
    cp "${ROOT_DIR}/debian/oxide-tech-local-agent.desktop" "${STAGE_DIR}/usr/share/applications/oxide-tech-local-agent.desktop"
    chmod 644 "${STAGE_DIR}/usr/share/applications/oxide-tech-local-agent.desktop"
fi

if [[ -f "${ROOT_DIR}/src-tauri/icons/128x128.png" ]]; then
    cp "${ROOT_DIR}/src-tauri/icons/128x128.png" "${STAGE_DIR}/usr/share/icons/hicolor/128x128/apps/oxide-tech-local-agent.png"
fi
if [[ -f "${ROOT_DIR}/src-tauri/icons/32x32.png" ]]; then
    cp "${ROOT_DIR}/src-tauri/icons/32x32.png" "${STAGE_DIR}/usr/share/icons/hicolor/32x32/apps/oxide-tech-local-agent.png"
fi
if [[ -f "${ROOT_DIR}/src-tauri/icon.svg" ]]; then
    cp "${ROOT_DIR}/src-tauri/icon.svg" "${STAGE_DIR}/usr/share/icons/hicolor/scalable/apps/oxide-tech-local-agent.svg"
fi

# Copy default configuration
cp "${ROOT_DIR}/config.toml" "${STAGE_DIR}/etc/oxide-tech/config.toml"
chmod 644 "${STAGE_DIR}/etc/oxide-tech/config.toml"

# Copy systemd service
cp "${ROOT_DIR}/debian/oxide-agent.service" "${STAGE_DIR}/lib/systemd/system/oxide-agent.service"
chmod 644 "${STAGE_DIR}/lib/systemd/system/oxide-agent.service"

# Copy UI static assets
if [[ -d "${UI_DIR}/dist" ]]; then
    cp -r "${UI_DIR}/dist/"* "${STAGE_DIR}/usr/share/oxide-tech/studio/"
fi

# Copy Debian maintainer scripts & control files
sed -e "s/@VERSION@/${PKG_VERSION}/g" \
    -e "s/@ARCH@/${ARCH}/g" \
    "${ROOT_DIR}/debian/control.template" > "${STAGE_DIR}/DEBIAN/control"

cp "${ROOT_DIR}/debian/conffiles" "${STAGE_DIR}/DEBIAN/conffiles"
cp "${ROOT_DIR}/debian/postinst" "${STAGE_DIR}/DEBIAN/postinst"
cp "${ROOT_DIR}/debian/prerm" "${STAGE_DIR}/DEBIAN/prerm"

chmod 755 "${STAGE_DIR}/DEBIAN/postinst"
chmod 755 "${STAGE_DIR}/DEBIAN/prerm"
chmod 644 "${STAGE_DIR}/DEBIAN/control"
chmod 644 "${STAGE_DIR}/DEBIAN/conffiles"

# 5. Build Debian Package using dpkg-deb
DEB_FILE="${OUTPUT_DIR}/${PKG_NAME}_${PKG_VERSION}_${ARCH}.deb"
echo "[+] Step 4: Building Debian package with dpkg-deb..."
dpkg-deb --root-owner-group --build "${STAGE_DIR}" "${DEB_FILE}"

echo "======================================================================"
echo "[✓] Successfully created Debian package:"
echo "    ${DEB_FILE}"
echo "======================================================================"
