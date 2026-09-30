"""
Oxide Unsloth Compatibility Module
Drop-in replacement for `unsloth` v0.1.900-beta in Python workflows.
"""

from .fast_language_model import FastLanguageModel
from .trainer import OxideGRPOTrainer, OxideSFTTrainer, is_bfloat16_supported, unsloth_train
from .rewards import rust_compiler_reward, memory_safety_reward, spice_simulation_reward

__version__ = "0.1.900-beta"
__all__ = [
    "FastLanguageModel",
    "OxideGRPOTrainer",
    "OxideSFTTrainer",
    "is_bfloat16_supported",
    "unsloth_train",
    "rust_compiler_reward",
    "memory_safety_reward",
    "spice_simulation_reward",
]
