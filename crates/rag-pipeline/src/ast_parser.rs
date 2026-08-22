use std::path::PathBuf;
use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event, EventKind, RecommendedWatcher};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use syn::{Item};
use std::fs;

/// AST parser that watches a directory of Rust source files and extracts a simple symbol
/// table (function names, struct names, enum names). The resulting symbol table is stored
/// in an in‑memory `HashMap` keyed by the file path.
pub struct AstParser {
    /// Root directory to watch.
    root: PathBuf,
    /// Shared symbol table.
    symbols: Arc<Mutex<HashMap<PathBuf, Vec<String>>>>,
    /// Underlying file system watcher.
    watcher: RecommendedWatcher,
}

impl AstParser {
    /// Create a new `AstParser` watching `root`. Immediately performs an initial scan.
    pub fn new(root: impl Into<PathBuf>) -> NotifyResult<Self> {
        let root = root.into();
        let symbols = Arc::new(Mutex::new(HashMap::new()));
        let symbols_clone = Arc::clone(&symbols);
        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            if ext == "rs" {
                                // Parse the file and update symbols.
                                if let Ok(src) = fs::read_to_string(&path) {
                                    if let Ok(ast) = syn::parse_file(&src) {
                                        let mut names = Vec::new();
                                        for item in ast.items {
                                            match item {
                                                Item::Fn(f) => names.push(f.sig.ident.to_string()),
                                                Item::Struct(s) => names.push(s.ident.to_string()),
                                                Item::Enum(e) => names.push(e.ident.to_string()),
                                                _ => {}
                                            }
                                        }
                                        let mut map = symbols_clone.lock().unwrap();
                                        map.insert(path.clone(), names);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }, notify::Config::default())?;
        watcher.watch(&root, RecursiveMode::Recursive)?;
        // Initial full scan
        Self::initial_scan(&root, &symbols)?;
        Ok(Self { root, symbols, watcher })
    }

    fn initial_scan(root: &PathBuf, symbols: &Arc<Mutex<HashMap<PathBuf, Vec<String>>>>) -> NotifyResult<()> {
        for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(src) = fs::read_to_string(path) {
                    if let Ok(ast) = syn::parse_file(&src) {
                        let mut names = Vec::new();
                        for item in ast.items {
                            match item {
                                Item::Fn(f) => names.push(f.sig.ident.to_string()),
                                Item::Struct(s) => names.push(s.ident.to_string()),
                                Item::Enum(e) => names.push(e.ident.to_string()),
                                _ => {}
                            }
                        }
                        symbols.lock().unwrap().insert(path.to_path_buf(), names);
                    }
                }
            }
        }
        Ok(())
    }

    /// Retrieve a snapshot of the current symbol table.
    pub fn snapshot(&self) -> HashMap<PathBuf, Vec<String>> {
        self.symbols.lock().unwrap().clone()
    }
}
