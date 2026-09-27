---
okf_version: "0.2"
type: Module
title: distributed
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed
language: rust
---

# distributed

## Relationships

| Type | Target |
|------|--------|
| related | [ZeroStage](/crates/model-trainer/src/distributed/ZeroStage.md) |
| related | [ProcessGroup](/crates/model-trainer/src/distributed/ProcessGroup.md) |
| related | [new](/crates/model-trainer/src/distributed/new.md) |
| related | [is_master](/crates/model-trainer/src/distributed/is_master.md) |
| related | [new](/crates/model-trainer/src/distributed/new.md) |
| related | [is_master](/crates/model-trainer/src/distributed/is_master.md) |
| related | [PartitionedTensor](/crates/model-trainer/src/distributed/PartitionedTensor.md) |
| related | [MmapGgufWeightLoader](/crates/model-trainer/src/distributed/MmapGgufWeightLoader.md) |
| related | [open](/crates/model-trainer/src/distributed/open.md) |
| related | [register_tensor_offset](/crates/model-trainer/src/distributed/register_tensor_offset.md) |
| related | [read_f32_slice](/crates/model-trainer/src/distributed/read_f32_slice.md) |
| related | [open](/crates/model-trainer/src/distributed/open.md) |
| related | [register_tensor_offset](/crates/model-trainer/src/distributed/register_tensor_offset.md) |
| related | [read_f32_slice](/crates/model-trainer/src/distributed/read_f32_slice.md) |
| related | [DistributedEngine](/crates/model-trainer/src/distributed/DistributedEngine.md) |
| related | [new](/crates/model-trainer/src/distributed/new.md) |
| related | [register_parameter](/crates/model-trainer/src/distributed/register_parameter.md) |
| related | [register_mmap_parameter](/crates/model-trainer/src/distributed/register_mmap_parameter.md) |
| related | [all_gather_parameter](/crates/model-trainer/src/distributed/all_gather_parameter.md) |
| related | [reduce_scatter_gradients](/crates/model-trainer/src/distributed/reduce_scatter_gradients.md) |
| related | [new](/crates/model-trainer/src/distributed/new.md) |
| related | [register_parameter](/crates/model-trainer/src/distributed/register_parameter.md) |
| related | [register_mmap_parameter](/crates/model-trainer/src/distributed/register_mmap_parameter.md) |
| related | [all_gather_parameter](/crates/model-trainer/src/distributed/all_gather_parameter.md) |
| related | [reduce_scatter_gradients](/crates/model-trainer/src/distributed/reduce_scatter_gradients.md) |
| related | [test_zero3_parameter_partitioning_and_gather](/crates/model-trainer/src/distributed/test_zero3_parameter_partitioning_and_gather.md) |
| related | [test_mmap_gguf_loader_zero_copy_partitioning](/crates/model-trainer/src/distributed/test_mmap_gguf_loader_zero_copy_partitioning.md) |
| related | [memmap2](/_dependencies/cargo/memmap2.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
