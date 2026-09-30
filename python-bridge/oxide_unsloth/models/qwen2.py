"""
Qwen2 / Qwen2.5 / Qwen2.5-Coder Fused Kernel Patching
"""

import types
from typing import Any
from ..kernels import fast_rms_layernorm, fast_swiglu


def patch_qwen2(model: Any) -> Any:
    """
    Patches a HuggingFace Qwen2ForCausalLM model with fused kernels.
    """
    if not hasattr(model, "model") or not hasattr(model.model, "layers"):
        return model

    # Patch RMSNorm
    if hasattr(model.model, "norm"):
        def _fused_norm_forward(self, hidden_states):
            return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
        model.model.norm.forward = types.MethodType(_fused_norm_forward, model.model.norm)

    # Patch layers
    for layer in model.model.layers:
        if hasattr(layer, "input_layernorm"):
            def _fused_input_norm(self, hidden_states):
                return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
            layer.input_layernorm.forward = types.MethodType(_fused_input_norm, layer.input_layernorm)

        if hasattr(layer, "post_attention_layernorm"):
            def _fused_post_norm(self, hidden_states):
                return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
            layer.post_attention_layernorm.forward = types.MethodType(_fused_post_norm, layer.post_attention_layernorm)

        if hasattr(layer, "mlp"):
            mlp = layer.mlp
            def _fused_mlp_forward(self, x):
                gate = self.gate_proj(x)
                up = self.up_proj(x)
                inter = fast_swiglu(gate, up)
                return self.down_proj(inter)
            mlp.forward = types.MethodType(_fused_mlp_forward, mlp)

    setattr(model, "_oxide_qwen2_patched", True)
    return model
