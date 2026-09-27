# distributed

## Classs

- [DistributedEngine](DistributedEngine.md) — ZeRO-3 Distributed Parameter & Optimizer Engine
- [MmapGgufWeightLoader](MmapGgufWeightLoader.md) — Zero-Copy Memory-Mapped Weight Loader for GGUF & SafeTensors Shards
- [PartitionedTensor](PartitionedTensor.md) — Distributed Tensor Partition
- [ProcessGroup](ProcessGroup.md) — Cluster Node / Process Group Description
- [ZeroStage](ZeroStage.md) — Zero Redundancy Optimizer Stage

## Functions

- [all_gather_parameter](all_gather_parameter.md) — Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass
- [all_gather_parameter](all_gather_parameter_1.md) — Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass
- [is_master](is_master.md)
- [is_master](is_master_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [open](open.md) — Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)
- [open](open_1.md) — Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)
- [read_f32_slice](read_f32_slice.md) — Extract zero-copy f32 slice directly from mapped memory
- [read_f32_slice](read_f32_slice_1.md) — Extract zero-copy f32 slice directly from mapped memory
- [reduce_scatter_gradients](reduce_scatter_gradients.md) — Ring Reduce-Scatter gradients across cluster ranks
- [reduce_scatter_gradients](reduce_scatter_gradients_1.md) — Ring Reduce-Scatter gradients across cluster ranks
- [register_mmap_parameter](register_mmap_parameter.md) — Register partitioned parameter directly from memory-mapped GGUF loader
- [register_mmap_parameter](register_mmap_parameter_1.md) — Register partitioned parameter directly from memory-mapped GGUF loader
- [register_parameter](register_parameter.md) — Partition a global parameter across world_size ranks according to ZeRO stage
- [register_parameter](register_parameter_1.md) — Partition a global parameter across world_size ranks according to ZeRO stage
- [register_tensor_offset](register_tensor_offset.md) — Register tensor byte offset within memory-mapped buffer
- [register_tensor_offset](register_tensor_offset_1.md) — Register tensor byte offset within memory-mapped buffer
- [test_mmap_gguf_loader_zero_copy_partitioning](test_mmap_gguf_loader_zero_copy_partitioning.md) — [tokio::test]
- [test_zero3_parameter_partitioning_and_gather](test_zero3_parameter_partitioning_and_gather.md) — [tokio::test]
