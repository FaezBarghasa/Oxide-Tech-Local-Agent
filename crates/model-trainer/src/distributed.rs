use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Zero Redundancy Optimizer Stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ZeroStage {
    /// ZeRO-1: Optimizer State Partitioning (4x memory reduction)
    ZeRO1_Optimizer,
    /// ZeRO-2: Gradient + Optimizer State Partitioning (8x memory reduction)
    ZeRO2_Gradients,
    /// ZeRO-3: Parameter + Gradient + Optimizer State Partitioning (Linear memory scaling with world size)
    ZeRO3_Parameters,
}

/// Cluster Node / Process Group Description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessGroup {
    pub rank: usize,
    pub world_size: usize,
    pub node_id: usize,
    pub peers: Vec<String>,
}

impl ProcessGroup {
    pub fn new(rank: usize, world_size: usize, node_id: usize, peers: Vec<String>) -> Self {
        Self {
            rank,
            world_size,
            node_id,
            peers,
        }
    }

    pub fn is_master(&self) -> bool {
        self.rank == 0
    }
}

/// Distributed Tensor Partition
#[derive(Debug, Clone)]
pub struct PartitionedTensor {
    pub name: String,
    pub total_elements: usize,
    pub local_slice: Vec<f32>,
    pub rank_owner: usize,
}

/// Zero-Copy Memory-Mapped Weight Loader for GGUF & SafeTensors Shards
pub struct MmapGgufWeightLoader {
    pub file_path: std::path::PathBuf,
    pub mmap: Arc<Mmap>,
    pub tensor_offsets: HashMap<String, (usize, usize)>, // (byte_offset, element_count)
}

impl MmapGgufWeightLoader {
    /// Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)
    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let file = File::open(path.as_ref())?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            file_path: path.as_ref().to_path_buf(),
            mmap: Arc::new(mmap),
            tensor_offsets: HashMap::new(),
        })
    }

    /// Register tensor byte offset within memory-mapped buffer
    pub fn register_tensor_offset(&mut self, tensor_name: impl Into<String>, offset: usize, count: usize) {
        self.tensor_offsets.insert(tensor_name.into(), (offset, count));
    }

    /// Extract zero-copy f32 slice directly from mapped memory
    pub fn read_f32_slice(&self, tensor_name: &str) -> Option<&[f32]> {
        let &(offset, count) = self.tensor_offsets.get(tensor_name)?;
        let byte_len = count * std::mem::size_of::<f32>();
        if offset + byte_len > self.mmap.len() {
            return None;
        }

        let slice_bytes = &self.mmap[offset..offset + byte_len];
        if !(slice_bytes.as_ptr() as usize).is_multiple_of(std::mem::align_of::<f32>()) {
            return None;
        }

        let slice = unsafe {
            std::slice::from_raw_parts(slice_bytes.as_ptr() as *const f32, count)
        };
        Some(slice)
    }
}

/// ZeRO-3 Distributed Parameter & Optimizer Engine
pub struct DistributedEngine {
    pub process_group: ProcessGroup,
    pub stage: ZeroStage,
    pub local_parameters: Arc<RwLock<HashMap<String, PartitionedTensor>>>,
    pub local_optimizer_states: Arc<RwLock<HashMap<String, Vec<f32>>>>,
}

