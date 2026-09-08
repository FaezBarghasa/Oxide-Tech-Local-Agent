pub struct SparseVectorRepresentation {
    pub indices: Vec<u32>,
    pub values: Vec<f32>,
}

pub fn tokenize_bm25(query: &str) -> SparseVectorRepresentation {
    let mut indices = Vec::new();
    let mut values = Vec::new();

    for (i, word) in query.to_lowercase().split_whitespace().enumerate() {
        let mut hash = 5381u32;
        for c in word.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
        }
        indices.push(hash);
        values.push(1.0 + (1.0 / (i + 1) as f32)); // basic term weighting
    }

    SparseVectorRepresentation { indices, values }
}
