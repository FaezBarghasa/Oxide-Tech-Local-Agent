# mmap_tensor

## Classs

- [AlignedTensorMap](AlignedTensorMap.md) — Safe aligned tensor memory mapping with strict alignment assertion and advisory reader file-locking.
- [GgufTensorInfo](GgufTensorInfo.md) — Metadata describing an individual tensor inside an IMatrix / GGUF model file.
- [GgufTensorType](GgufTensorType.md) — GGUF tensor quantization formats including mixed-precision Importance Matrix (IMatrix) types.
- [HardenedTensorMap](HardenedTensorMap.md) — Hardened tensor memory mapping satisfying AVX-512 and ARM NEON cacheline alignment invariants (>= 64 bytes).
- [MemoryAdvice](MemoryAdvice.md) — Kernel memory access pattern advice for memory-mapped model files.
- [MmapModel](MmapModel.md) — Zero-copy memory-mapped model container.
- [TensorSlice](TensorSlice.md) — A zero-copy slice of memory-mapped model weights tied to the underlying `Mmap` lifetime.

## Functions

- [advise](advise.md) — Advise the Linux kernel on paging strategies for this model region.
- [advise](advise_1.md) — Advise the Linux kernel on paging strategies for this model region.
- [as_ptr](as_ptr.md) — Return the raw pointer to tensor data for FFI and CUDA kernels.
- [as_ptr](as_ptr_1.md) — Return the raw pointer to tensor data for FFI and CUDA kernels.
- [as_slice](as_slice.md) — Read data as a standard byte slice.
- [as_slice](as_slice_1.md) — Read data as a standard byte slice.
- [as_slice](as_slice_2.md) — Access mapped data as an immutable f32 slice.
- [as_slice](as_slice_3.md) — Access mapped data as an immutable f32 slice.
- [as_slice](as_slice_4.md) — [inline(always)]
- [as_slice](as_slice_5.md) — [inline(always)]
- [from_file](from_file.md) — Memory map a model file from disk into virtual address space without copying.
- [from_file](from_file_1.md) — Memory map a model file from disk into virtual address space without copying.
- [get_tensor_slice](get_tensor_slice.md) — Extract a zero-copy tensor slice with strict boundary and alignment checks.
- [get_tensor_slice](get_tensor_slice_1.md) — Extract a zero-copy tensor slice with strict boundary and alignment checks.
- [is_empty](is_empty.md) — Check if mapped region is empty.
- [is_empty](is_empty_1.md) — Check if mapped region is empty.
- [is_empty](is_empty_2.md) — Check if slice is empty.
- [is_empty](is_empty_3.md) — Check if slice is empty.
- [is_empty](is_empty_4.md) — Check if slice is empty.
- [is_empty](is_empty_5.md) — Check if slice is empty.
- [is_empty](is_empty_6.md)
- [is_empty](is_empty_7.md)
- [len](len.md) — Total mapped byte length.
- [len](len_1.md) — Total mapped byte length.
- [len](len_2.md) — Byte length of the tensor slice.
- [len](len_3.md) — Byte length of the tensor slice.
- [len](len_4.md) — Number of f32 elements.
- [len](len_5.md) — Number of f32 elements.
- [len](len_6.md)
- [len](len_7.md)
- [load_aligned](load_aligned.md) — Load generic typed tensor slice enforcing >= 64-byte SIMD alignment and shared advisory read lock.
- [load_aligned](load_aligned_1.md) — Load generic typed tensor slice enforcing >= 64-byte SIMD alignment and shared advisory read lock.
- [load_f32](load_f32.md) — Load f32 tensor slice from disk with advisory reader lock and SIMD alignment verification.
- [load_f32](load_f32_1.md) — Load f32 tensor slice from disk with advisory reader lock and SIMD alignment verification.
- [test_aligned_tensor_map_f32](test_aligned_tensor_map_f32.md) — [test]
- [test_mmap_tensor_lifecycle](test_mmap_tensor_lifecycle.md) — [test]
