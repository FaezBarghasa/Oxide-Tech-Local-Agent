use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinMode {
    InputFloating,
    InputPullUp,
    InputPullDown,
    OutputPushPull,
    OutputOpenDrain,
    AlternateFunctionPushPull(u8),
    AlternateFunctionOpenDrain(u8),
    Analog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinConfig {
    pub pin_name: String,
    pub net_name: String,
    pub mode: PinMode,
    pub estimated_current_ma: f32,
    pub is_5v_tolerant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McuPowerBudget {
    pub vdd_voltage: f32,
    pub max_total_vdd_current_ma: f32,
    pub max_per_pin_current_ma: f32,
}

impl Default for McuPowerBudget {
    fn default() -> Self {
        Self {
            vdd_voltage: 3.3,
            max_total_vdd_current_ma: 150.0, // Typical STM32F4 max VDD current
            max_per_pin_current_ma: 25.0,    // Typical per-pin limit
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum HardwareSafetyViolation {
    #[error(
        "Net short-circuit contention: Multiple Push-Pull outputs connected to net '{net_name}' ({pins:?})"
    )]
    NetShortCircuitContention { net_name: String, pins: Vec<String> },
    #[error("Per-pin current limit exceeded on pin '{pin}': {current_ma} mA > {limit_ma} mA")]
    PerPinCurrentExceeded {
        pin: String,
        current_ma: f32,
        limit_ma: f32,
    },
    #[error("Total MCU power budget exceeded on VDD: {total_ma} mA > {limit_ma} mA")]
    TotalPowerBudgetExceeded { total_ma: f32, limit_ma: f32 },
    #[error(
        "Critical Debug/JTAG pin locked: Pin '{pin}' is mapped to debugging function ({debug_function}) and cannot be configured as generic output"
    )]
    DebugPinContention { pin: String, debug_function: String },
    #[error(
        "Voltage level violation on non-5V-tolerant pin '{pin}': net '{net_name}' operating at 5.0V"
    )]
    VoltageToleranceExceeded { pin: String, net_name: String },
}

/// Static Physical Rule and Pre-Flight Hardware Constraint Solver
#[derive(Debug, Clone)]
pub struct HardwareSafetyChecker {
    power_budget: McuPowerBudget,
    protected_pins: HashMap<String, String>, // e.g., "PA13" -> "SWDIO", "PA14" -> "SWCLK"
}

impl Default for HardwareSafetyChecker {
    fn default() -> Self {
        let mut protected = HashMap::new();
        protected.insert("PA13".to_string(), "SWDIO (JTAG/SWD Debug)".to_string());
        protected.insert("PA14".to_string(), "SWCLK (JTAG/SWD Debug)".to_string());
        protected.insert("PB3".to_string(), "SWO / TRACESWO (Debug)".to_string());
        protected.insert("PB4".to_string(), "NJTRST (JTAG)".to_string());
        protected.insert("PA15".to_string(), "JTDI (JTAG)".to_string());

        Self {
            power_budget: McuPowerBudget::default(),
            protected_pins: protected,
        }
    }
}

impl HardwareSafetyChecker {
    pub fn new(power_budget: McuPowerBudget) -> Self {
        Self {
            power_budget,
            ..Default::default()
        }
    }

