# hardfault_parser

## Classs

- [ArmRegisterFrame](ArmRegisterFrame.md) — [derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
- [DecodedCfsr](DecodedCfsr.md) — ARM Cortex-M Configurable Fault Status Register (CFSR) Flags
- [DecodedHfsr](DecodedHfsr.md) — ARM Cortex-M HardFault Status Register (HFSR) Flags
- [HardFaultParser](HardFaultParser.md)
- [HardFaultReport](HardFaultReport.md) — Structured Cortex-M Crash and Panic Report

## Functions

- [parse_fault_frame](parse_fault_frame.md) — Decode raw ARM register status values into a structured fault diagnostic
- [parse_fault_frame](parse_fault_frame_1.md) — Decode raw ARM register status values into a structured fault diagnostic
- [test_divide_by_zero_usage_fault](test_divide_by_zero_usage_fault.md) — [test]
- [test_precise_bus_fault_with_bfar](test_precise_bus_fault_with_bfar.md) — [test]
