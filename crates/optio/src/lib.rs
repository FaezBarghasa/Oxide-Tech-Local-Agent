pub mod oscillation;
pub mod dag;
pub mod impact_analysis;
pub mod context_slicer;
pub mod persona_loop;

pub use oscillation::OscillationDetector;
pub use dag::{TaskDag, TaskNode};
pub use impact_analysis::{ImpactAnalyzer, ImpactSurface};
pub use context_slicer::{GraphContextSlicer, SubgraphSlice};
pub use persona_loop::{PersonaOrchestrator, PlanOutput, PlanStep};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillation() {
        let mut detector = OscillationDetector::new(3);

        assert!(detector.record_and_check("cargo_check:thumbv7em").is_ok());
        assert!(detector.record_and_check("cargo_check:thumbv7em").is_ok());
        
        let third_res = detector.record_and_check("cargo_check:thumbv7em");
        assert!(third_res.is_err(), "Detector must flag oscillation on 3rd identical attempt");
        assert!(third_res.unwrap_err().contains("Oscillation detected"));
    }

    #[test]
    fn test_dag_resolution() {
        let mut dag = TaskDag::new();
        dag.add_node("parse_svd", "Parse SVD file", vec![]);
        dag.add_node("generate_driver", "Generate SPI driver", vec!["parse_svd".to_string()]);
        dag.add_node("verify_driver", "Verify SPI driver in QEMU", vec!["generate_driver".to_string()]);

        let ready = dag.get_ready_tasks();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "parse_svd");

        dag.mark_completed("parse_svd");
        let ready2 = dag.get_ready_tasks();
        assert_eq!(ready2.len(), 1);
        assert_eq!(ready2[0].id, "generate_driver");
    }
}