    /// Run comprehensive pre-flight verification on the proposed pinout and netlist configuration
    pub fn verify_hardware_safety(
        &self,
        pins: &[PinConfig],
        net_voltages: &HashMap<String, f32>,
    ) -> Result<(), Vec<HardwareSafetyViolation>> {
        let mut violations = Vec::new();

        // 1. Group by net to detect multi-driver Push-Pull short circuit contention
        let mut net_to_push_pull_pins: HashMap<String, Vec<String>> = HashMap::new();
        let mut total_current_ma = 0.0;
        let mut seen_pins = HashSet::new();

        for pin in pins {
            seen_pins.insert(pin.pin_name.clone());

            // Check Debug Pin locks
            if let Some(debug_fn) = self.protected_pins.get(&pin.pin_name) {
                match pin.mode {
                    PinMode::OutputPushPull
                    | PinMode::OutputOpenDrain
                    | PinMode::AlternateFunctionPushPull(_) => {
                        violations.push(HardwareSafetyViolation::DebugPinContention {
                            pin: pin.pin_name.clone(),
                            debug_function: debug_fn.clone(),
                        });
                    }
                    _ => {}
                }
            }

            // Check Push-Pull outputs per net
            match pin.mode {
                PinMode::OutputPushPull | PinMode::AlternateFunctionPushPull(_) => {
                    net_to_push_pull_pins
                        .entry(pin.net_name.clone())
                        .or_default()
                        .push(pin.pin_name.clone());
                }
                _ => {}
            }

            // Per-pin current evaluation
            if pin.estimated_current_ma > self.power_budget.max_per_pin_current_ma {
                violations.push(HardwareSafetyViolation::PerPinCurrentExceeded {
                    pin: pin.pin_name.clone(),
                    current_ma: pin.estimated_current_ma,
                    limit_ma: self.power_budget.max_per_pin_current_ma,
                });
            }
            total_current_ma += pin.estimated_current_ma.abs();

            // Voltage level tolerance check (e.g. 5V net on non-FT pin)
            if let Some(&voltage) = net_voltages.get(&pin.net_name)
                && voltage > 3.6
                && !pin.is_5v_tolerant
            {
                violations.push(HardwareSafetyViolation::VoltageToleranceExceeded {
                    pin: pin.pin_name.clone(),
                    net_name: pin.net_name.clone(),
                });
            }
        }

        // Evaluate Net Contention (more than 1 push-pull driver on same net)
        for (net, drivers) in net_to_push_pull_pins {
            if drivers.len() > 1 && net != "GND" && net != "VDD" && net != "NC" {
                violations.push(HardwareSafetyViolation::NetShortCircuitContention {
                    net_name: net,
                    pins: drivers,
                });
            }
        }

        // Evaluate Total VDD Current Envelope
        if total_current_ma > self.power_budget.max_total_vdd_current_ma {
            violations.push(HardwareSafetyViolation::TotalPowerBudgetExceeded {
                total_ma: total_current_ma,
                limit_ma: self.power_budget.max_total_vdd_current_ma,
            });
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pull_contention_detected() {
        let checker = HardwareSafetyChecker::default();
        let pins = vec![
            PinConfig {
                pin_name: "PB6".into(),
                net_name: "SPI1_MOSI".into(),
                mode: PinMode::OutputPushPull,
                estimated_current_ma: 5.0,
                is_5v_tolerant: true,
            },
            PinConfig {
                pin_name: "PB7".into(),
                net_name: "SPI1_MOSI".into(), // Same net! Contention!
                mode: PinMode::OutputPushPull,
                estimated_current_ma: 5.0,
                is_5v_tolerant: true,
            },
        ];

        let result = checker.verify_hardware_safety(&pins, &HashMap::new());
        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(matches!(
            violations[0],
            HardwareSafetyViolation::NetShortCircuitContention { .. }
        ));
    }

    #[test]
    fn test_jtag_pin_protection() {
        let checker = HardwareSafetyChecker::default();
        let pins = vec![PinConfig {
            pin_name: "PA13".into(), // SWDIO!
            net_name: "LED_HEARTBEAT".into(),
            mode: PinMode::OutputPushPull,
            estimated_current_ma: 10.0,
            is_5v_tolerant: false,
        }];

        let result = checker.verify_hardware_safety(&pins, &HashMap::new());
        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(matches!(
            violations[0],
            HardwareSafetyViolation::DebugPinContention { .. }
        ));
    }

    #[test]
    fn test_total_power_envelope_exceeded() {
        let checker = HardwareSafetyChecker::new(McuPowerBudget {
            vdd_voltage: 3.3,
            max_total_vdd_current_ma: 50.0,
            max_per_pin_current_ma: 25.0,
        });

        let pins = vec![
            PinConfig {
                pin_name: "PC0".into(),
                net_name: "RELAY_1".into(),
                mode: PinMode::OutputPushPull,
                estimated_current_ma: 20.0,
                is_5v_tolerant: true,
            },
            PinConfig {
                pin_name: "PC1".into(),
                net_name: "RELAY_2".into(),
                mode: PinMode::OutputPushPull,
                estimated_current_ma: 20.0,
                is_5v_tolerant: true,
            },
            PinConfig {
                pin_name: "PC2".into(),
                net_name: "RELAY_3".into(),
                mode: PinMode::OutputPushPull,
                estimated_current_ma: 20.0, // Total = 60mA > 50mA limit
                is_5v_tolerant: true,
            },
        ];

        let result = checker.verify_hardware_safety(&pins, &HashMap::new());
        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(
            violations
                .iter()
                .any(|v| matches!(v, HardwareSafetyViolation::TotalPowerBudgetExceeded { .. }))
        );
    }
}
