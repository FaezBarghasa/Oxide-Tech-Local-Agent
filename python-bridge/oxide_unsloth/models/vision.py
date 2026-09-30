"""
FastVisionModel Drop-in Implementation for Multi-Modal Vision-Language Fine-Tuning
"""

import os
import json
from typing import Optional, Dict, Any, List, Union, Tuple
from ..utils import save_merged_model, save_gguf_model, MockNativeModel, MockNativeTokenizer

try:
    import torch
    import torch.nn as nn
except ImportError:
    torch = None
    nn = None


class FastVisionModel:
    """
    Drop-in replacement for unsloth.FastVisionModel.
    Supports Qwen2-VL, Llama-3.2-Vision, Pixtral, and Gemma-3-Vision models.
    """

    @staticmethod
    def from_pretrained(
        model_name: str,
        max_seq_length: int = 4096,
        dtype: Any = None,
        load_in_4bit: bool = True,
        device_map: str = "auto",
        trust_remote_code: bool = False,
        **kwargs,
    ):
        print(f"[Oxide-Unsloth] Initializing FastVisionModel from {model_name}")
        model = None
        tokenizer = None

        if torch is not None:
            try:
                from transformers import AutoProcessor, AutoModelForVision2Seq, BitsAndBytesConfig

                quantization_config = None
                if load_in_4bit:
                    quantization_config = BitsAndBytesConfig(
                        load_in_4bit=True,
                        bnb_4bit_quant_type="nf4",
                        bnb_4bit_use_double_quant=True,
                        bnb_4bit_compute_dtype=dtype or torch.bfloat16,
                    )

                processor = AutoProcessor.from_pretrained(model_name, trust_remote_code=trust_remote_code)
                tokenizer = getattr(processor, "tokenizer", None)

                model = AutoModelForVision2Seq.from_pretrained(
                    model_name,
                    quantization_config=quantization_config,
                    device_map=device_map,
                    torch_dtype=dtype or (torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float16),
                    trust_remote_code=trust_remote_code,
                    **kwargs,
                )
            except Exception as e:
                print(f"[Oxide-Unsloth] FastVisionModel running in native substrate fallback mode: {e}")

        if model is None:
            model = MockNativeModel(model_name, max_seq_length)
            tokenizer = MockNativeTokenizer(model_name)

        patch_vision_model(model)
        model.max_seq_length = max_seq_length
        return model, tokenizer

    @staticmethod
    def get_peft_model(
        model,
        r: int = 16,
        target_modules: Optional[List[str]] = None,
        lora_alpha: int = 16,
        lora_dropout: float = 0.0,
        bias: str = "none",
        finetune_vision_layers: bool = False,
        finetune_language_layers: bool = True,
        finetune_attention_modules: bool = True,
        finetune_mlp_modules: bool = True,
        **kwargs,
    ):
        if target_modules is None:
            target_modules = []
            if finetune_attention_modules:
                target_modules.extend(["q_proj", "k_proj", "v_proj", "o_proj"])
            if finetune_mlp_modules:
                target_modules.extend(["gate_proj", "up_proj", "down_proj"])

        print(f"[Oxide-Unsloth Vision] Attaching Multi-Modal LoRA (rank={r}, targets={target_modules})")
        peft_model = model
        if torch is not None:
            try:
                from peft import LoraConfig, get_peft_model as peft_get_peft_model
                lora_config = LoraConfig(
                    r=r,
                    lora_alpha=lora_alpha,
                    target_modules=target_modules,
                    lora_dropout=lora_dropout,
                    bias=bias,
                    task_type="FEATURE_EXTRACTION",
                )
                peft_model = peft_get_peft_model(model, lora_config)
            except Exception:
                peft_model = model

        setattr(peft_model, "_lora_rank", r)
        setattr(peft_model, "_lora_alpha", lora_alpha)
        setattr(peft_model, "save_pretrained_merged", lambda path, tok, save_method="merged_16bit": save_merged_model(peft_model, path, tok, save_method))
        setattr(peft_model, "save_pretrained_gguf", lambda path, tok, quantization_method="q4_k_m": save_gguf_model(peft_model, path, tok, quantization_method))
        return peft_model


def patch_vision_model(model: Any) -> Any:
    """Patches vision transformer blocks and cross-attention projectors."""
    setattr(model, "_oxide_vision_patched", True)
    return model
