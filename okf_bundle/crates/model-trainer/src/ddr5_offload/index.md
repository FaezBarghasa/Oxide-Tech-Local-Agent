# ddr5_offload

## Classs

- [Ddr5TierManager](Ddr5TierManager.md) — Dynamic DDR5 RAM Tier Manager that automatically offloads layers, optimizer states,
- [MemoryTier](MemoryTier.md) — Storage tier for tensor or KV-cache chunks.
- [TieredBuffer](TieredBuffer.md) — Metadata and storage descriptor for offloaded or tier-managed tensors.

## Functions

- [current_ddr5_usage_mb](current_ddr5_usage_mb.md) — Calculate total current DDR5 host memory allocation in Megabytes.
- [current_ddr5_usage_mb](current_ddr5_usage_mb_1.md) — Calculate total current DDR5 host memory allocation in Megabytes.
- [default](default.md)
- [default](default_1.md)
- [new](new.md)
- [new](new_1.md)
- [offload_to_ddr5](offload_to_ddr5.md) — Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.
- [offload_to_ddr5](offload_to_ddr5_1.md) — Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.
- [prefetch_to_vram](prefetch_to_vram.md) — Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.
- [prefetch_to_vram](prefetch_to_vram_1.md) — Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.
- [register_gpu_buffer](register_gpu_buffer.md) — Register a buffer in GPU VRAM
- [register_gpu_buffer](register_gpu_buffer_1.md) — Register a buffer in GPU VRAM
- [should_offload_to_ddr5](should_offload_to_ddr5.md) — Check if GPU VRAM pressure requires offload to DDR5 RAM.
- [should_offload_to_ddr5](should_offload_to_ddr5_1.md) — Check if GPU VRAM pressure requires offload to DDR5 RAM.
- [test_ddr5_offloading_and_prefetch_cycle](test_ddr5_offloading_and_prefetch_cycle.md) — [tokio::test]
