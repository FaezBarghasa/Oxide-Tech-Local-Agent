use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ASTNodeRepr {
    pub id: usize,
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: (usize, usize),
    pub end_point: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedAST {
    pub file_hash: String,
    pub language: String,
    pub nodes: Vec<ASTNodeRepr>,
}

pub fn save_ast_cache(path: &std::path::Path, ast: &CachedAST) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(128 * 1024, file); // 128KB Write Buffer
    bincode::serialize_into(&mut writer, ast)?;
    writer.flush()?;
    Ok(())
}

pub fn load_ast_cache(path: &std::path::Path) -> Result<CachedAST, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(128 * 1024, file); // 128KB Read Buffer
    let ast: CachedAST = bincode::deserialize_from(reader)?;
    Ok(ast)
}
