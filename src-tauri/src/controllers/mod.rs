//! Domain Controller Facade for Oxide-Tech Local Agent Desktop Monolith.
//!
//! Groups all Tauri commands into 4 cohesive engineering domains:
//! - [`agent_controller`]: Inference, dynamic GGUF VRAM admission, training, arena, and skills.
//! - [`system_controller`]: Hardware SWD probe-rs, telemetry, doctor, tunnels, config, and updater.
//! - [`workspace_controller`]: STAIR Code-ToC search, Memanto GraphRAG memory, documents, and research.
//! - [`forge_controller`]: Audio STT/TTS, diffusion media, binary/PTX disassembly, and formal verifier.

pub mod agent_controller;
pub mod forge_controller;
pub mod system_controller;
pub mod workspace_controller;
