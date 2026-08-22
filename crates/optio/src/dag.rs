use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    pub id: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskDag {
    pub nodes: HashMap<String, TaskNode>,
}

impl TaskDag {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, description: &str, dependencies: Vec<String>) {
        self.nodes.insert(
            id.to_string(),
            TaskNode {
                id: id.to_string(),
                description: description.to_string(),
                dependencies,
                completed: false,
            },
        );
    }

    pub fn mark_completed(&mut self, id: &str) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.completed = true;
        }
    }

    pub fn get_ready_tasks(&self) -> Vec<TaskNode> {
        let completed_ids: HashSet<String> = self
            .nodes
            .values()
            .filter(|n| n.completed)
            .map(|n| n.id.clone())
            .collect();

        self.nodes
            .values()
            .filter(|n| !n.completed)
            .filter(|n| n.dependencies.iter().all(|dep| completed_ids.contains(dep)))
            .cloned()
            .collect()
    }

    pub fn is_all_completed(&self) -> bool {
        self.nodes.values().all(|n| n.completed)
    }
}
