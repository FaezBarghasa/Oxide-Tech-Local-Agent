use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionRound {
    pub round: usize,
    pub critique: String,
    pub resolution: String,
    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    pub id: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub completed: bool,
    pub reflections: Vec<ReflectionRound>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskDag {
    pub nodes: HashMap<String, TaskNode>,
    pub critic_edges: Vec<(String, String)>, // (Task, CriticTask)
}

impl TaskDag {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            critic_edges: Vec::new(),
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
                reflections: Vec::new(),
            },
        );
    }

    pub fn add_critic_edge(&mut self, task_id: &str, critic_task_id: &str) {
        self.critic_edges
            .push((task_id.to_string(), critic_task_id.to_string()));
    }

    pub fn record_reflection(&mut self, id: &str, round: ReflectionRound) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.reflections.push(round);
        }
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
