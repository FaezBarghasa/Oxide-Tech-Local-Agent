#!/usr/bin/env bash
# ==============================================================================
# Oxide-Tech Local Agent OS - System & Environment Diagnostics (doctor.sh)
# ==============================================================================

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}======================================================${NC}"
echo -e "${BLUE}   Oxide-Tech Local Agent OS - Doctor Diagnostics     ${NC}"
echo -e "${BLUE}======================================================${NC}"
echo ""

PASS_COUNT=0
WARN_COUNT=0
FAIL_COUNT=0

check_cmd() {
    local name="$1"
    local cmd="$2"
    local required="${3:-true}"
    
    if command -v "$cmd" &>/dev/null; then
        local ver
        ver=$($cmd --version 2>&1 | head -n 1 || echo "installed")
        echo -e "  [${GREEN}OK${NC}] $name: $ver"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        if [ "$required" = "true" ]; then
            echo -e "  [${RED}FAIL${NC}] $name ($cmd) is NOT installed."
            FAIL_COUNT=$((FAIL_COUNT + 1))
        else
            echo -e "  [${YELLOW}WARN${NC}] $name ($cmd) not found (optional)."
            WARN_COUNT=$((WARN_COUNT + 1))
        fi
    fi
}

echo -e "${BLUE}1. Host & Core Toolchain:${NC}"
check_cmd "Rust Compiler" "rustc" "true"
check_cmd "Cargo" "cargo" "true"
check_cmd "Node.js" "node" "true"
check_cmd "pnpm" "pnpm" "false"
check_cmd "Bubblewrap Sandbox" "bwrap" "true"
check_cmd "Git" "git" "true"

echo ""
echo -e "${BLUE}2. Embedded & Simulation Ecosystem:${NC}"
check_cmd "probe-rs" "probe-rs" "false"
check_cmd "QEMU x86_64" "qemu-system-x86_64" "false"
check_cmd "KiCad CLI" "kicad-cli" "false"

echo ""
echo -e "${BLUE}3. Inference & Storage Backends:${NC}"
check_cmd "Ollama" "ollama" "false"
check_cmd "Docker (optional)" "docker" "false"

# Check GPU / NVIDIA
echo ""
echo -e "${BLUE}4. Hardware Acceleration (GPU):${NC}"
if command -v nvidia-smi &>/dev/null; then
    gpu_info=$(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader 2>/dev/null || echo "NVIDIA GPU Detected")
    echo -e "  [${GREEN}OK${NC}] NVIDIA GPU: $gpu_info"
    PASS_COUNT=$((PASS_COUNT + 1))
else
    echo -e "  [${YELLOW}INFO${NC}] No NVIDIA GPU detected. System will operate in CPU Lite Mode (Ollama/llama.cpp)."
fi

# Check udev rules for probe-rs
echo ""
echo -e "${BLUE}5. System Security & Permissions:${NC}"
if [ -f "/etc/udev/rules.d/99-probe-rs.rules" ] || [ -f "/usr/lib/udev/rules.d/69-probe-rs.rules" ]; then
    echo -e "  [${GREEN}OK${NC}] probe-rs udev rules present."
    PASS_COUNT=$((PASS_COUNT + 1))
else
    echo -e "  [${YELLOW}WARN${NC}] probe-rs udev rules not found. Hardware flashing may require sudo unless scripts/install_udev_rules.sh is run."
    WARN_COUNT=$((WARN_COUNT + 1))
fi

echo ""
echo -e "${BLUE}======================================================${NC}"
echo -e "Summary: ${GREEN}$PASS_COUNT passed${NC}, ${YELLOW}$WARN_COUNT warnings${NC}, ${RED}$FAIL_COUNT failed${NC}"
if [ "$FAIL_COUNT" -eq 0 ]; then
    echo -e "${GREEN}System is ready to run Oxide-Tech Local Agent OS!${NC}"
else
    echo -e "${RED}Please install missing required dependencies before proceeding.${NC}"
    exit 1
fi
