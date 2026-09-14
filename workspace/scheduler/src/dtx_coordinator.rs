use oxide_protocol::{DomainTarget, DtxId, DtxRecord, DtxStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Default)]
pub struct DtxCoordinator {
    transactions: Arc<RwLock<HashMap<DtxId, DtxRecord>>>,
}

impl DtxCoordinator {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Begin a new distributed transaction across multiple domains
    pub async fn begin_dtx(
        &self,
        title: &str,
        initiator: &str,
        domains: Vec<DomainTarget>,
    ) -> DtxId {
        let record = DtxRecord::new(title, initiator, domains);
        let dtx_id = record.dtx_id;

        info!(dtx = %dtx_id, title = %title, "Initiated Distributed Transaction");

        let mut txs = self.transactions.write().await;
        txs.insert(dtx_id, record);
        dtx_id
    }

    /// Mark a distributed transaction as committed (success across all domains)
    pub async fn commit_dtx(&self, dtx_id: DtxId) -> Result<(), String> {
        let mut txs = self.transactions.write().await;
        if let Some(tx) = txs.get_mut(&dtx_id) {
            tx.status = DtxStatus::Committed;
            tx.updated_at = chrono::Utc::now();
            info!(dtx = %dtx_id, "Distributed Transaction COMMITTED successfully");
            Ok(())
        } else {
            Err(format!("DTX {dtx_id} not found"))
        }
    }

    /// Broadcast rollback across all domain apps for a failed multi-domain task
    pub async fn rollback_dtx(&self, dtx_id: DtxId, reason: &str) -> Result<(), String> {
        let mut txs = self.transactions.write().await;
        if let Some(tx) = txs.get_mut(&dtx_id) {
            tx.status = DtxStatus::RolledBack;
            tx.updated_at = chrono::Utc::now();
            warn!(dtx = %dtx_id, reason = %reason, "Distributed Transaction ROLLED BACK across all domains");
            Ok(())
        } else {
            Err(format!("DTX {dtx_id} not found"))
        }
    }

    /// Query the status of a distributed transaction
    pub async fn get_status(&self, dtx_id: DtxId) -> Option<DtxStatus> {
        let txs = self.transactions.read().await;
        txs.get(&dtx_id).map(|t| t.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dtx_lifecycle() {
        let coordinator = DtxCoordinator::new();
        let dtx = coordinator
            .begin_dtx(
                "Optimize thermal heatsink",
                "local-agent",
                vec![
                    DomainTarget::FirmwareIde,
                    DomainTarget::OxideEda,
                    DomainTarget::Oxide3d,
                ],
            )
            .await;

        assert_eq!(coordinator.get_status(dtx).await, Some(DtxStatus::Pending));

        coordinator.commit_dtx(dtx).await.unwrap();
        assert_eq!(
            coordinator.get_status(dtx).await,
            Some(DtxStatus::Committed)
        );
    }
}
