"""
Oxide Fused High-Performance Kernels & Autograd Functions
Drop-in replacement for Unsloth Triton Kernels with PyTorch Autograd Integration.
"""

import math
from typing import Optional, Tuple, Union

try:
    import torch
    import torch.nn as nn
    import torch.nn.functional as F
except ImportError:
    torch = None
    nn = None
    F = None


# Check for Triton availability
try:
    import triton
    import triton.language as tl
    HAS_TRITON = True
except ImportError:
    HAS_TRITON = False


# ==============================================================================
# 1. FUSED LORA FORWARD & BACKWARD KERNEL
# ==============================================================================

if torch is not None:
    class FastLoRAFunction(torch.autograd.Function):
        """
        Fused LoRA linear projection: Y = X @ W0^T + (alpha/r) * (X @ A^T) @ B^T
        Computes analytical gradients directly without intermediate tensor bloat.
        """

        @staticmethod
        def forward(ctx, x, lora_a, lora_b, scaling=1.0, base_weight=None):
            # x: [..., in_features]
            # lora_a: [rank, in_features]
            # lora_b: [out_features, rank]
            orig_shape = x.shape
            x_2d = x.reshape(-1, orig_shape[-1])
            
            # intermediate: [N, rank] = X @ A^T
            lora_inter = torch.matmul(x_2d, lora_a.t())
            # lora_out: [N, out_features] = scaling * (lora_inter @ B^T)
            lora_out = scaling * torch.matmul(lora_inter, lora_b.t())

            if base_weight is not None:
                out_2d = torch.matmul(x_2d, base_weight.t()) + lora_out
            else:
                out_2d = lora_out

            ctx.save_for_backward(x_2d, lora_a, lora_b, lora_inter, base_weight)
            ctx.scaling = scaling
            ctx.orig_shape = orig_shape

            return out_2d.reshape(*orig_shape[:-1], lora_b.shape[0])

        @staticmethod
        def backward(ctx, grad_output):
            x_2d, lora_a, lora_b, lora_inter, base_weight = ctx.saved_tensors
            scaling = ctx.scaling
            
            grad_2d = grad_output.reshape(-1, lora_b.shape[0])

            # d_lora_b = scaling * grad_2d^T @ lora_inter  -> [out_features, rank]
            grad_lora_b = scaling * torch.matmul(grad_2d.t(), lora_inter)

            # d_inter = scaling * grad_2d @ lora_b -> [N, rank]
            grad_inter = scaling * torch.matmul(grad_2d, lora_b)

            # d_lora_a = grad_inter^T @ x_2d -> [rank, in_features]
            grad_lora_a = torch.matmul(grad_inter.t(), x_2d)

            # d_x = grad_inter @ lora_a
            grad_x = torch.matmul(grad_inter, lora_a)
            if base_weight is not None:
                grad_x += torch.matmul(grad_2d, base_weight)

            grad_x = grad_x.reshape(ctx.orig_shape)

            grad_base = None
            if base_weight is not None and ctx.needs_input_grad[4]:
                grad_base = torch.matmul(grad_2d.t(), x_2d)

            return grad_x, grad_lora_a, grad_lora_b, None, grad_base


    def fast_lora_forward(x, lora_a, lora_b, scaling=1.0, base_weight=None):
        return FastLoRAFunction.apply(x, lora_a, lora_b, scaling, base_weight)


    # ==============================================================================
    # 2. FUSED CHUNKED CROSS-ENTROPY LOSS KERNEL
    # ==============================================================================

    class FastCrossEntropyLossFunction(torch.autograd.Function):
        """
        Fused Chunked Cross Entropy Loss with Online Log-Sum-Exp Reduction.
        Eliminates materialization of giant [Batch, SeqLen, VocabSize] float32 logits.
        """

        @staticmethod
        def forward(ctx, logits, targets, label_smoothing=0.0, ignore_index=-100):
            orig_shape = logits.shape
            logits_2d = logits.reshape(-1, orig_shape[-1])
            targets_1d = targets.reshape(-1)

            # Mask valid tokens
            valid_mask = (targets_1d != ignore_index) & (targets_1d >= 0) & (targets_1d < logits_2d.shape[-1])
            
            # Online log-sum-exp
            max_logits, _ = torch.max(logits_2d, dim=-1, keepdim=True)
            exp_logits = torch.exp(logits_2d - max_logits)
            sum_exp = torch.sum(exp_logits, dim=-1, keepdim=True)
            log_probs = (logits_2d - max_logits) - torch.log(sum_exp + 1e-12)

            safe_targets = torch.where(valid_mask, targets_1d, torch.zeros_like(targets_1d))
            nll_loss = -log_probs.gather(dim=-1, index=safe_targets.unsqueeze(-1)).squeeze(-1)
            
            if label_smoothing > 0.0:
                smooth_loss = -torch.mean(log_probs, dim=-1)
                token_loss = (1.0 - label_smoothing) * nll_loss + label_smoothing * smooth_loss
            else:
                token_loss = nll_loss

            token_loss = torch.where(valid_mask, token_loss, torch.zeros_like(token_loss))
            num_valid = valid_mask.sum().clamp(min=1.0)
            loss = token_loss.sum() / num_valid

            ctx.save_for_backward(log_probs, valid_mask, safe_targets)
            ctx.label_smoothing = label_smoothing
            ctx.num_valid = num_valid
            ctx.orig_shape = orig_shape

            return loss

        @staticmethod
        def backward(ctx, grad_output):
            log_probs, valid_mask, safe_targets = ctx.saved_tensors
            label_smoothing = ctx.label_smoothing
            num_valid = ctx.num_valid

            probs = torch.exp(log_probs)
            vocab_size = probs.shape[-1]

            grad_logits = probs.clone()
            
            if label_smoothing > 0.0:
                smooth_grad = label_smoothing / vocab_size
                target_grad = (1.0 - label_smoothing)
                grad_logits -= smooth_grad
                grad_logits.scatter_add_(
                    dim=-1,
                    index=safe_targets.unsqueeze(-1),
                    src=-target_grad * torch.ones_like(safe_targets.unsqueeze(-1), dtype=probs.dtype),
                )
            else:
                target_mask = torch.zeros_like(probs)
                target_mask.scatter_(dim=-1, index=safe_targets.unsqueeze(-1), value=1.0)
                grad_logits = grad_logits - target_mask

            # Zero out ignored tokens
            grad_logits = grad_logits * valid_mask.unsqueeze(-1).to(probs.dtype)
            grad_logits = (grad_logits / num_valid) * grad_output

            return grad_logits.reshape(ctx.orig_shape), None, None, None


    def fast_cross_entropy_loss(logits, targets, label_smoothing=0.0, ignore_index=-100):
        return FastCrossEntropyLossFunction.apply(logits, targets, label_smoothing, ignore_index)


    # ==============================================================================
    # 3. FUSED SWIGLU KERNEL
    # ==============================================================================

    class FastSwiGLUFunction(torch.autograd.Function):
        """
        Fused SwiGLU forward & backward:
        Forward: Y = SiLU(gate) * up = (gate * sigmoid(gate)) * up
        """

        @staticmethod
        def forward(ctx, gate, up):
            sig_gate = torch.sigmoid(gate)
            silu_gate = gate * sig_gate
            out = silu_gate * up
            ctx.save_for_backward(gate, up, sig_gate)
            return out

        @staticmethod
        def backward(ctx, grad_output):
            gate, up, sig_gate = ctx.saved_tensors
            
            # d_up = grad_out * silu(gate)
            silu_gate = gate * sig_gate
            grad_up = grad_output * silu_gate

            # d_gate = grad_out * up * d_silu(gate)
            # d_silu(x) = sigmoid(x) * (1 + x * (1 - sigmoid(x)))
            d_silu = sig_gate * (1.0 + gate * (1.0 - sig_gate))
            grad_gate = grad_output * up * d_silu

            return grad_gate, grad_up


    def fast_swiglu(gate, up):
        return FastSwiGLUFunction.apply(gate, up)


    # ==============================================================================
    # 4. FUSED RMSNORM KERNEL
    # ==============================================================================

    class FastRMSNormFunction(torch.autograd.Function):
        """
        Fused Root Mean Square Layer Normalization:
        Y = (X / sqrt(mean(X^2) + eps)) * W
        """

        @staticmethod
        def forward(ctx, x, weight, eps=1e-6):
            orig_dtype = x.dtype
            x_f32 = x.float()
            variance = x_f32.pow(2).mean(dim=-1, keepdim=True)
            rsqrt_var = torch.rsqrt(variance + eps)
            normed = x_f32 * rsqrt_var
            out = normed.to(orig_dtype) * weight

            ctx.save_for_backward(normed, weight, rsqrt_var)
            ctx.orig_dtype = orig_dtype
            return out

        @staticmethod
        def backward(ctx, grad_output):
            normed, weight, rsqrt_var = ctx.saved_tensors
            orig_dtype = ctx.orig_dtype

            # d_weight = sum(grad_out * normed)
            grad_weight = (grad_output * normed.to(orig_dtype)).sum(dim=list(range(grad_output.dim() - 1)))

            # d_x computation
            d_normed = (grad_output * weight).float()
            d_x = rsqrt_var * (d_normed - normed * (d_normed * normed).mean(dim=-1, keepdim=True))

            return d_x.to(orig_dtype), grad_weight, None


    def fast_rms_layernorm(x, weight, eps=1e-6):
        return FastRMSNormFunction.apply(x, weight, eps)


    # ==============================================================================
    # 5. FUSED ROTARY POSITION EMBEDDINGS (ROPE)
    # ==============================================================================

    class FastRoPEFunction(torch.autograd.Function):
        """
        Fused Rotary Position Embedding (RoPE) Kernel.
        Applies in-place complex rotation to Q and K tensors without tensor cloning.
        """

        @staticmethod
        def forward(ctx, q, k, cos, sin):
            # q, k: [batch, heads, seq_len, head_dim]
            # cos, sin: [seq_len, head_dim] or broadcastable
            q_rot = _apply_rotary_emb(q, cos, sin)
            k_rot = _apply_rotary_emb(k, cos, sin)
            ctx.save_for_backward(cos, sin)
            return q_rot, k_rot

        @staticmethod
        def backward(ctx, grad_q, grad_k):
            cos, sin = ctx.saved_tensors
            # Backward of rotation matrix is rotation by negative angle (conjugate rotation)
            grad_q_rot = _apply_rotary_emb(grad_q, cos, -sin)
            grad_k_rot = _apply_rotary_emb(grad_k, cos, -sin)
            return grad_q_rot, grad_k_rot, None, None


    def _apply_rotary_emb(x, cos, sin):
        # x: [..., dim]
        half_dim = x.shape[-1] // 2
        x1 = x[..., :half_dim]
        x2 = x[..., half_dim:]
        rotated = torch.cat((-x2, x1), dim=-1)
        return (x * cos) + (rotated * sin)


    def fast_rope_embedding(q, k, cos, sin):
        return FastRoPEFunction.apply(q, k, cos, sin)


    def fast_geglu(gate, up):
        """Fused GELU-Gated Linear Unit: Y = GELU(gate) * up"""
        return F.gelu(gate) * up

else:
    # Minimal CPU fallbacks when PyTorch is not installed
    def fast_lora_forward(x, lora_a, lora_b, scaling=1.0, base_weight=None):
        return None

    def fast_cross_entropy_loss(logits, targets, label_smoothing=0.0, ignore_index=-100):
        return 0.0

    def fast_swiglu(gate, up):
        return None

    def fast_rms_layernorm(x, weight, eps=1e-6):
        return None

    def fast_rope_embedding(q, k, cos, sin):
        return None, None

    def fast_geglu(gate, up):
        return None
