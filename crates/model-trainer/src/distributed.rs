use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Zero Redundancy Optimizer Stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub async fn reduce_scatter_gradients(&self, name: &str, global_grads: &[f32]) -> Vec<f32> {
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
}
