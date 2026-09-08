#[cfg(test)]
mod tests {
    use wasm_forge::{WasmEngine, SkillCrystallizer};

    #[tokio::test]
    async fn test_wat_compilation_and_crystallization() {
        let wat_src = r#"
            (module
                (func (export "add") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add)
            )
        "#;

        let engine = WasmEngine::new().expect("engine init failed");
        let bytes = engine.compile_wat(wat_src).expect("WAT compilation failed");
        assert!(!bytes.is_empty());

        let crystallizer = SkillCrystallizer::new();
        let tool = crystallizer.crystallize_from_wat(
            "fast_add",
            "Hardware fast addition module",
            wat_src
        ).await.expect("crystallization failed");

        assert_eq!(tool.name, "fast_add");
        assert_eq!(tool.wasm_bytes, bytes);

        let retrieved = crystallizer.get_tool("fast_add").await;
        assert!(retrieved.is_some());
    }
}
