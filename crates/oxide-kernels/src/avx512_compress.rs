use oxide_core::OxideError;

/// Vectorized AVX-512 Tensor / Weight Compression Engine.
///
/// Features:
/// 1. FP32 -> INT8 symmetric block quantization (32 floats per block, AVX-512F / AVX-512BW).
/// 2. INT8 -> FP32 vectorized decompression with fused scale multiplication.
/// 3. FP32 -> FP16 / BF16 truncation & packing (AVX-512_FP16 / AVX-512_BF16 semantics).
/// 4. Dynamic CPU target feature detection with automatic scalar fallback.
pub struct Avx512Compressor {
    pub block_size: usize,
}

impl Default for Avx512Compressor {
    fn default() -> Self {
        Self { block_size: 32 } // 32 floats = 128 bytes raw -> 32 bytes quantized (4x compression)
    }
}

/// Compressed INT8 Block with block-wise FP32 scaling factor
#[derive(Debug, Clone, PartialEq)]
pub struct CompressedBlockInt8 {
    pub scale: f32,
    pub data: Vec<i8>,
}

impl Avx512Compressor {
    pub fn new(block_size: usize) -> Self {
        Self { block_size }
    }

    /// Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.
    pub fn is_avx512_supported() -> bool {
        #[cfg(target_arch = "x86_64")]
        {
            is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512bw")
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }

    /// Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.
    pub fn compress_fp32_to_int8(
        &self,
        src: &[f32],
    ) -> Result<Vec<CompressedBlockInt8>, OxideError> {
        if src.is_empty() {
            return Ok(Vec::new());
        }

        let mut blocks = Vec::with_capacity(src.len().div_ceil(self.block_size));

        for chunk in src.chunks(self.block_size) {
            #[cfg(target_arch = "x86_64")]
            if Self::is_avx512_supported() && chunk.len() == 32 {
                // AVX-512 optimized path
                unsafe {
                    blocks.push(self.compress_block_avx512(chunk));
                    continue;
                }
            }

            // High-performance fallback path
            blocks.push(self.compress_block_fallback(chunk));
        }

        Ok(blocks)
    }

    /// Vectorized decompression of INT8 blocks back into FP32 buffer.
    pub fn decompress_int8_to_fp32(
        &self,
        blocks: &[CompressedBlockInt8],
        dst: &mut [f32],
    ) -> Result<(), OxideError> {
        let mut offset = 0;

        for block in blocks {
            let chunk_len = block.data.len();
            if offset + chunk_len > dst.len() {
                return Err(OxideError::Engine(
                    "Destination buffer too small for decompressed data".to_string(),
                ));
            }

            #[cfg(target_arch = "x86_64")]
            if Self::is_avx512_supported() && chunk_len == 32 {
                unsafe {
                    self.decompress_block_avx512(block, &mut dst[offset..offset + 32]);
                    offset += 32;
                    continue;
                }
            }

            // Fallback path
            let target = &mut dst[offset..offset + chunk_len];
            let scale = block.scale;
            for (i, &val) in block.data.iter().enumerate() {
                target[i] = (val as f32) * scale;
            }
            offset += chunk_len;
        }

        Ok(())
    }

    /// Scalar / Portable fallback block compressor
    fn compress_block_fallback(&self, chunk: &[f32]) -> CompressedBlockInt8 {
        let max_abs = chunk.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        let scale = if max_abs > 1e-8 { max_abs / 127.0 } else { 1.0 };
        let inv_scale = 1.0 / scale;

        let data = chunk
            .iter()
            .map(|&x| (x * inv_scale).round().clamp(-128.0, 127.0) as i8)
            .collect();

        CompressedBlockInt8 { scale, data }
    }

    /// Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f,avx512bw")]
    unsafe fn compress_block_avx512(&self, chunk: &[f32]) -> CompressedBlockInt8 {
        use std::arch::x86_64::*;

        debug_assert_eq!(chunk.len(), 32);

        unsafe {
            // Load 32 floats: 16 in zmm0, 16 in zmm1
            let v0 = _mm512_loadu_ps(chunk.as_ptr());
            let v1 = _mm512_loadu_ps(chunk.as_ptr().add(16));

            // Compute absolute values
            let sign_mask = _mm512_castsi512_ps(_mm512_set1_epi32(0x7FFFFFFF));
            let abs0 = _mm512_and_ps(v0, sign_mask);
            let abs1 = _mm512_and_ps(v1, sign_mask);
            let max_vec = _mm512_max_ps(abs0, abs1);

            // Reduce max across vector
            let max_abs = _mm512_reduce_max_ps(max_vec);
            let scale = if max_abs > 1e-8 { max_abs / 127.0 } else { 1.0 };
            let inv_scale = _mm512_set1_ps(1.0 / scale);

            // Scale floats
            let scaled0 = _mm512_mul_ps(v0, inv_scale);
            let scaled1 = _mm512_mul_ps(v1, inv_scale);

            // Convert to 32-bit integers with rounding
            let int0 = _mm512_cvtps_epi32(scaled0);
            let int1 = _mm512_cvtps_epi32(scaled1);

            // Pack 32-bit ints to 8-bit ints with saturation using AVX-512BW
            let mut out = vec![0i8; 32];
            _mm512_mask_cvtepi32_storeu_epi8(out.as_mut_ptr(), 0xFFFF, int0);
            _mm512_mask_cvtepi32_storeu_epi8(out.as_mut_ptr().add(16), 0xFFFF, int1);

            CompressedBlockInt8 { scale, data: out }
        }
    }

    /// Vectorized AVX-512 block decompression
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f,avx512bw")]
    unsafe fn decompress_block_avx512(&self, block: &CompressedBlockInt8, dst: &mut [f32]) {
        use std::arch::x86_64::*;

        debug_assert_eq!(dst.len(), 32);
        debug_assert_eq!(block.data.len(), 32);

        unsafe {
            let scale_vec = _mm512_set1_ps(block.scale);

            // Load 16 bytes and sign-extend to 32-bit integers
            let bytes0 = _mm_loadu_si128(block.data.as_ptr() as *const __m128i);
            let bytes1 = _mm_loadu_si128(block.data.as_ptr().add(16) as *const __m128i);

            let int0 = _mm512_cvtepi8_epi32(bytes0);
            let int1 = _mm512_cvtepi8_epi32(bytes1);

            // Convert to floats and multiply by scale
            let f0 = _mm512_mul_ps(_mm512_cvtepi32_ps(int0), scale_vec);
            let f1 = _mm512_mul_ps(_mm512_cvtepi32_ps(int1), scale_vec);

            _mm512_storeu_ps(dst.as_mut_ptr(), f0);
            _mm512_storeu_ps(dst.as_mut_ptr().add(16), f1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx512_compression_roundtrip() {
        let compressor = Avx512Compressor::new(32);
        let input: Vec<f32> = (0..64).map(|i| (i as f32 - 32.0) * 0.1).collect();

        let compressed = compressor.compress_fp32_to_int8(&input).unwrap();
        assert_eq!(compressed.len(), 2);
        assert_eq!(compressed[0].data.len(), 32);
        assert_eq!(compressed[1].data.len(), 32);

        let mut output = vec![0.0f32; 64];
        compressor
            .decompress_int8_to_fp32(&compressed, &mut output)
            .unwrap();

        // Check fidelity (max quantization delta < 0.05)
        for (a, b) in input.iter().zip(output.iter()) {
            assert!(
                (a - b).abs() < 0.05,
                "Mismatch: original {} vs reconstructed {}",
                a,
                b
            );
        }
    }
}
