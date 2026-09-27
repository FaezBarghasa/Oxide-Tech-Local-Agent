---
okf_version: "0.2"
type: Module
title: handler
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler
language: rust
---

# handler

## Relationships

| Type | Target |
|------|--------|
| related | [ReadFileInput](/crates/oxide-mcp/src/handler/ReadFileInput.md) |
| related | [WriteFileInput](/crates/oxide-mcp/src/handler/WriteFileInput.md) |
| related | [ApplyDiffInput](/crates/oxide-mcp/src/handler/ApplyDiffInput.md) |
| related | [CargoCheckInput](/crates/oxide-mcp/src/handler/CargoCheckInput.md) |
| related | [CargoClippyInput](/crates/oxide-mcp/src/handler/CargoClippyInput.md) |
| related | [QdrantSearchInput](/crates/oxide-mcp/src/handler/QdrantSearchInput.md) |
| related | [FetchCrateDocsInput](/crates/oxide-mcp/src/handler/FetchCrateDocsInput.md) |
| related | [ListSymbolsInput](/crates/oxide-mcp/src/handler/ListSymbolsInput.md) |
| related | [ProbeRsFlashInput](/crates/oxide-mcp/src/handler/ProbeRsFlashInput.md) |
| related | [ProbeRsReadRttInput](/crates/oxide-mcp/src/handler/ProbeRsReadRttInput.md) |
| related | [QemuBootInput](/crates/oxide-mcp/src/handler/QemuBootInput.md) |
| related | [QemuUartInput](/crates/oxide-mcp/src/handler/QemuUartInput.md) |
| related | [RenodeLoadInput](/crates/oxide-mcp/src/handler/RenodeLoadInput.md) |
| related | [KiCadSchematicInput](/crates/oxide-mcp/src/handler/KiCadSchematicInput.md) |
| related | [BlenderMeshInput](/crates/oxide-mcp/src/handler/BlenderMeshInput.md) |
| related | [LiveDocsScrapeInput](/crates/oxide-mcp/src/handler/LiveDocsScrapeInput.md) |
| related | [AnalyzeCompilerFailureInput](/crates/oxide-mcp/src/handler/AnalyzeCompilerFailureInput.md) |
| related | [AutonomousCodeReviewInput](/crates/oxide-mcp/src/handler/AutonomousCodeReviewInput.md) |
| related | [McpServer](/crates/oxide-mcp/src/handler/McpServer.md) |
| related | [new](/crates/oxide-mcp/src/handler/new.md) |
| related | [analyze_compiler_failure](/crates/oxide-mcp/src/handler/analyze_compiler_failure.md) |
| related | [autonomous_code_review](/crates/oxide-mcp/src/handler/autonomous_code_review.md) |
| related | [read_file](/crates/oxide-mcp/src/handler/read_file.md) |
| related | [write_file](/crates/oxide-mcp/src/handler/write_file.md) |
| related | [apply_diff](/crates/oxide-mcp/src/handler/apply_diff.md) |
| related | [cargo_check](/crates/oxide-mcp/src/handler/cargo_check.md) |
| related | [cargo_clippy](/crates/oxide-mcp/src/handler/cargo_clippy.md) |
| related | [qdrant_search](/crates/oxide-mcp/src/handler/qdrant_search.md) |
| related | [fetch_crate_docs](/crates/oxide-mcp/src/handler/fetch_crate_docs.md) |
| related | [list_symbols](/crates/oxide-mcp/src/handler/list_symbols.md) |
| related | [probe_rs_flash](/crates/oxide-mcp/src/handler/probe_rs_flash.md) |
| related | [probe_rs_read_rtt](/crates/oxide-mcp/src/handler/probe_rs_read_rtt.md) |
| related | [qemu_boot](/crates/oxide-mcp/src/handler/qemu_boot.md) |
| related | [qemu_send_uart](/crates/oxide-mcp/src/handler/qemu_send_uart.md) |
| related | [renode_load_platform](/crates/oxide-mcp/src/handler/renode_load_platform.md) |
| related | [kicad_process_schematic](/crates/oxide-mcp/src/handler/kicad_process_schematic.md) |
| related | [blender_generate_mesh](/crates/oxide-mcp/src/handler/blender_generate_mesh.md) |
| related | [live_docs_scrape](/crates/oxide-mcp/src/handler/live_docs_scrape.md) |
| related | [new](/crates/oxide-mcp/src/handler/new.md) |
| related | [analyze_compiler_failure](/crates/oxide-mcp/src/handler/analyze_compiler_failure.md) |
| related | [autonomous_code_review](/crates/oxide-mcp/src/handler/autonomous_code_review.md) |
| related | [read_file](/crates/oxide-mcp/src/handler/read_file.md) |
| related | [write_file](/crates/oxide-mcp/src/handler/write_file.md) |
| related | [apply_diff](/crates/oxide-mcp/src/handler/apply_diff.md) |
| related | [cargo_check](/crates/oxide-mcp/src/handler/cargo_check.md) |
| related | [cargo_clippy](/crates/oxide-mcp/src/handler/cargo_clippy.md) |
| related | [qdrant_search](/crates/oxide-mcp/src/handler/qdrant_search.md) |
| related | [fetch_crate_docs](/crates/oxide-mcp/src/handler/fetch_crate_docs.md) |
| related | [list_symbols](/crates/oxide-mcp/src/handler/list_symbols.md) |
| related | [probe_rs_flash](/crates/oxide-mcp/src/handler/probe_rs_flash.md) |
| related | [probe_rs_read_rtt](/crates/oxide-mcp/src/handler/probe_rs_read_rtt.md) |
| related | [qemu_boot](/crates/oxide-mcp/src/handler/qemu_boot.md) |
| related | [qemu_send_uart](/crates/oxide-mcp/src/handler/qemu_send_uart.md) |
| related | [renode_load_platform](/crates/oxide-mcp/src/handler/renode_load_platform.md) |
| related | [kicad_process_schematic](/crates/oxide-mcp/src/handler/kicad_process_schematic.md) |
| related | [blender_generate_mesh](/crates/oxide-mcp/src/handler/blender_generate_mesh.md) |
| related | [live_docs_scrape](/crates/oxide-mcp/src/handler/live_docs_scrape.md) |
| related | [get_info](/crates/oxide-mcp/src/handler/get_info.md) |
| related | [list_tools](/crates/oxide-mcp/src/handler/list_tools.md) |
| related | [call_tool](/crates/oxide-mcp/src/handler/call_tool.md) |
| related | [get_info](/crates/oxide-mcp/src/handler/get_info.md) |
| related | [list_tools](/crates/oxide-mcp/src/handler/list_tools.md) |
| related | [call_tool](/crates/oxide-mcp/src/handler/call_tool.md) |
| related | [schemars](/_dependencies/cargo/schemars.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
| related | [rmcp](/_dependencies/cargo/rmcp.md) |
