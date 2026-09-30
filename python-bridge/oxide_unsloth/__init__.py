"""
Oxide Unsloth Compatibility Module
Drop-in replacement for `unsloth` v0.1.900-beta in Python workflows.
"""

from .fast_language_model import FastLanguageModel
from .models.vision import FastVisionModel
from .trainer import (
    OxideGRPOTrainer,
    OxideSFTTrainer,
    OxideDPOTrainer,
    OxideORPOTrainer,
    is_bfloat16_supported,
    unsloth_train,
    unsloth_save_model,
    unsloth_compile,
)
from .rewards import (
    rust_compiler_reward,
    memory_safety_reward,
    spice_simulation_reward,
    embedded_timing_reward,
    eda_drc_reward,
    math_reasoning_reward,
)
from .kernels import (
    fast_lora_forward,
    fast_cross_entropy_loss,
    fast_swiglu,
    fast_rms_layernorm,
    fast_rope_embedding,
    fast_geglu,
)
from .tokenizer_utils import (
    get_chat_template,
    standardize_sharegpt,
)

# Aliases for 100% Unsloth v0.1.900-beta API Parity
PatchFastLanguageModel = FastLanguageModel
PatchSFTTrainer = OxideSFTTrainer
PatchDPOTrainer = OxideDPOTrainer
PatchGRPOTrainer = OxideGRPOTrainer

__version__ = "0.1.900-beta"
__all__ = [
    # Models
    "FastLanguageModel",
    "FastVisionModel",
    "PatchFastLanguageModel",
    # Trainers
    "OxideGRPOTrainer",
    "OxideSFTTrainer",
    "OxideDPOTrainer",
    "OxideORPOTrainer",
    "PatchSFTTrainer",
    "PatchDPOTrainer",
    "PatchGRPOTrainer",
    "is_bfloat16_supported",
    "unsloth_train",
    "unsloth_save_model",
    "unsloth_compile",
    # Reward Verifiers
    "rust_compiler_reward",
    "memory_safety_reward",
    "spice_simulation_reward",
    "embedded_timing_reward",
    "eda_drc_reward",
    "math_reasoning_reward",
    # Fused Kernels
    "fast_lora_forward",
    "fast_cross_entropy_loss",
    "fast_swiglu",
    "fast_rms_layernorm",
    "fast_rope_embedding",
    "fast_geglu",
    # Tokenizer
    "get_chat_template",
    "standardize_sharegpt",
]
