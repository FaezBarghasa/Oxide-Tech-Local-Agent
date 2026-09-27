#!/usr/bin/env bash
# ============================================================================
# Oxide-Tech-Local-Agent: Local Weights & Model Downloader
# Fetches quantization checkpoints (GGUF, safetensors) for offline-first inference.
# ============================================================================

set -euo pipefail

MODELS_DIR="${1:-crates/models}"
mkdir -p "$MODELS_DIR"

echo "[*] Initializing model directory at: $MODELS_DIR"

BASE_URL="https://huggingface.co/Qwen"
DEFAULT_MODELS=(
    "Qwen2.5-Coder-7B-Instruct-GGUF/resolve/main/qwen2.5-coder-7b-instruct-q4_k_m.gguf"
)

download_file() {
    local target_rel="$1"
    local filename
    filename="$(basename "$target_rel")"
    local target_path="$MODELS_DIR/$filename"

    if [ -f "$target_path" ]; then
        echo "[+] Model file '$filename' already exists. Skipping."
        return 0
    fi

    local url="$BASE_URL/$target_rel"
    echo "[*] Downloading $filename from $url ..."

    if command -v curl >/dev/null 2>&1; then
        curl -L -C - --retry 3 --retry-delay 2 --progress-bar -o "$target_path.tmp" "$url"
    elif command -v wget >/dev/null 2>&1; then
        wget -c --tries=3 -O "$target_path.tmp" "$url"
    else
        echo "[!] Neither curl nor wget found. Cannot proceed." >&2
        return 1
    fi

    mv "$target_path.tmp" "$target_path"
    echo "[+] Successfully downloaded $filename"
}

for model_rel in "${DEFAULT_MODELS[@]}"; do
    download_file "$model_rel" || echo "[!] Notice: Check network connection for model download."
done

echo "[+] Model repository ready at $MODELS_DIR"
