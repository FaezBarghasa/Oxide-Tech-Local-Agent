use crate::analyzer::{DisassembledFunction, DisassembledInstruction};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: usize,
    pub start_address: u64,
    pub end_address: u64,
    pub instructions: Vec<DisassembledInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeKind {
    Sequential,
    ConditionalBranch,
    UnconditionalJump,
    FunctionCall,
}

pub struct ControlFlowGraph {
    pub function_name: String,
    pub graph: DiGraph<BasicBlock, EdgeKind>,
}

impl ControlFlowGraph {
    pub fn from_function(func: &DisassembledFunction) -> Self {
        let mut graph = DiGraph::new();
        let mut blocks: Vec<BasicBlock> = Vec::new();
        let mut current_insts: Vec<DisassembledInstruction> = Vec::new();
        let mut block_id = 0;

        for inst in &func.instructions {
            current_insts.push(inst.clone());

            if inst.is_branch || inst.is_return || inst.is_call {
                if let (Some(first), Some(last)) = (current_insts.first(), current_insts.last()) {
                    blocks.push(BasicBlock {
                        id: block_id,
                        start_address: first.address,
                        end_address: last.address + last.length as u64,
                        instructions: std::mem::take(&mut current_insts),
                    });
                    block_id += 1;
                }
            }
        }

        if !current_insts.is_empty() {
            if let (Some(first), Some(last)) = (current_insts.first(), current_insts.last()) {
                blocks.push(BasicBlock {
                    id: block_id,
                    start_address: first.address,
                    end_address: last.address + last.length as u64,
                    instructions: current_insts,
                });
            }
        }

        let mut node_indices: HashMap<usize, NodeIndex> = HashMap::new();
        for block in &blocks {
            let idx = graph.add_node(block.clone());
            node_indices.insert(block.id, idx);
        }

        // Add sequential flow edges between consecutive non-return blocks
        for i in 0..blocks.len().saturating_sub(1) {
            let curr = &blocks[i];
            let next = &blocks[i + 1];

            if let (Some(&from_idx), Some(&to_idx)) =
                (node_indices.get(&curr.id), node_indices.get(&next.id))
            {
                if let Some(last_inst) = curr.instructions.last() {
                    if !last_inst.is_return {
                        let edge_kind = if last_inst.is_branch {
                            EdgeKind::ConditionalBranch
                        } else {
                            EdgeKind::Sequential
                        };
                        graph.add_edge(from_idx, to_idx, edge_kind);
                    }
                }
            }
        }

        Self {
            function_name: func.name.clone(),
            graph,
        }
    }

    pub fn block_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}
