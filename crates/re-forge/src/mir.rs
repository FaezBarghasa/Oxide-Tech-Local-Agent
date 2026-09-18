use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirOp {
    Load { dest: String, addr: String },
    Store { addr: String, val: String },
    Add { dest: String, a: String, b: String },
    Sub { dest: String, a: String, b: String },
    Mul { dest: String, a: String, b: String },
    MmaTensorCore { dest: String, a: String, b: String, c: String },
    Branch { target_block: usize },
    CondBranch { cond: String, true_block: usize, false_block: usize },
    Return { val: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirBlock {
    pub block_id: usize,
    pub label: String,
    pub operations: Vec<MirOp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirFunction {
    pub name: String,
    pub entry_block: usize,
    pub blocks: Vec<MirBlock>,
}

impl MirFunction {
    pub fn build_cfg(&self) -> DiGraph<String, ()> {
        let mut g = DiGraph::new();
        let mut node_map = std::collections::HashMap::new();

        for b in &self.blocks {
            let idx = g.add_node(b.label.clone());
            node_map.insert(b.block_id, idx);
        }

        for b in &self.blocks {
            if let Some(&src_idx) = node_map.get(&b.block_id) {
                for op in &b.operations {
                    match op {
                        MirOp::Branch { target_block } => {
                            if let Some(&dst_idx) = node_map.get(target_block) {
                                g.add_edge(src_idx, dst_idx, ());
                            }
                        }
                        MirOp::CondBranch { true_block, false_block, .. } => {
                            if let Some(&t_idx) = node_map.get(true_block) {
                                g.add_edge(src_idx, t_idx, ());
                            }
                            if let Some(&f_idx) = node_map.get(false_block) {
                                g.add_edge(src_idx, f_idx, ());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mir_cfg_construction() {
        let func = MirFunction {
            name: "compute_kernel".to_string(),
            entry_block: 0,
            blocks: vec![
                MirBlock {
                    block_id: 0,
                    label: "entry".to_string(),
                    operations: vec![
                        MirOp::Load { dest: "v0".to_string(), addr: "r0".to_string() },
                        MirOp::CondBranch { cond: "v0".to_string(), true_block: 1, false_block: 2 },
                    ],
                },
                MirBlock {
                    block_id: 1,
                    label: "loop_body".to_string(),
                    operations: vec![
                        MirOp::MmaTensorCore {
                            dest: "d0".to_string(),
                            a: "a0".to_string(),
                            b: "b0".to_string(),
                            c: "c0".to_string(),
                        },
                        MirOp::Branch { target_block: 2 },
                    ],
                },
                MirBlock {
                    block_id: 2,
                    label: "exit".to_string(),
                    operations: vec![MirOp::Return { val: None }],
                },
            ],
        };

        let cfg = func.build_cfg();
        assert_eq!(cfg.node_count(), 3);
        assert_eq!(cfg.edge_count(), 3);
    }
}
