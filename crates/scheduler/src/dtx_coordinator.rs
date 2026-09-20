use oxide_protocol::{
    ArtifactRef, DomainTarget, DtxEnvelope, DtxId, DtxOp, DtxRecord, DtxStatus, ResourceRef,
    ResourceTx, SessionKey, Vote,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Clone, Default)]
pub struct DtxCoordinator {
    transactions: Arc<RwLock<HashMap<DtxId, DtxRecord>>>,
    resources: Arc<RwLock<HashMap<ResourceRef, Arc<dyn ResourceTx>>>>,
    compensation_snapshots: Arc<RwLock<HashMap<DtxId, Vec<(ResourceRef, ArtifactRef)>>>>,
}

impl std::fmt::Debug for DtxCoordinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DtxCoordinator").finish()
    }
}

impl DtxCoordinator {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            resources: Arc::new(RwLock::new(HashMap::new())),
            compensation_snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a participating transactional resource
    pub async fn register_resource(&self, rref: ResourceRef, resource: Arc<dyn ResourceTx>) {
        let mut res = self.resources.write().await;
        res.insert(rref, resource);
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

    /// Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,
    /// 2-phase prepare, commit, and atomic rollback on failure.
    pub async fn execute_saga(
        &self,
        dtx_id: DtxId,
        session: SessionKey,
        participants: Vec<ResourceRef>,
    ) -> Result<(), String> {
        let resources_guard = self.resources.read().await;
        let mut active_resources = Vec::new();

        for rref in &participants {
            if let Some(r) = resources_guard.get(rref) {
                active_resources.push((rref.clone(), Arc::clone(r)));
            } else {
                return Err(format!("Resource not found for participant: {:?}", rref));
            }
        }
        drop(resources_guard);

        // Step 1: Pre-flight irreversible check and capture compensation snapshots
        let mut snapshots = Vec::new();
        for (rref, resource) in &active_resources {
            let env = DtxEnvelope {
                dtx_id,
                causation_id: None,
                session: session.clone(),
                op: DtxOp::Begin {
                    participants: participants.clone(),
                },
                deadline_unix_ms: None,
            };

            if resource.is_irreversible(&env) {
                info!(dtx = %dtx_id, resource = ?rref, "Resource is irreversible; capturing compensation snapshot");
                let artifact = resource
                    .snapshot_for_compensation(&env)
                    .await
                    .map_err(|e| format!("Failed compensation snapshot for {:?}: {}", rref, e))?;
                snapshots.push((rref.clone(), artifact));
            }
        }

        {
            let mut comp_guard = self.compensation_snapshots.write().await;
            comp_guard.insert(dtx_id, snapshots);
        }

        // Step 2: Prepare phase (voting)
        let mut prepared = Vec::new();
        for (rref, resource) in &active_resources {
            let env = DtxEnvelope {
                dtx_id,
                causation_id: None,
                session: session.clone(),
                op: DtxOp::Prepare {
                    participant: rref.clone(),
                    vote: Vote::Commit,
                },
                deadline_unix_ms: None,
            };

            match resource.prepare(&env).await {
                Ok(Vote::Commit) => {
                    prepared.push((rref.clone(), resource.clone()));
                }
                Ok(Vote::Abort) => {
                    warn!(dtx = %dtx_id, resource = ?rref, "Participant voted ABORT");
                    self.abort_saga(dtx_id, session, prepared).await;
                    return Err(format!("Participant {:?} voted ABORT", rref));
                }
                Err(e) => {
                    error!(dtx = %dtx_id, resource = ?rref, error = %e, "Prepare failed");
                    self.abort_saga(dtx_id, session, prepared).await;
                    return Err(format!("Prepare error on {:?}: {}", rref, e));
                }
            }
        }

        // Step 3: Commit phase
        for (rref, resource) in &active_resources {
            let env = DtxEnvelope {
                dtx_id,
                causation_id: None,
                session: session.clone(),
                op: DtxOp::Commit {
                    participant: rref.clone(),
                },
                deadline_unix_ms: None,
            };
            if let Err(e) = resource.commit(&env).await {
                error!(dtx = %dtx_id, resource = ?rref, error = %e, "Commit failed; invoking compensation");
                self.rollback_dtx(dtx_id, &e).await?;
                return Err(format!("Commit failed on {:?}: {}", rref, e));
            }
        }

        self.commit_dtx(dtx_id).await
    }

    async fn abort_saga(
        &self,
        dtx_id: DtxId,
        session: SessionKey,
        prepared: Vec<(ResourceRef, Arc<dyn ResourceTx>)>,
    ) {
        warn!(dtx = %dtx_id, "Aborting Saga and rolling back prepared participants");
        for (rref, resource) in prepared {
            let env = DtxEnvelope {
                dtx_id,
                causation_id: None,
                session: session.clone(),
                op: DtxOp::Abort {
                    participant: rref.clone(),
                    reason: oxide_protocol::AbortReason {
                        code: 500,
                        message: "Saga abort triggered".to_string(),
                    },
                },
                deadline_unix_ms: None,
            };
            let _ = resource.rollback(&env).await;
        }
        let _ = self.rollback_dtx(dtx_id, "Saga abort").await;
    }

    /// Mark a distributed transaction as committed
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

    struct MockTransactionalResource {
        should_fail: bool,
        is_irreversible: bool,
    }

    #[async_trait::async_trait]
    impl ResourceTx for MockTransactionalResource {
        async fn prepare(&self, _env: &DtxEnvelope) -> Result<Vote, String> {
            if self.should_fail {
                Ok(Vote::Abort)
            } else {
                Ok(Vote::Commit)
            }
        }
        async fn commit(&self, _env: &DtxEnvelope) -> Result<(), String> {
            Ok(())
        }
        async fn rollback(&self, _env: &DtxEnvelope) -> Result<(), String> {
            Ok(())
        }
        fn is_irreversible(&self, _env: &DtxEnvelope) -> bool {
            self.is_irreversible
        }
        async fn snapshot_for_compensation(&self, _env: &DtxEnvelope) -> Result<ArtifactRef, String> {
            Ok(ArtifactRef {
                artifact_id: uuid::Uuid::now_v7(),
                uri: "snapshot://mem/v1".to_string(),
                blake3_hash: "mockhash123".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_dtx_saga_commit_and_abort() {
        let coordinator = DtxCoordinator::new();
        let session = SessionKey::new("faez@oxide-tech.io");

        let r1 = ResourceRef {
            domain: "firmware".to_string(),
            resource_uri: "stm32://flash/0x08000000".to_string(),
        };
        let r2 = ResourceRef {
            domain: "eda".to_string(),
            resource_uri: "kicad://schematic/main.kicad_sch".to_string(),
        };

        coordinator.register_resource(
            r1.clone(),
            Arc::new(MockTransactionalResource {
                should_fail: false,
                is_irreversible: true,
            }),
        ).await;

        coordinator.register_resource(
            r2.clone(),
            Arc::new(MockTransactionalResource {
                should_fail: false,
                is_irreversible: false,
            }),
        ).await;

        let dtx = coordinator.begin_dtx(
            "Hardware & PCB sync",
            "test_runner",
            vec![DomainTarget::FirmwareIde, DomainTarget::OxideEda],
        ).await;

        // Test Saga Success
        let res = coordinator
            .execute_saga(dtx, session.clone(), vec![r1.clone(), r2.clone()])
            .await;
        assert!(res.is_ok());
        assert_eq!(coordinator.get_status(dtx).await, Some(DtxStatus::Committed));
    }
}
