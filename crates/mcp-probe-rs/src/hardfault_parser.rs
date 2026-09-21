use serde::{Deserialize, Serialize};

/// ARM Cortex-M Configurable Fault Status Register (CFSR) Flags
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedCfsr {
    // MemManage Faults (MMFSR: bits [7:0])
    pub mem_instruction_access_violation: bool,
    pub mem_data_access_violation: bool,
    pub mem_stack_unstack_violation: bool,
    pub mem_fault_address_valid: bool,

    // Bus Faults (BFSR: bits [15:8])
    pub bus_instruction_error: bool,
    pub bus_precise_data_error: bool,
    pub bus_imprecise_data_error: bool,
    pub bus_unstack_error: bool,
    pub bus_fault_address_valid: bool,

    // Usage Faults (UFSR: bits [31:16])
    pub usage_undefined_instruction: bool,
    pub usage_invalid_state: bool,
    pub usage_invalid_pc_load: bool,
    pub usage_no_coprocessor: bool,
    pub usage_unaligned_access: bool,
    pub usage_divide_by_zero: bool,
}

/// ARM Cortex-M HardFault Status Register (HFSR) Flags
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedHfsr {
    pub vector_table_read_fault: bool,
    pub forced_hardfault: bool,
    pub debug_event: bool,
}

/// Structured Cortex-M Crash and Panic Report
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardFaultReport {
    pub pc: u32,
    pub lr: u32,
    pub sp: u32,
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub cfsr_raw: u32,
    pub hfsr_raw: u32,
    pub mmar_raw: Option<u32>,
    pub bfar_raw: Option<u32>,
    pub decoded_cfsr: DecodedCfsr,
    pub decoded_hfsr: DecodedHfsr,
    pub probable_cause: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmRegisterFrame {
    pub pc: u32,
    pub lr: u32,
    pub sp: u32,
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
}

pub struct HardFaultParser;

impl HardFaultParser {
    /// Decode raw ARM register status values into a structured fault diagnostic
    pub fn parse_fault_frame(
        frame: ArmRegisterFrame,
        cfsr: u32,
        hfsr: u32,
        mmar: Option<u32>,
        bfar: Option<u32>,
    ) -> HardFaultReport {
        let decoded_cfsr = DecodedCfsr {
            // MMFSR
            mem_instruction_access_violation: (cfsr & (1 << 0)) != 0,
            mem_data_access_violation: (cfsr & (1 << 1)) != 0,
            mem_stack_unstack_violation: (cfsr & (1 << 3)) != 0 || (cfsr & (1 << 4)) != 0,
            mem_fault_address_valid: (cfsr & (1 << 7)) != 0,

            // BFSR
            bus_instruction_error: (cfsr & (1 << 8)) != 0,
            bus_precise_data_error: (cfsr & (1 << 9)) != 0,
            bus_imprecise_data_error: (cfsr & (1 << 10)) != 0,
            bus_unstack_error: (cfsr & (1 << 11)) != 0,
            bus_fault_address_valid: (cfsr & (1 << 15)) != 0,

            // UFSR
            usage_undefined_instruction: (cfsr & (1 << 16)) != 0,
            usage_invalid_state: (cfsr & (1 << 17)) != 0,
            usage_invalid_pc_load: (cfsr & (1 << 18)) != 0,
            usage_no_coprocessor: (cfsr & (1 << 19)) != 0,
            usage_unaligned_access: (cfsr & (1 << 24)) != 0,
            usage_divide_by_zero: (cfsr & (1 << 25)) != 0,
        };

        let decoded_hfsr = DecodedHfsr {
            vector_table_read_fault: (hfsr & (1 << 1)) != 0,
            forced_hardfault: (hfsr & (1 << 30)) != 0,
            debug_event: (hfsr & (1 << 31)) != 0,
        };

        let mut causes = Vec::new();

        if decoded_cfsr.usage_divide_by_zero {
            causes.push("Integer divide by zero in user firmware");
        }
        if decoded_cfsr.usage_unaligned_access {
            causes.push("Unaligned memory access (e.g. pointer casting u8* to u32*)");
        }
        if decoded_cfsr.usage_undefined_instruction {
            causes.push("Attempted to execute undefined/corrupted instruction opcode");
        }
        if decoded_cfsr.bus_precise_data_error {
            if let Some(addr) = bfar {
                causes.push(Box::leak(
                    format!(
                        "Precise bus fault reading/writing memory address {:#010X}",
                        addr
                    )
                    .into_boxed_str(),
                ));
            } else {
                causes.push("Precise bus fault accessing invalid peripheral register or memory");
            }
        }
        if decoded_cfsr.mem_data_access_violation {
            if let Some(addr) = mmar {
                causes.push(Box::leak(
                    format!("MPU / Memory management violation at {:#010X}", addr).into_boxed_str(),
                ));
            } else {
                causes.push("Memory protection (MPU) fault accessing forbidden region");
            }
        }
        if causes.is_empty() {
            if decoded_hfsr.vector_table_read_fault {
                causes.push("Failed to read vector table during exception entry");
            } else {
                causes.push("Forced HardFault triggered by unhandled fault exception");
            }
        }

        let probable_cause = causes.join("; ");

        HardFaultReport {
            pc: frame.pc,
            lr: frame.lr,
            sp: frame.sp,
            r0: frame.r0,
            r1: frame.r1,
            r2: frame.r2,
            r3: frame.r3,
            r12: frame.r12,
            cfsr_raw: cfsr,
            hfsr_raw: hfsr,
            mmar_raw: mmar,
            bfar_raw: bfar,
            decoded_cfsr,
            decoded_hfsr,
            probable_cause,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_by_zero_usage_fault() {
        let frame = ArmRegisterFrame {
            pc: 0x0800_1234,
            lr: 0x0800_5678,
            sp: 0x2000_1000,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
        };

        let report =
            HardFaultParser::parse_fault_frame(frame, 0x0200_0000, 0x4000_0000, None, None);

        assert!(report.decoded_cfsr.usage_divide_by_zero);
        assert!(report.decoded_hfsr.forced_hardfault);
        assert!(report.probable_cause.contains("divide by zero"));
    }

    #[test]
    fn test_precise_bus_fault_with_bfar() {
        let frame = ArmRegisterFrame {
            pc: 0x0800_2000,
            lr: 0x0800_3000,
            sp: 0x2000_0800,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
        };

        let report = HardFaultParser::parse_fault_frame(
            frame,
            0x0000_8200,
            0x4000_0000,
            None,
            Some(0x4001_1000), // Peripheral register base
        );

        assert!(report.decoded_cfsr.bus_precise_data_error);
        assert!(report.decoded_cfsr.bus_fault_address_valid);
        assert!(report.probable_cause.contains("0x40011000"));
    }
}
