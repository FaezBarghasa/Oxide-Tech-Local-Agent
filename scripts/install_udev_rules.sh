#!/usr/bin/env bash
# ==============================================================================
# Install udev rules for probe-rs (ST-Link, J-Link, CMSIS-DAP, DAP-Link, ESP-Prog)
# ==============================================================================

set -euo pipefail

RULES_FILE="/etc/udev/rules.d/69-probe-rs.rules"

echo "Installing probe-rs udev rules to ${RULES_FILE}..."

if [ "$EUID" -ne 0 ]; then
    SUDO="sudo"
else
    SUDO=""
fi

$SUDO tee "$RULES_FILE" > /dev/null << 'EOF'
# ST-Link V1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3744", MODE="0666", GROUP="plugdev", TAG+="uaccess"
# ST-Link V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE="0666", GROUP="plugdev", TAG+="uaccess"
# ST-Link V2.1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE="0666", GROUP="plugdev", TAG+="uaccess"
# ST-Link V3
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374d", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374e", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374f", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3752", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3753", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3754", MODE="0666", GROUP="plugdev", TAG+="uaccess"

# SEGGER J-Link
ATTRS{idVendor}=="1366", MODE="0666", GROUP="plugdev", TAG+="uaccess"

# Raspberry Pi Debug Probe / Picoprobe
ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="000c", MODE="0666", GROUP="plugdev", TAG+="uaccess"

# CMSIS-DAP compatible adapters
ATTRS{idVendor}=="0d28", ATTRS{idProduct}=="0204", MODE="0666", GROUP="plugdev", TAG+="uaccess"
# WCH-Link
ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="8010", MODE="0666", GROUP="plugdev", TAG+="uaccess"
ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="8012", MODE="0666", GROUP="plugdev", TAG+="uaccess"

# ESP-Prog
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6010", MODE="0666", GROUP="plugdev", TAG+="uaccess"
EOF

echo "Reloading udev rules..."
$SUDO udevadm control --reload-rules
$SUDO udevadm trigger

echo "Done! You can now access hardware debug probes without root permissions."
