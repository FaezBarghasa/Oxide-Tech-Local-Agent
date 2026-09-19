use std::collections::HashMap;

/// Project scaffolder that produces a complete, compilable Cargo workspace or standalone crate.
pub struct ProjectScaffolder;

impl ProjectScaffolder {
    pub fn scaffold(
        crate_name: &str,
        emitted_code: &str,
        dependencies: &[String],
        is_binary: bool,
    ) -> HashMap<String, String> {
        let mut files = HashMap::new();

        // 1. Cargo.toml
        let mut cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
            crate_name
        );

        for dep in dependencies {
            match dep.as_str() {
                "serde" => cargo_toml.push_str("serde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\n"),
                "tokio" => cargo_toml.push_str("tokio = { version = \"1\", features = [\"full\"] }\n"),
                "anyhow" => cargo_toml.push_str("anyhow = \"1.0\"\n"),
                "thiserror" => cargo_toml.push_str("thiserror = \"2.0\"\n"),
                "tracing" => cargo_toml.push_str("tracing = \"0.1\"\n"),
                other => cargo_toml.push_str(&format!("{} = \"*\"\n", other)),
            }
        }

        files.insert("Cargo.toml".to_string(), cargo_toml);

        // 2. Source file (src/lib.rs or src/main.rs)
        if is_binary {
            let mut main_content = emitted_code.to_string();
            if !main_content.contains("fn main(") {
                main_content.push_str("\n\nfn main() {\n    println!(\"Project scaffolded by forge-rust!\");\n}\n");
            }
            files.insert("src/main.rs".to_string(), main_content);
        } else {
            files.insert("src/lib.rs".to_string(), emitted_code.to_string());
        }

        // 3. README.md
        let readme = format!(
            "# {}\n\nRefactored to idiomatic Rust 2024 using `forge-rust`.\n\n## Build & Test\n```bash\ncargo build\ncargo test\n```\n",
            crate_name
        );
        files.insert("README.md".to_string(), readme);

        files
    }
}
