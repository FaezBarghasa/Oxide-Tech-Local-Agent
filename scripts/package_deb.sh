#!/usr/bin/env bash
# ==============================================================================
# Oxide-Tech Local Agent OS — Debian (.deb) Package Builder
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

PKG_NAME="oxide-tech-local-agent"
PKG_VERSION="0.1.0"
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

# 2. Build Rust Gateway binary in release mode
echo "[+] Step 1: Compiling release binaries (gateway)..."
cd "${ROOT_DIR}"
cargo build --release -p gateway

GATEWAY_BIN="${TARGET_DIR}/release/gateway"
if [[ ! -f "${GATEWAY_BIN}" ]]; then
    echo "[-] Error: Binary ${GATEWAY_BIN} not found!" >&2
    exit 1
fi

# 3. Optional: Build Frontend UI if pnpm/node is present
echo "[+] Step 2: Preparing Frontend UI Studio..."
UI_DIR="${ROOT_DIR}/ui/oxide-agent-studio"
if [[ -d "${UI_DIR}" ]]; then
    if [[ ! -d "${UI_DIR}/dist" ]]; then
        if command -v pnpm &>/dev/null; then
            echo "[*] Building UI with pnpm in ${UI_DIR}..."
            (cd "${UI_DIR}" && pnpm build || true)
        elif command -v npm &>/dev/null; then
            echo "[*] Building UI with npm in ${UI_DIR}..."
            (cd "${UI_DIR}" && npm run build || true)
        fi
    fi
fi

# 4. Construct Debian Staging Hierarchy
echo "[+] Step 3: Staging file hierarchy..."
mkdir -p "${STAGE_DIR}/DEBIAN"
mkdir -p "${STAGE_DIR}/usr/bin"
mkdir -p "${STAGE_DIR}/etc/oxide-tech"
mkdir -p "${STAGE_DIR}/lib/systemd/system"
mkdir -p "${STAGE_DIR}/usr/share/oxide-tech/studio"
mkdir -p "${STAGE_DIR}/var/lib/oxide-tech"
mkdir -p "${STAGE_DIR}/var/log/oxide-tech"

# Copy binary & create symlinks
cp "${GATEWAY_BIN}" "${STAGE_DIR}/usr/bin/oxide-gateway"
chmod 755 "${STAGE_DIR}/usr/bin/oxide-gateway"
ln -sf "/usr/bin/oxide-gateway" "${STAGE_DIR}/usr/bin/oxide-tech-agent"

# Copy default configuration
cp "${ROOT_DIR}/config.toml" "${STAGE_DIR}/etc/oxide-tech/config.toml"
chmod 644 "${STAGE_DIR}/etc/oxide-tech/config.toml"

# Copy systemd service
cp "${ROOT_DIR}/debian/oxide-agent.service" "${STAGE_DIR}/lib/systemd/system/oxide-agent.service"
chmod 644 "${STAGE_DIR}/lib/systemd/system/oxide-agent.service"

# Copy UI static assets if present
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
