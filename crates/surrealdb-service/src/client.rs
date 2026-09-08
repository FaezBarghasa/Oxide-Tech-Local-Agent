use crate::schema::SymbolRecord;
use surrealdb::engine::any::connect;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tree_sitter_service::ast::ParsedSymbol;

pub struct SurrealClient {
    pub db: Surreal<Any>,
}

impl SurrealClient {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let db_url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "mem://".to_string());

        let db = connect(&db_url).await?;
        db.use_ns("workspace").use_db("ast").await?;
        Ok(Self { db })
    }

    pub async fn clear_db(&self) -> Result<(), surrealdb::Error> {
        let _: Vec<SymbolRecord> = self.db.delete("symbol").await?;
        let _ = self.db.query("DELETE implements, belongs_to").await?;
        Ok(())
    }

    pub async fn save_symbols(&self, symbols: &[ParsedSymbol]) -> Result<(), surrealdb::Error> {
        // Clear old records first
        self.clear_db().await?;

        // 1. Insert all symbols and keep track of their IDs
        let mut inserted_ids = std::collections::HashMap::new();

        for sym in symbols {
            let record = SymbolRecord {
                id: None,
                name: sym.name.clone(),
                kind: sym.kind.clone(),
                file_path: sym.file_path.clone(),
                start_line: sym.start_line,
                end_line: sym.end_line,
                content: sym.content.clone(),
                doc_comment: sym.doc_comment.clone(),
                fields: sym.fields.clone(),
                variants: sym.variants.clone(),
                methods: sym.methods.clone(),
                implements_trait: sym.implements_trait.clone(),
                target_type: sym.target_type.clone(),
            };

            let created: Option<SymbolRecord> = self.db.create("symbol").content(record).await?;

            if let Some(rec) = created {
                if let Some(ref rec_id) = rec.id {
                    let key = (sym.kind.clone(), sym.name.clone(), sym.file_path.clone());
                    inserted_ids.insert(key, rec_id.clone());
                }
            }
        }

        // 2. Establish relationships/edges
        for sym in symbols {
            let key = (sym.kind.clone(), sym.name.clone(), sym.file_path.clone());
            if let Some(from_id) = inserted_ids.get(&key) {
                if sym.kind == "impl" {
                    if let Some(ref target_type) = sym.target_type {
                        for other_kind in &["struct", "enum"] {
                            for ((k, name, _), to_id) in &inserted_ids {
                                if k == other_kind && name == target_type {
                                    let _ = self
                                        .db
                                        .query("RELATE $from->belongs_to->$to")
                                        .bind(("from", from_id.clone()))
                                        .bind(("to", to_id.clone()))
                                        .await?;
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(ref implements_trait) = sym.implements_trait {
                        for ((k, name, _), to_id) in &inserted_ids {
                            if k == "trait" && name == implements_trait {
                                let _ = self
                                    .db
                                    .query("RELATE $from->implements->$to")
                                    .bind(("from", from_id.clone()))
                                    .bind(("to", to_id.clone()))
                                    .await?;
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn get_workspace_context(&self) -> Result<String, surrealdb::Error> {
        let mut response = self.db.query("SELECT * FROM symbol").await?;
        let symbols: Vec<SymbolRecord> = response.take(0)?;

        if symbols.is_empty() {
            return Ok("Workspace is empty or has not been parsed yet.".to_string());
        }

        let mut files_map: std::collections::BTreeMap<String, Vec<SymbolRecord>> =
            std::collections::BTreeMap::new();
        for sym in symbols {
            files_map
                .entry(sym.file_path.clone())
                .or_default()
                .push(sym);
        }

        let mut context = String::new();
        for (file_path, syms) in files_map {
            context.push_str(&format!("File: {}\n", file_path));
            for s in syms {
                match s.kind.as_str() {
                    "struct" => {
                        context.push_str(&format!("  struct {}\n", s.name));
                        if let Some(ref fields) = s.fields {
                            for f in fields {
                                context
                                    .push_str(&format!("    - field {}: {}\n", f.name, f.r#type));
                            }
                        }
                    }
                    "enum" => {
                        context.push_str(&format!("  enum {}\n", s.name));
                        if let Some(ref variants) = s.variants {
                            for v in variants {
                                context.push_str(&format!("    - variant {}\n", v));
                            }
                        }
                    }
                    "trait" => {
                        context.push_str(&format!("  trait {}\n", s.name));
                        if let Some(ref methods) = s.methods {
                            for m in methods {
                                context.push_str(&format!("    - method {}\n", m.signature));
                            }
                        }
                    }
                    "impl" => {
                        if let (Some(ref tr), Some(ref target)) =
                            (&s.implements_trait, &s.target_type)
                        {
                            context.push_str(&format!("  impl {} for {}\n", tr, target));
                        } else if let Some(ref target) = s.target_type {
                            context.push_str(&format!("  impl {}\n", target));
                        } else {
                            context.push_str(&format!("  impl {}\n", s.name));
                        }
                        if let Some(ref methods) = s.methods {
                            for m in methods {
                                context.push_str(&format!("    - method {}\n", m.signature));
                            }
                        }
                    }
                    "function" => {
                        context.push_str(&format!("  fn {}\n", s.name));
                    }
                    _ => {}
                }
            }
            context.push('\n');
        }

        Ok(context.trim().to_string())
    }
}
