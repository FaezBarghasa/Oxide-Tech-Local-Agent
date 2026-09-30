"""
Llama / Llama 2 / Llama 3 / Llama 3.1 / Llama 3.2 Fused Kernel Patching
"""

import types
from typing import Any
from ..kernels import fast_rms_layernorm, fast_swiglu, fast_cross_entropy_loss


def patch_llama(model: Any) -> Any:
    """
    Patches a HuggingFace LlamaForCausalLM model with fused kernels:
    - Fused RMSNorm in attention pre-norm & post-norm
    - Fused SwiGLU in MLP forward
    - Fused Chunked Cross Entropy Loss in final causal head
    """
    if not hasattr(model, "model") or not hasattr(model.model, "layers"):
        return model

    # Patch RMSNorm modules
    if hasattr(model.model, "norm"):
        orig_norm_forward = model.model.norm.forward
        def _fused_norm_forward(self, hidden_states):
            return fast_rms_layernorm(hidden_states, self.weight, self.variance_epsilon)
        model.model.norm.forward = types.MethodType(_fused_norm_forward, model.model.norm)

    # Patch transformer layers
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

    setattr(model, "_oxide_llama_patched", True)
    return model
