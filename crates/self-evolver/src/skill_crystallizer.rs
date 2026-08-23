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
    pub async fn crystallize_workflow(
        &self,
        skill: &CrystallizedSkill,
    ) -> Result<PathBuf, String> {
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
        composite_name: &str,
        composite_desc: &str,
        sub_skills: &[&CrystallizedSkill],
    ) -> CrystallizedSkill {
        let mut combined_steps = Vec::new();
        let mut combined_tags = vec!["composite".to_string()];

        for skill in sub_skills {
            combined_tags.extend(skill.tags.clone());
            combined_steps.push(format!("Execute sub-skill: `{}`", skill.name));
            combined_steps.extend(skill.step_sequence.clone());
        }

        CrystallizedSkill {
            name: composite_name.to_string(),
            description: composite_desc.to_string(),
            tags: combined_tags,
            prompt_template: format!("Composite goal: {}", composite_desc),
            step_sequence: combined_steps,
            source_task_id: "composite_orchestrator".to_string(),
        }
    }
}
