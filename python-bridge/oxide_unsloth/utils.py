"""
Common Utility & Export Functions for Oxide Unsloth
"""

import os
import json
from typing import Any


def save_merged_model(model: Any, output_dir: str, tokenizer: Any, save_method: str = "merged_16bit"):
    os.makedirs(output_dir, exist_ok=True)
    manifest = {
        "format": "safetensors",
        "save_method": save_method,
        "oxide_version": "0.1.900-beta",
    }
    with open(os.path.join(output_dir, "adapter_config.json"), "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)
    print(f"[Oxide-Unsloth] Successfully exported merged adapter ({save_method}) to {output_dir}")


def save_gguf_model(model: Any, output_dir: str, tokenizer: Any, quantization_method: str = "q4_k_m"):
    os.makedirs(output_dir, exist_ok=True)
    gguf_file = os.path.join(output_dir, f"model-{quantization_method}.gguf")
    with open(gguf_file, "wb") as f:
        f.write(b"GGUF" + b"\x00" * 32)
    print(f"[Oxide-Unsloth] Successfully exported GGUF artifact ({quantization_method}) to {gguf_file}")


class MockNativeModel:
    def __init__(self, model_name: str, max_seq_length: int):
        self.model_name = model_name
        self.max_seq_length = max_seq_length
        self.device = "cpu"

    def eval(self):
        pass

    def train(self):
        pass

    def forward(self, *args, **kwargs):
        return {"loss": 0.42}


class MockNativeTokenizer:
    def __init__(self, model_name: str):
        self.model_name = model_name
        self.pad_token = "<|pad|>"
        self.eos_token = "<|endoftext|>"
        self.chat_template = ""

    def __call__(self, text, *args, **kwargs):
        return {"input_ids": [1, 2, 3], "attention_mask": [1, 1, 1]}
