# avx512_compress

## Classs

- [Avx512Compressor](Avx512Compressor.md) — Vectorized AVX-512 Tensor / Weight Compression Engine.
- [CompressedBlockInt8](CompressedBlockInt8.md) — Compressed INT8 Block with block-wise FP32 scaling factor

## Functions

- [compress_block_avx512](compress_block_avx512.md) — Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)
- [compress_block_avx512](compress_block_avx512_1.md) — Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)
- [compress_block_fallback](compress_block_fallback.md) — Scalar / Portable fallback block compressor
- [compress_block_fallback](compress_block_fallback_1.md) — Scalar / Portable fallback block compressor
- [compress_fp32_to_int8](compress_fp32_to_int8.md) — Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.
- [compress_fp32_to_int8](compress_fp32_to_int8_1.md) — Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.
- [decompress_block_avx512](decompress_block_avx512.md) — Vectorized AVX-512 block decompression
- [decompress_block_avx512](decompress_block_avx512_1.md) — Vectorized AVX-512 block decompression
- [decompress_int8_to_fp32](decompress_int8_to_fp32.md) — Vectorized decompression of INT8 blocks back into FP32 buffer.
- [decompress_int8_to_fp32](decompress_int8_to_fp32_1.md) — Vectorized decompression of INT8 blocks back into FP32 buffer.
- [default](default.md)
- [default](default_1.md)
- [is_avx512_supported](is_avx512_supported.md) — Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.
- [is_avx512_supported](is_avx512_supported_1.md) — Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.
- [new](new.md)
- [new](new_1.md)
- [test_avx512_compression_roundtrip](test_avx512_compression_roundtrip.md) — [test]
