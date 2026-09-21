use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pin logic directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinDirection {
    Input,         // Circuit drives MCU pin
    Output,        // MCU drives circuit node
    Bidirectional, // Multi-master / tri-state
}

/// Dynamic digital pin bridge linking QEMU GPIOs and analog SPICE nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalPinBridge {
    pub mcu_pin_name: String,
    pub spice_net_name: String,
    pub direction: PinDirection,
    pub logic_high_voltage: f64,
    pub logic_low_voltage: f64,
}

impl DigitalPinBridge {
    pub fn new(
        mcu_pin_name: impl Into<String>,
        spice_net_name: impl Into<String>,
        direction: PinDirection,
    ) -> Self {
        Self {
            mcu_pin_name: mcu_pin_name.into(),
            spice_net_name: spice_net_name.into(),
            direction,
            logic_high_voltage: 3.3,
            logic_low_voltage: 0.0,
        }
    }

    /// Converts MCU digital boolean state to SPICE analog voltage
    pub fn mcu_to_voltage(&self, is_high: bool) -> f64 {
        if is_high {
            self.logic_high_voltage
        } else {
            self.logic_low_voltage
        }
    }

    /// Converts SPICE analog voltage to MCU digital state with 70%/30% hysteresis
    pub fn voltage_to_mcu(&self, voltage: f64) -> Option<bool> {
        let v_high_thresh = self.logic_high_voltage * 0.7;
        let v_low_thresh = self.logic_high_voltage * 0.3;

        if voltage >= v_high_thresh {
            Some(true)
        } else if voltage <= v_low_thresh {
            Some(false)
        } else {
            None // Metastable / in-transition
        }
    }
}

/// Time synchronization mode between discrete MCU cycles and analog SPICE time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoSimSyncMode {
    Lockstep,    // Continuous advance
    FastForward, // Leap to next scheduled I/O transition
    Adaptive,    // Scale step size dynamically based on dV/dt activity
}

/// High-performance MCU + SPICE Orchestrator Bridge
#[derive(Debug, Clone)]
pub struct McuSpiceBridge {
    pub pin_bridges: HashMap<String, DigitalPinBridge>,
    pub sync_mode: CoSimSyncMode,
    pub sim_time_sec: f64,
    pub time_step_sec: f64,
    pub total_mcu_cycles: u64,
}

impl Default for McuSpiceBridge {
    fn default() -> Self {
        Self {
            pin_bridges: HashMap::new(),
            sync_mode: CoSimSyncMode::Adaptive,
            sim_time_sec: 0.0,
            time_step_sec: 1e-6, // 1 microsecond base step
            total_mcu_cycles: 0,
        }
    }
}

impl McuSpiceBridge {
    pub fn register_pin(&mut self, bridge: DigitalPinBridge) {
        self.pin_bridges.insert(bridge.mcu_pin_name.clone(), bridge);
    }

    /// Step simulation timeline by delta_t and advance cycles based on MCU frequency
    pub fn step_simulation(&mut self, delta_t: f64, mcu_freq_hz: f64) {
        self.sim_time_sec += delta_t;
        let cycles = (delta_t * mcu_freq_hz) as u64;
        self.total_mcu_cycles += cycles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_pin_bridge_hysteresis() {
        let bridge = DigitalPinBridge::new("PA5", "NET_LED", PinDirection::Output);

        // Assert high / low conversion
        assert_eq!(bridge.mcu_to_voltage(true), 3.3);
        assert_eq!(bridge.mcu_to_voltage(false), 0.0);

        // Assert hysteresis
        assert_eq!(bridge.voltage_to_mcu(3.0), Some(true));  // > 2.31V
        assert_eq!(bridge.voltage_to_mcu(0.5), Some(false)); // < 0.99V
        assert_eq!(bridge.voltage_to_mcu(1.65), None);        // Metastable
    }

    #[test]
    fn test_mcu_spice_orchestrator_step() {
        let mut orch = McuSpiceBridge::default();
        orch.register_pin(DigitalPinBridge::new("PB6", "NET_I2C_SCL", PinDirection::Bidirectional));

        // Step 1 ms at 168 MHz (STM32F4)
        orch.step_simulation(1e-3, 168_000_000.0);
        assert!((orch.sim_time_sec - 1e-3).abs() < 1e-9);
        assert_eq!(orch.total_mcu_cycles, 168_000);
    }
}