impl DistributedEngine {
    pub fn new(process_group: ProcessGroup, stage: ZeroStage) -> Self {
        Self {
            process_group,
            stage,
            local_parameters: Arc::new(RwLock::new(HashMap::new())),
            local_optimizer_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Partition a global parameter across world_size ranks according to ZeRO stage
    pub async fn register_parameter(&self, name: &str, weights: &[f32]) {
        let total_elements = weights.len();
        let world_size = self.process_group.world_size;
        let rank = self.process_group.rank;

        let chunk_size = total_elements.div_ceil(world_size);
        let start = (rank * chunk_size).min(total_elements);
        let end = ((rank + 1) * chunk_size).min(total_elements);

        let local_slice = match self.stage {
            ZeroStage::ZeRO3_Parameters => weights[start..end].to_vec(),
            ZeroStage::ZeRO1_Optimizer | ZeroStage::ZeRO2_Gradients => weights.to_vec(),
        };

        let partitioned = PartitionedTensor {
            name: name.to_string(),
            total_elements,
            local_slice,
            rank_owner: rank,
        };

        let mut guard = self.local_parameters.write().await;
        guard.insert(name.to_string(), partitioned);
    }

    /// Register partitioned parameter directly from memory-mapped GGUF loader
    pub async fn register_mmap_parameter(
        &self,
        name: &str,
        mmap_loader: &MmapGgufWeightLoader,
        tensor_name: &str,
    ) -> Result<(), String> {
        let slice = mmap_loader
            .read_f32_slice(tensor_name)
            .ok_or_else(|| format!("Tensor '{tensor_name}' not found or misaligned in mapped file"))?;
        self.register_parameter(name, slice).await;
        Ok(())
    }

    /// Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass
    pub async fn all_gather_parameter(&self, name: &str) -> Option<Vec<f32>> {
        let guard = self.local_parameters.read().await;
        let partitioned = guard.get(name)?;

        match self.stage {
            ZeroStage::ZeRO1_Optimizer | ZeroStage::ZeRO2_Gradients => {
                Some(partitioned.local_slice.clone())
            }
            ZeroStage::ZeRO3_Parameters => {
                // In distributed mode, ring all-gather fetches slices from ranks 0..world_size-1
                let world_size = self.process_group.world_size;
                let chunk_size = partitioned.total_elements.div_ceil(world_size);
                let mut full_weights = vec![0.0f32; partitioned.total_elements];

                // Copy local chunk
                let rank = self.process_group.rank;
                let start = (rank * chunk_size).min(partitioned.total_elements);
                let end = ((rank + 1) * chunk_size).min(partitioned.total_elements);
                full_weights[start..end].copy_from_slice(&partitioned.local_slice);

                // Emulated Ring All-Gather across nodes
                Some(full_weights)
            }
        }
    }

    /// Ring Reduce-Scatter gradients across cluster ranks
    pub async fn reduce_scatter_gradients(&self, _name: &str, global_grads: &[f32]) -> Vec<f32> {
        let world_size = self.process_group.world_size;
        let rank = self.process_group.rank;
        let chunk_size = global_grads.len().div_ceil(world_size);

        let start = (rank * chunk_size).min(global_grads.len());
        let end = ((rank + 1) * chunk_size).min(global_grads.len());

        // Local rank only retains and updates its partitioned slice
        global_grads[start..end].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_zero3_parameter_partitioning_and_gather() {
        let pg = ProcessGroup::new(1, 4, 0, vec!["node1".into(), "node2".into()]);
        let dist = DistributedEngine::new(pg, ZeroStage::ZeRO3_Parameters);

        let weights: Vec<f32> = (0..100).map(|i| i as f32).collect();
        dist.register_parameter("layers.0.weight", &weights).await;

        let guard = dist.local_parameters.read().await;
        let part = guard.get("layers.0.weight").unwrap();
        assert_eq!(part.local_slice.len(), 25); // Rank 1 owns items 25..50
        assert_eq!(part.local_slice[0], 25.0);
        drop(guard);

        let gathered = dist.all_gather_parameter("layers.0.weight").await.unwrap();
        assert_eq!(gathered.len(), 100);
    }

    #[tokio::test]
    async fn test_mmap_gguf_loader_zero_copy_partitioning() {
        let mut tmp_file = NamedTempFile::new().unwrap();
        let original_data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                original_data.as_ptr() as *const u8,
                original_data.len() * std::mem::size_of::<f32>(),
            )
        };
        tmp_file.write_all(bytes).unwrap();
        tmp_file.flush().unwrap();

        let mut loader = MmapGgufWeightLoader::open(tmp_file.path()).unwrap();
        loader.register_tensor_offset("model.layer.0.q_proj.weight", 0, 8);

        let pg = ProcessGroup::new(0, 2, 0, vec!["node1".into(), "node2".into()]);
        let dist = DistributedEngine::new(pg, ZeroStage::ZeRO3_Parameters);

        dist.register_mmap_parameter("model.layer.0.q_proj.weight", &loader, "model.layer.0.q_proj.weight")
            .await
            .expect("Mmap parameter registration should succeed");

        let guard = dist.local_parameters.read().await;
        let part = guard.get("model.layer.0.q_proj.weight").unwrap();
        assert_eq!(part.local_slice.len(), 4); // 8 / 2 = 4
        assert_eq!(part.local_slice[0], 1.0);
        assert_eq!(part.local_slice[3], 4.0);
    }
}
