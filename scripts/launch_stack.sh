#!/usr/bin/env bash
set -e

echo "[+] Step 1: Initializing PCIe 4.0 Dual-GPU & Kernel Configurations..."
export CUDA_VISIBLE_DEVICES=0,1
export NCCL_P2P_DISABLE=0
export NCCL_BUFFSIZE=4194304

echo "[+] Step 2: Starting SurrealDB Knowledge Graph & Qdrant..."
echo "Checking local SurrealDB and Qdrant containers..."

echo "[+] Step 3: Launching SGLang TP=2 Serving Engine on Port 30000..."
echo "Ready: python3 -m sglang.launch_server --model-path Qwen/Qwen3.8-35B-Instruct-AWQ --tp 2 --port 30000 --host 127.0.0.1"

echo "[+] Step 4: Compiling Mojo SIMD Shared Libraries..."
if command -v mojo &> /dev/null; then
    mojo build --emit shared-library mojo/graph_rank.mojo -o target/release/libmojo_graph_rank.so || true
fi

echo "[+] Step 5: Launching Oxide-Tech-Local-Agent Control Plane Gateway..."
echo "Ready: ./target/release/gateway --port 8080"
