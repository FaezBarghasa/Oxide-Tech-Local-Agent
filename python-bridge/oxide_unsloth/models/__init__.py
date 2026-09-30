"""
Model-specific Monkey Patching & Optimization Modules
"""

from .llama import patch_llama
from .qwen2 import patch_qwen2
from .mistral import patch_mistral
from .gemma import patch_gemma
from .vision import FastVisionModel, patch_vision_model

__all__ = [
    "patch_llama",
    "patch_qwen2",
    "patch_mistral",
    "patch_gemma",
    "FastVisionModel",
    "patch_vision_model",
]
