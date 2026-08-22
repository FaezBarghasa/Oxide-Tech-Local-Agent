#!/usr/bin/env bash
set -euo pipefail

export CUDA_VISIBLE_DEVICES=0,1
export NCCL_DEBUG=WARN
export NCCL_IB_DISABLE=1 

echo "Starting Speculative-Decoding vLLM Server on 2x RTX 3090..."
# Speculative decoding pairing 27B Fable-5 (Chief) with 3B VibeThinker (Draft)
python3 -m vllm.entrypoints.openai.api_server \
    --model TeichAI/Qwen3.6-27B-Fable-5-Experimental-GGUF \
    --tensor-parallel-size 2 \
    --speculative-model mradermacher/Mythos-nano-i1-GGUF \
    --num-speculative-tokens 5 \
    --gpu-memory-utilization 0.90 \
    --max-model-len 32768 \
    --port 8000 \
    --host 127.0.0.1 \
    --enable-prefix-caching \
    --kv-cache-dtype auto \
    --trust-remote-code \
    --enable-chunked-prefill=true
