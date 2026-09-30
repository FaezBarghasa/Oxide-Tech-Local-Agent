"""
Unit Tests for Fused PyTorch Autograd Kernels
Validates numerical consistency between Oxide fused kernels and reference PyTorch implementations.
"""

import pytest
import math

try:
    import torch
    import torch.nn as nn
    import torch.nn.functional as F
    HAS_TORCH = True
except ImportError:
    HAS_TORCH = False

from oxide_unsloth.kernels import (
    fast_lora_forward,
    fast_cross_entropy_loss,
    fast_swiglu,
    fast_rms_layernorm,
    fast_rope_embedding,
    fast_geglu,
)


@pytest.mark.skipif(not HAS_TORCH, reason="PyTorch not available")
class TestFusedKernels:
    def test_fast_lora_forward_and_backward(self):
        batch, seq_len, in_f, out_f, rank = 2, 4, 32, 64, 8
        scaling = 2.0

        x = torch.randn(batch, seq_len, in_f, requires_grad=True)
        lora_a = torch.randn(rank, in_f, requires_grad=True)
        lora_b = torch.randn(out_f, rank, requires_grad=True)

        # Fused forward
        y_fused = fast_lora_forward(x, lora_a, lora_b, scaling=scaling)
        assert y_fused.shape == (batch, seq_len, out_f)

        # Reference forward
        x_2d = x.reshape(-1, in_f)
        inter = torch.matmul(x_2d, lora_a.t())
        y_ref = (scaling * torch.matmul(inter, lora_b.t())).reshape(batch, seq_len, out_f)

        assert torch.allclose(y_fused, y_ref, atol=1e-5)

        # Backward test
        grad_out = torch.randn_like(y_fused)
        y_fused.backward(grad_out)

        assert x.grad is not None
        assert lora_a.grad is not None
        assert lora_b.grad is not None

    def test_fast_cross_entropy_loss(self):
        batch_seq, vocab = 16, 128
        logits = torch.randn(batch_seq, vocab, requires_grad=True)
        targets = torch.randint(0, vocab, (batch_seq,))

        fused_loss = fast_cross_entropy_loss(logits, targets)
        ref_loss = F.cross_entropy(logits, targets)

        assert torch.allclose(fused_loss, ref_loss, atol=1e-5)

        # Backward check
        fused_loss.backward()
        assert logits.grad is not None

    def test_fast_swiglu(self):
        gate = torch.randn(4, 16, requires_grad=True)
        up = torch.randn(4, 16, requires_grad=True)

        out_fused = fast_swiglu(gate, up)
        out_ref = F.silu(gate) * up

        assert torch.allclose(out_fused, out_ref, atol=1e-5)

        out_fused.sum().backward()
        assert gate.grad is not None
        assert up.grad is not None

    def test_fast_rms_layernorm(self):
        x = torch.randn(2, 8, 32, requires_grad=True)
        weight = torch.ones(32, requires_grad=True)

        out_fused = fast_rms_layernorm(x, weight, eps=1e-6)
        
        # Reference RMSNorm
        var = x.pow(2).mean(dim=-1, keepdim=True)
        out_ref = x * torch.rsqrt(var + 1e-6) * weight

        assert torch.allclose(out_fused, out_ref, atol=1e-5)

        out_fused.sum().backward()
        assert x.grad is not None
        assert weight.grad is not None

    def test_fast_rope_embedding(self):
        b, h, s, d = 2, 4, 8, 16
        q = torch.randn(b, h, s, d, requires_grad=True)
        k = torch.randn(b, h, s, d, requires_grad=True)
        cos = torch.ones(s, d)
        sin = torch.zeros(s, d)

        # With cos=1, sin=0, rotation should be exact identity
        q_rot, k_rot = fast_rope_embedding(q, k, cos, sin)
        assert torch.allclose(q_rot, q, atol=1e-5)
        assert torch.allclose(k_rot, k, atol=1e-5)

    def test_fast_geglu(self):
        gate = torch.randn(2, 10)
        up = torch.randn(2, 10)
        out = fast_geglu(gate, up)
        assert out.shape == (2, 10)
