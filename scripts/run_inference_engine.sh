#!/usr/bin/env bash
set -e

export CUDA_VISIBLE_DEVICES=0,1

echo "Starting SGLang High-Throughput CUDA 13.3 Serving Runtime on dual GPUs (TP=2)..."

sglang serve \
  --model-path Qwen/Qwen2.5-Coder-32B-Instruct-AWQ \
  --tp-size 2 \
  --host 0.0.0.0 \
  --port 8080 \
  --mem-fraction-static 0.90 \
  --enable-p2p-check \
  --context-length 32768 \
  --log-level info
