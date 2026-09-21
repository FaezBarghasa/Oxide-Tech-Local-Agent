use forge_rust::{ForgeRust, LanguageDetector, RefactorConfig, SourceLanguage};

#[test]
fn test_language_detection() {
    assert_eq!(
        LanguageDetector::detect("#include <stdio.h>\nint main() { return 0; }", None),
        SourceLanguage::C
    );
    assert_eq!(
        LanguageDetector::detect(
            "def calculate_sum(a: int, b: int) -> int:\n    return a + b",
            None
        ),
        SourceLanguage::Python
    );
    assert_eq!(
        LanguageDetector::detect(
            "interface UserProfile {\n    id: string;\n    age?: number;\n}",
            None
        ),
        SourceLanguage::TypeScript
    );
    assert_eq!(
        LanguageDetector::detect("package main\n\ntype Worker struct {\n    ID int\n}", None),
        SourceLanguage::Go
    );
}

#[test]
fn test_refactor_c_to_rust() {
    let c_code = r#"
        typedef struct {
            int id;
            char* name;
            float temperature;
        } SensorData;

        int read_sensor(SensorData* self) {
            return 0;
        }
    "#;

    let config = RefactorConfig {
        module_name: "c_sensor".to_string(),
        language_hint: Some(SourceLanguage::C),
        verify_syntax: true,
        is_binary: false,
    };

    let result = ForgeRust::refactor(c_code, config).expect("Refactoring C should succeed");
    assert!(result.rust_code.contains("struct SensorData"));
    assert!(result.rust_code.contains("fn read_sensor"));
    assert!(result.files.contains_key("Cargo.toml"));
    assert!(result.files.contains_key("src/lib.rs"));
}

#[test]
fn test_refactor_python_to_rust() {
    let py_code = r#"
class TelemetryStream:
    pass

async def connect_stream(endpoint: str) -> bool:
    return True
    "#;

    let config = RefactorConfig {
        module_name: "py_telemetry".to_string(),
        language_hint: Some(SourceLanguage::Python),
        verify_syntax: true,
        is_binary: false,
    };

    let result = ForgeRust::refactor(py_code, config).expect("Refactoring Python should succeed");
    assert!(result.rust_code.contains("struct TelemetryStream"));
    assert!(result.rust_code.contains("async fn connect_stream"));
    assert!(
        result
            .uir
            .required_dependencies
            .contains(&"tokio".to_string())
    );
}

#[test]
fn test_refactor_typescript_to_rust() {
    let ts_code = r#"
interface DeviceController {
    connect(port: string): Promise<boolean>;
    disconnect(): void;
}
    "#;

    let config = RefactorConfig {
        module_name: "ts_device".to_string(),
        language_hint: Some(SourceLanguage::TypeScript),
        verify_syntax: true,
        is_binary: false,
    };

    let result =
        ForgeRust::refactor(ts_code, config).expect("Refactoring TypeScript should succeed");
    assert!(result.rust_code.contains("trait DeviceController"));
    assert!(result.rust_code.contains("async fn connect"));
    assert!(result.rust_code.contains("fn disconnect"));
}

#[test]
fn test_refactor_go_to_rust() {
    let go_code = r#"
package main

type MotorState struct {
    RPM int
    Enabled bool
}

func (m *MotorState) SetSpeed(targetRPM int) error {
    return nil
}
    "#;

    let config = RefactorConfig {
        module_name: "go_motor".to_string(),
        language_hint: Some(SourceLanguage::Go),
        verify_syntax: true,
        is_binary: false,
    };

    let result = ForgeRust::refactor(go_code, config).expect("Refactoring Go should succeed");
    assert!(result.rust_code.contains("struct MotorState"));
    assert!(result.rust_code.contains("fn set_speed"));
}
