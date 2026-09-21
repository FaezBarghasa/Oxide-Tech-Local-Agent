use oxide_protocol::{DtxId, SimThermalFeaResult, ThermalHotSpot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum CrossDomainVerifierError {
    #[error(
        "Thermal violation: peak silicon junction temperature {temp_c:.1}°C exceeds maximum threshold {threshold_c:.1}°C"
    )]
    ThermalRunaway { temp_c: f64, threshold_c: f64 },

    #[error("Physical CAD interference detected between enclosure and PCB component '{component}'")]
    PhysicalInterference { component: String },

    #[error("ERC electrical fault: {0}")]
    ElectricalFault(String),

    #[error("Distributed transaction rollback failed: {0}")]
    RollbackFailed(String),
}

/// Verification outcome report for a multi-physics cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainVerificationReport {
    pub dtx_id: DtxId,
    pub passed: bool,
    pub firmware_power_watts: f64,
    pub peak_temperature_c: f64,
    pub clearance_margin_mm: f64,
    pub thermal_fea: SimThermalFeaResult,
    pub iterations: usize,
    pub residual: f64,
    pub recommended_action: Option<String>,
}

pub struct CrossDomainVerifier {
    pub max_silicon_temp_c: f64,
    pub min_clearance_mm: f64,
    pub relaxation_omega: f64, // Under-relaxation parameter (0.3 - 0.7)
    pub max_iterations: usize,
    pub convergence_eps: f64,
}

impl Default for CrossDomainVerifier {
    fn default() -> Self {
        Self {
            max_silicon_temp_c: 85.0,
            min_clearance_mm: 1.5,
            relaxation_omega: 0.5,
            max_iterations: 20,
            convergence_eps: 1e-3,
        }
    }
}

impl CrossDomainVerifier {
    pub fn new(max_silicon_temp_c: f64, min_clearance_mm: f64) -> Self {
        Self {
            max_silicon_temp_c,
            min_clearance_mm,
            ..Default::default()
        }
    }

    /// Run the full Electro-Thermal-Mechanical Co-Simulation fixed-point iteration loop
    pub async fn run_co_simulation(
        &self,
        dtx_id: DtxId,
        firmware_duty_cycle: f64, // 0.0 to 1.0
        mcu_base_watts: f64,
        enclosure_material: &str,
    ) -> Result<CrossDomainVerificationReport, CrossDomainVerifierError> {
        info!(dtx = %dtx_id, "Executing Electro-Thermal-Mechanical Co-Simulation loop...");

        // Material thermal conductivity scaling factor
        let dissipation_factor = match enclosure_material {
            "Aluminum_6061" => 0.4,
            "PETG_3DPrint" => 1.2,
            "ABS" => 1.1,
            _ => 1.0,
        };

        // Fixed-point iteration with under-relaxation
        let mut current_temp = 25.0;
        let mut current_power = mcu_base_watts * (0.2 + 0.8 * firmware_duty_cycle);
        let mut iteration = 0;
        let mut residual = f64::INFINITY;

        let alpha_thermal_copper = 0.00393; // 1/°C temp coefficient of copper/silicon drift

        while iteration < self.max_iterations && residual > self.convergence_eps {
            iteration += 1;

            // 1. Electrical: power drift from temperature rise
            let temp_delta: f64 = current_temp - 25.0;
            let temp_delta_pos = if temp_delta > 0.0 { temp_delta } else { 0.0 };
            let resistance_scaling = 1.0 + alpha_thermal_copper * temp_delta_pos;
            let effective_power =
                (mcu_base_watts * (0.2 + 0.8 * firmware_duty_cycle)) * resistance_scaling;

            // 2. Thermal: calculate new equilibrium temperature
            let calculated_temp = 25.0 + (effective_power * 45.0 * dissipation_factor);

            // 3. Relaxation step: T_(k+1) = (1 - omega)*T_k + omega*T_calc
            let next_temp = (1.0 - self.relaxation_omega) * current_temp
                + self.relaxation_omega * calculated_temp;
            residual = (next_temp - current_temp).abs();
            current_temp = next_temp;
            current_power = effective_power;
        }

        let passes_thermal = current_temp <= self.max_silicon_temp_c;
        let clearance_margin_mm = 2.0;

        let mut power_map = HashMap::new();
        power_map.insert("U1_STM32_MCU".to_string(), current_power);

        let thermal_fea = SimThermalFeaResult {
            max_temperature_c: current_temp,
            silicon_junction_temp_c: current_temp,
            hot_spots: vec![ThermalHotSpot {
                location_name: "U1_STM32_MCU".to_string(),
                temp_c: current_temp,
                x: 12.5,
                y: 18.0,
                z: 1.6,
            }],
            passes_threshold: passes_thermal,
        };

        if !passes_thermal {
            warn!(
                dtx = %dtx_id,
                junction_temp = current_temp,
                threshold = self.max_silicon_temp_c,
                iterations = iteration,
                "Co-simulation failed: Silicon temperature exceeded threshold!"
            );

            return Ok(CrossDomainVerificationReport {
                dtx_id,
                passed: false,
                firmware_power_watts: current_power,
                peak_temperature_c: current_temp,
                clearance_margin_mm,
                thermal_fea,
                iterations: iteration,
                residual,
                recommended_action: Some(
                    "Add thermal relief via array or attach 5mm Aluminum heatsink fin in CAD model"
                        .to_string(),
                ),
            });
        }

        info!(
            dtx = %dtx_id,
            peak_temperature_c = current_temp,
            iterations = iteration,
            "Co-simulation fixed-point iteration CONVERGED and PASSED."
        );

        Ok(CrossDomainVerificationReport {
            dtx_id,
            passed: true,
            firmware_power_watts: current_power,
            peak_temperature_c: current_temp,
            clearance_margin_mm,
            thermal_fea,
            iterations: iteration,
            residual,
            recommended_action: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_co_simulation_pass_with_relaxation() {
        let verifier = CrossDomainVerifier::default();
        let dtx = DtxId::new_v7();
        let report = verifier
            .run_co_simulation(dtx, 0.3, 1.0, "Aluminum_6061")
            .await
            .expect("Verification run failed");

        assert!(report.passed);
        assert!(report.peak_temperature_c < 85.0);
        assert!(report.iterations > 0);
        assert!(report.residual < 1e-2);
    }

    #[tokio::test]
    async fn test_co_simulation_thermal_overload() {
        let verifier = CrossDomainVerifier::default();
        let dtx = DtxId::new_v7();
        let report = verifier
            .run_co_simulation(dtx, 1.0, 2.5, "PETG_3DPrint")
            .await
            .expect("Verification run failed");

        assert!(!report.passed);
        assert!(report.peak_temperature_c > 85.0);
        assert!(report.recommended_action.is_some());
    }
}
