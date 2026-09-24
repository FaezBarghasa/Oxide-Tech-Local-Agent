use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../schemas/pcb_layout.fbs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let schema_path = Path::new(&manifest_dir).join("../../schemas/pcb_layout.fbs");
    let checksum_path = Path::new(&manifest_dir).join("../../schemas/pcb_layout.fbs.blake3");
    let generated_file = Path::new(&manifest_dir).join("../../generated/pcb_layout_generated.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_out_file = out_dir.join("pcb_layout_generated.rs");

    if !schema_path.exists() {
        println!("cargo:warning=Schema pcb_layout.fbs not found at {:?}", schema_path);
        return;
    }

    let schema_bytes = match fs::read(&schema_path) {
        Ok(b) => b,
        Err(e) => {
            println!("cargo:warning=Failed to read schema file: {}", e);
            return;
        }
    };
    let current_hash = blake3::hash(&schema_bytes).to_hex().to_string();

    let stored_hash = if checksum_path.exists() {
        fs::read_to_string(&checksum_path).unwrap_or_default().trim().to_string()
    } else {
        String::new()
    };

    let flatc_in_path = Command::new("flatc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if current_hash != stored_hash && flatc_in_path {
        println!("cargo:warning=pcb_layout.fbs checksum changed. Recompiling via flatc...");
        let status = Command::new("flatc")
            .arg("--rust")
            .arg("-o")
            .arg(&out_dir)
            .arg(&schema_path)
            .status();

        if let Ok(st) = status {
            if st.success() {
                let _ = fs::write(&checksum_path, &current_hash);
                let _ = fs::copy(out_dir.join("pcb_layout_generated.rs"), &generated_file);
            }
        }
    }

    // Ensure target_out_file exists in OUT_DIR from pre-generated file if needed
    if !target_out_file.exists() && generated_file.exists() {
        let _ = fs::copy(&generated_file, &target_out_file);
    }

    if !checksum_path.exists() {
        let _ = fs::write(&checksum_path, &current_hash);
    }
}
