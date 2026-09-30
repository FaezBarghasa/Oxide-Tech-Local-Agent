use oxide_kernels::{
    ChunkedCrossEntropyConfig, ChunkedCrossEntropyKernel, FusedSwiGLUKernel, LoRALinearKernel,
    QLoraNf4Dequant,
};

#[test]
fn test_fused_chunked_cross_entropy_online_reduction() {
    let config = ChunkedCrossEntropyConfig {
        chunk_size: 2,
        label_smoothing: 0.0,
        ignore_index: -100,
    };
    let kernel = ChunkedCrossEntropyKernel::new(config);

    let hidden_dim = 4;
    let vocab_size = 3;
    let num_tokens = 4;

    let hidden_states = vec![
        1.0, 0.5, -0.2, 0.8,
        -0.5, 1.2, 0.3, -0.1,
        0.2, -0.8, 1.5, 0.4,
        0.9, 0.1, -0.4, 0.6,
    ];

    let weights_lm_head = vec![
        0.1, 0.2, 0.3, 0.4, // vocab 0
        -0.2, 0.5, -0.1, 0.3, // vocab 1
        0.4, -0.3, 0.2, 0.1, // vocab 2
    ];

    let targets = vec![0i64, 1i64, 2i64, 0i64];

    let (loss, grad_hidden) = kernel
        .compute_loss(&hidden_states, &weights_lm_head, &targets, hidden_dim, vocab_size)
        .expect("Chunked cross entropy compute failed");

    assert!(loss > 0.0, "Loss must be positive");
    assert_eq!(grad_hidden.len(), num_tokens * hidden_dim);
    for g in grad_hidden {
        assert!(!g.is_nan(), "Gradient contains NaN");
        assert!(!g.is_infinite(), "Gradient contains Inf");
    }
}

#[test]
fn test_fused_swiglu_forward_and_backward() {
    let gate = vec![1.0, -1.0, 2.0, 0.0];
    let up = vec![0.5, 2.0, -0.5, 1.0];
    let mut out = vec![0.0; 4];

    FusedSwiGLUKernel::forward(&gate, &up, &mut out);

    assert!(out[0] > 0.0);
    assert!(out[1] < 0.0);

    let d_out = vec![1.0, 1.0, 1.0, 1.0];
    let mut d_gate = vec![0.0; 4];
    let mut d_up = vec![0.0; 4];

    FusedSwiGLUKernel::backward(&gate, &up, &d_out, &mut d_gate, &mut d_up);

    for g in d_gate {
        assert!(!g.is_nan());
    }
    for u in d_up {
        assert!(!u.is_nan());
    }
}

#[test]
fn test_fused_lora_nf4_dequant_and_linear_forward_backward() {
    let (v0, v1) = QLoraNf4Dequant::dequantize_byte(0x10, 2.0);
    assert!(!v0.is_nan());
    assert!(!v1.is_nan());

    let in_features = 4;
    let out_features = 2;
    let rank = 2;
    let alpha = 16.0;
    let num_tokens = 2;

    let kernel = LoRALinearKernel::new(in_features, out_features, rank, alpha);

    let x = vec![1.0, 2.0, 0.5, -1.0, 0.2, -0.4, 1.1, 0.8];
    let base_w = vec![0.1; out_features * in_features];
    let lora_a = vec![0.05; rank * in_features];
    let lora_b = vec![0.02; out_features * rank];

    let mut out = vec![0.0; num_tokens * out_features];
    kernel.forward(&x, &base_w, &lora_a, &lora_b, num_tokens, &mut out);

    assert_eq!(out.len(), num_tokens * out_features);

    let d_out = vec![1.0; num_tokens * out_features];
    let mut d_lora_a = vec![0.0; rank * in_features];
    let mut d_lora_b = vec![0.0; out_features * rank];
    let mut d_x = vec![0.0; num_tokens * in_features];

    kernel.backward(
        &x,
        &lora_a,
        &lora_b,
        &d_out,
        num_tokens,
        &mut d_lora_a,
        &mut d_lora_b,
        &mut d_x,
    );

    for da in d_lora_a {
        assert!(!da.is_nan());
    }
    for db in d_lora_b {
        assert!(!db.is_nan());
    }
    for dx in d_x {
        assert!(!dx.is_nan());
    }
}
