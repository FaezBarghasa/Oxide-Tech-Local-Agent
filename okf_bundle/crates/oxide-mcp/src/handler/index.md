# handler

## Classs

- [AnalyzeCompilerFailureInput](AnalyzeCompilerFailureInput.md) — [derive(Deserialize, JsonSchema)]
- [ApplyDiffInput](ApplyDiffInput.md) — [derive(Deserialize, JsonSchema)]
- [AutonomousCodeReviewInput](AutonomousCodeReviewInput.md) — [derive(Deserialize, JsonSchema)]
- [BlenderMeshInput](BlenderMeshInput.md) — [derive(Deserialize, JsonSchema)]
- [CargoCheckInput](CargoCheckInput.md) — [derive(Deserialize, JsonSchema)]
- [CargoClippyInput](CargoClippyInput.md) — [derive(Deserialize, JsonSchema)]
- [FetchCrateDocsInput](FetchCrateDocsInput.md) — [derive(Deserialize, JsonSchema)]
- [KiCadSchematicInput](KiCadSchematicInput.md) — [derive(Deserialize, JsonSchema)]
- [ListSymbolsInput](ListSymbolsInput.md) — [derive(Deserialize, JsonSchema)]
- [LiveDocsScrapeInput](LiveDocsScrapeInput.md) — [derive(Deserialize, JsonSchema)]
- [McpServer](McpServer.md) — [derive(Clone)]
- [ProbeRsFlashInput](ProbeRsFlashInput.md) — [derive(Deserialize, JsonSchema)]
- [ProbeRsReadRttInput](ProbeRsReadRttInput.md) — [derive(Deserialize, JsonSchema)]
- [QdrantSearchInput](QdrantSearchInput.md) — [derive(Deserialize, JsonSchema)]
- [QemuBootInput](QemuBootInput.md) — [derive(Deserialize, JsonSchema)]
- [QemuUartInput](QemuUartInput.md) — [derive(Deserialize, JsonSchema)]
- [ReadFileInput](ReadFileInput.md) — [derive(Deserialize, JsonSchema)]
- [RenodeLoadInput](RenodeLoadInput.md) — [derive(Deserialize, JsonSchema)]
- [WriteFileInput](WriteFileInput.md) — [derive(Deserialize, JsonSchema)]

## Functions

- [analyze_compiler_failure](analyze_compiler_failure.md)
- [analyze_compiler_failure](analyze_compiler_failure_1.md)
- [apply_diff](apply_diff.md) — [tool(description = "Apply a search and replace diff to a file in the workspace")]
- [apply_diff](apply_diff_1.md) — [tool(description = "Apply a search and replace diff to a file in the workspace")]
- [autonomous_code_review](autonomous_code_review.md)
- [autonomous_code_review](autonomous_code_review_1.md)
- [blender_generate_mesh](blender_generate_mesh.md) — [tool(description = "Generate 3D mesh object in Blender")]
- [blender_generate_mesh](blender_generate_mesh_1.md) — [tool(description = "Generate 3D mesh object in Blender")]
- [call_tool](call_tool.md)
- [call_tool](call_tool_1.md)
- [cargo_check](cargo_check.md) — [tool(description = "Run cargo check inside the native sandbox")]
- [cargo_check](cargo_check_1.md) — [tool(description = "Run cargo check inside the native sandbox")]
- [cargo_clippy](cargo_clippy.md) — [tool(description = "Run cargo clippy inside the native sandbox")]
- [cargo_clippy](cargo_clippy_1.md) — [tool(description = "Run cargo clippy inside the native sandbox")]
- [fetch_crate_docs](fetch_crate_docs.md) — [tool(description = "Fetch and index crate docs from docs.rs for a specific version")]
- [fetch_crate_docs](fetch_crate_docs_1.md) — [tool(description = "Fetch and index crate docs from docs.rs for a specific version")]
- [get_info](get_info.md)
- [get_info](get_info_1.md)
- [kicad_process_schematic](kicad_process_schematic.md) — [tool(description = "Generate schematic and run Design Rule Checks in KiCad")]
- [kicad_process_schematic](kicad_process_schematic_1.md) — [tool(description = "Generate schematic and run Design Rule Checks in KiCad")]
- [list_symbols](list_symbols.md) — [tool(description = "List all parsed symbols in a file using tree-sitter")]
- [list_symbols](list_symbols_1.md) — [tool(description = "List all parsed symbols in a file using tree-sitter")]
- [list_tools](list_tools.md)
- [list_tools](list_tools_1.md)
- [live_docs_scrape](live_docs_scrape.md) — [tool(description = "Scrape docs.rs for crate updates and updates RAG index")]
- [live_docs_scrape](live_docs_scrape_1.md) — [tool(description = "Scrape docs.rs for crate updates and updates RAG index")]
- [new](new.md) — Create a new McpServer instance.
- [new](new_1.md) — Create a new McpServer instance.
- [probe_rs_flash](probe_rs_flash.md) — [tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")]
- [probe_rs_flash](probe_rs_flash_1.md) — [tool(description = "Flash binary to hardware using probe-rs (requires human confirmation)")]
- [probe_rs_read_rtt](probe_rs_read_rtt.md) — [tool(description = "Read RTT logs from target chip using probe-rs")]
- [probe_rs_read_rtt](probe_rs_read_rtt_1.md) — [tool(description = "Read RTT logs from target chip using probe-rs")]
- [qdrant_search](qdrant_search.md) — [tool(description = "Semantic search over embedded Rust crate docs and workspace AST")]
- [qdrant_search](qdrant_search_1.md) — [tool(description = "Semantic search over embedded Rust crate docs and workspace AST")]
- [qemu_boot](qemu_boot.md) — [tool(description = "Boot OS image in QEMU")]
- [qemu_boot](qemu_boot_1.md) — [tool(description = "Boot OS image in QEMU")]
- [qemu_send_uart](qemu_send_uart.md) — [tool(description = "Send command to QEMU serial port UART")]
- [qemu_send_uart](qemu_send_uart_1.md) — [tool(description = "Send command to QEMU serial port UART")]
- [read_file](read_file.md) — [tool(description = "Read a file from the workspace")]
- [read_file](read_file_1.md) — [tool(description = "Read a file from the workspace")]
- [renode_load_platform](renode_load_platform.md) — [tool(description = "Load platform description script in Renode")]
- [renode_load_platform](renode_load_platform_1.md) — [tool(description = "Load platform description script in Renode")]
- [write_file](write_file.md) — [tool(description = "Write a file to the workspace")]
- [write_file](write_file_1.md) — [tool(description = "Write a file to the workspace")]
