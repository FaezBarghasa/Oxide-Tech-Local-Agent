//! Embedded compile-time assets (.rodata) via rust-embed.
//! Ensures 100% single-binary zero-extraction serving and self-contained runtime.

use rust_embed::RustEmbed;

/// In-memory compiled frontend assets from `src/dist/`.
#[derive(RustEmbed)]
#[folder = "../src/dist/"]
pub struct WebAssets;

/// Embedded SurrealDB schema migrations.
#[derive(RustEmbed)]
#[folder = "../crates/surrealdb-service/src/migrations/"]
#[include = "*.surql"]
pub struct DbMigrations;

/// Embedded setup scripts and template configurations.
#[derive(RustEmbed)]
#[folder = "../scripts/"]
#[include = "config.toml", "install_udev_rules.sh", "init-surrealdb.surql"]
pub struct SetupAssets;
