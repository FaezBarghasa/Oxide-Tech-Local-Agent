use chrono::{DateTime, Utc};
use oxide_protocol::DtxId;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SegmentJournalError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Hash chain corruption detected at sequence {0}")]
    HashChainCorrupted(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    Thought,
    ToolCall,
    Observation,
    Verify,
    Commit,
    Reward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentEvent {
    pub seq: u64,
    pub dtx_id: DtxId,
    pub session_id: Uuid,
    pub at: DateTime<Utc>,
    pub kind: EventKind,
    pub payload: Vec<u8>,
    pub hash_prev: [u8; 32],
}

impl SegmentEvent {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.seq.to_le_bytes());
        hasher.update(self.dtx_id.0.as_bytes());
        hasher.update(self.session_id.as_bytes());
        hasher.update(&[self.kind as u8]);
        hasher.update(&self.payload);
        hasher.update(&self.hash_prev);
        *hasher.finalize().as_bytes()
    }
}

/// Append-only segment file logger (`seg-*.rjnl`)
pub struct SegmentJournal {
    session_id: Uuid,
    journal_dir: PathBuf,
    current_seq: std::sync::atomic::AtomicU64,
    last_hash: std::sync::RwLock<[u8; 32]>,
}

impl SegmentJournal {
    pub fn new(base_dir: impl AsRef<Path>, session_id: Uuid) -> std::io::Result<Self> {
        let journal_dir = base_dir.as_ref().join(session_id.to_string());
        std::fs::create_dir_all(&journal_dir)?;

        Ok(Self {
            session_id,
            journal_dir,
            current_seq: std::sync::atomic::AtomicU64::new(0),
            last_hash: std::sync::RwLock::new([0u8; 32]),
        })
    }

    pub fn append(
        &self,
        dtx_id: DtxId,
        kind: EventKind,
        payload: Vec<u8>,
    ) -> Result<SegmentEvent, SegmentJournalError> {
        let seq = self
            .current_seq
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let prev_hash = *self.last_hash.read().unwrap();

        let event = SegmentEvent {
            seq,
            dtx_id,
            session_id: self.session_id,
            at: Utc::now(),
            kind,
            payload,
            hash_prev: prev_hash,
        };

        let new_hash = event.compute_hash();
        *self.last_hash.write().unwrap() = new_hash;

        let segment_file = self.journal_dir.join(format!("seg-{:08}.rjnl", seq / 1000));
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(segment_file)?;

        use std::io::Write;
        let data = serde_json::to_vec(&event)?;
        let len = (data.len() as u32).to_le_bytes();
        file.write_all(&len)?;
        file.write_all(&data)?;
        file.flush()?;

        Ok(event)
    }

    pub fn replay(&self, upto_seq: Option<u64>) -> Result<Vec<SegmentEvent>, SegmentJournalError> {
        let mut events = Vec::new();
        let mut expected_prev_hash = [0u8; 32];

        let mut segment_paths = Vec::new();
        for entry in std::fs::read_dir(&self.journal_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rjnl") {
                segment_paths.push(path);
            }
        }
        segment_paths.sort();

        for path in segment_paths {
            let bytes = std::fs::read(&path)?;
            let mut cursor = 0;
            while cursor + 4 <= bytes.len() {
                let len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if cursor + len > bytes.len() {
                    break;
                }
                let event: SegmentEvent = serde_json::from_slice(&bytes[cursor..cursor + len])?;
                cursor += len;

                if event.hash_prev != expected_prev_hash {
                    return Err(SegmentJournalError::HashChainCorrupted(event.seq));
                }
                expected_prev_hash = event.compute_hash();

                if let Some(max_seq) = upto_seq
                    && event.seq > max_seq
                {
                    return Ok(events);
                }
                events.push(event);
            }
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_journal_append_and_replay() {
        let temp_dir = std::env::temp_dir().join(format!("oxide_journal_test_{}", Uuid::now_v7()));
        let session_id = Uuid::now_v7();
        let journal = SegmentJournal::new(&temp_dir, session_id).unwrap();

        let dtx = DtxId::new_v7();
        journal
            .append(dtx, EventKind::Thought, b"Initiating RE task".to_vec())
            .unwrap();
        journal
            .append(dtx, EventKind::ToolCall, b"disasm_x86_64".to_vec())
            .unwrap();
        journal
            .append(dtx, EventKind::Verify, b"Formal proof passed".to_vec())
            .unwrap();

        let replayed = journal.replay(None).unwrap();
        assert_eq!(replayed.len(), 3);
        assert_eq!(replayed[0].seq, 0);
        assert_eq!(replayed[1].seq, 1);
        assert_eq!(replayed[2].seq, 2);

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
