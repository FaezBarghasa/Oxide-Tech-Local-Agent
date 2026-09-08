use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Crystallized Skill metadata and recipe format (compatible with Antigravity / Agent Skill spec)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrystallizedSkill {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub prompt_template: String,
    pub step_sequence: Vec<String>,
    pub source_task_id: String,
}

impl CrystallizedSkill {
    /// Format into standard SKILL.md with YAML frontmatter
    pub fn to_markdown(&self) -> String {
        format!(
            "---\nname: {}\ndescription: {}\ntags: [{}]\n---\n\n# {}\n\n{}\n\n## Execution Workflow\n{}\n",
            self.name,
            self.description,
            self.tags.join(", "),
            self.name,
            self.description,
            self.step_sequence
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. {}", i + 1, s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct SkillCrystallizer {
    pub skills_dir: PathBuf,
}

impl SkillCrystallizer {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }

    /// Distill a successful multi-step task execution into a crystallized skill
    pub async fn crystallize_workflow(&self, skill: &CrystallizedSkill) -> Result<PathBuf, String> {
        info!("Crystallizing workflow skill: {}", skill.name);
        let skill_folder = self.skills_dir.join(&skill.name);
        tokio::fs::create_dir_all(&skill_folder)
            .await
            .map_err(|e| format!("Failed to create skill directory: {}", e))?;

        let skill_file = skill_folder.join("SKILL.md");
        let content = skill.to_markdown();

        tokio::fs::write(&skill_file, content)
            .await
            .map_err(|e| format!("Failed to write SKILL.md: {}", e))?;

        Ok(skill_file)
    }

    /// Compose multiple crystallized skills into a composite higher-order workflow
    pub fn compose_skills(
        &self,
        name: &str,
        description: &str,
        skills: &[&CrystallizedSkill],
    ) -> CrystallizedSkill {
        let mut composite_steps = Vec::new();
        let mut all_tags = vec!["composite".to_string()];

        for skill in skills {
            composite_steps.push(format!("### Phase: {}", skill.name));
            composite_steps.extend(skill.step_sequence.clone());
            for tag in &skill.tags {
                if !all_tags.contains(tag) {
                    all_tags.push(tag.clone());
                }
            }
        }

        CrystallizedSkill {
            name: name.to_string(),
            description: description.to_string(),
            tags: all_tags,
            prompt_template: format!("Execute composite pipeline: {}", name),
            step_sequence: composite_steps,
            source_task_id: "composite_orchestration".to_string(),
        }
    }

    /// Crystallize a high-frequency composite skill into a high-performance Wasm tool manifest
    pub async fn crystallize_wasm_super_tool(
        &self,
        skill: &CrystallizedSkill,
        wasm_bytes: &[u8],
    ) -> Result<PathBuf, String> {
        info!("Crystallizing Wasm super-tool for skill: {}", skill.name);
        let tool_folder = self.skills_dir.join("wasm_tools").join(&skill.name);
        tokio::fs::create_dir_all(&tool_folder)
            .await
            .map_err(|e| format!("Failed to create tool directory: {}", e))?;

        let wasm_file = tool_folder.join(format!("{}.wasm", skill.name));
        tokio::fs::write(&wasm_file, wasm_bytes)
            .await
            .map_err(|e| format!("Failed to write wasm binary: {}", e))?;

        let manifest_file = tool_folder.join("manifest.json");
        let manifest = serde_json::json!({
            "name": skill.name,
            "description": skill.description,
            "wasm_binary": format!("{}.wasm", skill.name),
            "tags": skill.tags,
            "step_count": skill.step_sequence.len()
        });

        tokio::fs::write(
            &manifest_file,
            serde_json::to_string_pretty(&manifest)
                .map_err(|e| format!("Failed to serialize manifest: {}", e))?,
        )
        .await
        .map_err(|e| format!("Failed to write manifest.json: {}", e))?;

        Ok(wasm_file)
    }
}
