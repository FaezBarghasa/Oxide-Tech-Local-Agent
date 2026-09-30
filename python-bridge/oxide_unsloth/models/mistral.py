"""
Mistral / Mixtral Fused Kernel Patching
"""

import types
from typing import Any
from ..kernels import fast_rms_layernorm, fast_swiglu


def patch_mistral(model: Any) -> Any:
    """Patches MistralForCausalLM models with fused kernels."""
    if not hasattr(model, "model") or not hasattr(model.model, "layers"):
        return model

    if hasattr(model.model, "norm"):
        def _fused_norm(self, hidden_states):
            return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
        model.model.norm.forward = types.MethodType(_fused_norm, model.model.norm)

    for layer in model.model.layers:
        if hasattr(layer, "input_layernorm"):
            def _fused_in_norm(self, hidden_states):
                return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
            layer.input_layernorm.forward = types.MethodType(_fused_in_norm, layer.input_layernorm)

        if hasattr(layer, "post_attention_layernorm"):
            def _fused_out_norm(self, hidden_states):
                return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
            layer.post_attention_layernorm.forward = types.MethodType(_fused_out_norm, layer.post_attention_layernorm)

        if hasattr(layer, "mlp") and hasattr(layer.mlp, "gate_proj"):
            mlp = layer.mlp
            def _fused_mlp(self, x):
                gate = self.gate_proj(x)
                up = self.up_proj(x)
                return self.down_proj(fast_swiglu(gate, up))
            mlp.forward = types.MethodType(_fused_mlp, mlp)

    setattr(model, "_oxide_mistral_patched", True)
    return model
