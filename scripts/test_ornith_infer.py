#!/usr/bin/env python3
import os
import time
from llama_cpp import Llama

model_path = os.path.expanduser("~/models/ornith-1.0-9b-Q4_K_M.gguf")
print(f"[+] Loading Ornith-1.0-9B from {model_path} (35 GPU layers offloaded)...")

start_load = time.time()
llm = Llama(
    model_path=model_path,
    n_gpu_layers=35,
    n_ctx=2048,
    verbose=True,
)
print(f"[+] Model loaded in {time.time() - start_load:.2f}s!")

prompt = "<|im_start|>system\nYou are the Oxide-Tech Embedded Firmware and EDA Architecture Agent.<|im_end|>\n<|im_start|>user\nProvide a high-performance DMA SPI transfer function for STM32 in Rust no_std.<|im_end|>\n<|im_start|>assistant\n"

print("[+] Generating response from Ornith-1.0-9B...")
start_infer = time.time()
output = llm(prompt, max_tokens=150, temperature=0.1)
elapsed = time.time() - start_infer

text = output["choices"][0]["text"]
tokens = output["usage"]["completion_tokens"]
tps = tokens / elapsed if elapsed > 0 else 0

print("\n" + "="*50)
print(f"GENERATION RESULT ({tokens} tokens in {elapsed:.2f}s -> {tps:.1f} tok/s):")
print("="*50)
print(text)
print("="*50)
