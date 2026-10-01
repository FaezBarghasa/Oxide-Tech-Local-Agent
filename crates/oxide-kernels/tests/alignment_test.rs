use oxide_kernels::ternary::load_simd_512_guarded;

#[test]
fn test_simd_alignment_offsets_0_to_63() {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            // Allocate 128 bytes to test offsets 0 through 63
            let mut buffer = vec![0u8; 128];
            for i in 0..128 {
                buffer[i] = (i % 255) as u8;
            }

            for offset in 0..64 {
                unsafe {
                    let ptr = buffer.as_ptr().add(offset);
                    let _val = load_simd_512_guarded(ptr);
                    // Successfully loaded without #GP hardware trap
                }
            }
        }
    }
}
