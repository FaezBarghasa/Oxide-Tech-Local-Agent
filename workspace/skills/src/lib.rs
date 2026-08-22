use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thinker::SkillType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillInfo {
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
}

pub struct SkillRegistry {
    skills: HashMap<String, SkillInfo>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            skills: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    pub fn register(&mut self, skill: SkillInfo) {
        self.skills.insert(skill.name.clone(), skill);
    }

    pub fn get(&self, name: &str) -> Option<&SkillInfo> {
        self.skills.get(name)
    }

    pub fn list(&self) -> Vec<SkillInfo> {
        self.skills.values().cloned().collect()
    }

    fn register_defaults(&mut self) {
        self.register(SkillInfo {
            name: "code_generate".to_string(),
            description: "Generates new source files or components based on requirements"
                .to_string(),
            skill_type: SkillType::Generate,
        });
        self.register(SkillInfo {
            name: "code_refactor".to_string(),
            description:
                "Restructures existing code to improve readability, performance, or modularity"
                    .to_string(),
            skill_type: SkillType::Refactor,
        });
        self.register(SkillInfo {
            name: "code_review".to_string(),
            description:
                "Analyzes code style, bugs, and architectures against established profiles"
                    .to_string(),
            skill_type: SkillType::Review,
        });
        self.register(SkillInfo {
            name: "design_system".to_string(),
            description:
                "Designs higher-level component interactions, PCB pinouts, or CAD assemblies"
                    .to_string(),
            skill_type: SkillType::Design,
        });
        self.register(SkillInfo {
            name: "upgrade_framework".to_string(),
            description:
                "Identifies API changes and updates files to migrate to newer library versions"
                    .to_string(),
            skill_type: SkillType::Upgrade,
        });
        self.register(SkillInfo {
            name: "analyze_log".to_string(),
            description:
                "Inspects logs, compiler errors, or simulation outputs to identify root causes"
                    .to_string(),
            skill_type: SkillType::Analyze,
        });
    }
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}
