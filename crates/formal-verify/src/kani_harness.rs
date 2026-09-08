use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaniHarnessTarget {
    pub function_name: String,
    pub input_bounds: Vec<(String, String)>,
    pub check_no_panic: bool,
    pub check_no_overflow: bool,
}

/// Generates automated `#[kani::proof]` verification harnesses for embedded Rust algorithms.
pub fn generate_kani_proof_harness(target: &KaniHarnessTarget) -> String {
    let mut harness = String::new();
    harness.push_str("#[cfg(kani)]\n");
    harness.push_str("#[kani::proof]\n");
    harness.push_str("#[kani::unwind(10)]\n");
    harness.push_str(&format!("fn verify_{}_bounds() {{\n", target.function_name));

    for (var_name, type_str) in &target.input_bounds {
        harness.push_str(&format!(
            "    let {}: {} = kani::any();\n",
            var_name, type_str
        ));
    }

    harness.push_str(&format!(
        "    // Call target function with non-deterministic inputs\n"
    ));
    harness.push_str(&format!("    let _res = {}();\n", target.function_name));
    harness.push_str("}\n");

    harness
}
