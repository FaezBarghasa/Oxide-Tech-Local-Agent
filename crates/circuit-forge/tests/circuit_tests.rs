use circuit_forge::{
    CircuitBuilder, CircuitCommand, CircuitScript, calculate_grid_layout, run_erc,
    serialize_to_kicad_sch,
};

#[test]
fn test_fluent_builder_and_graph_topology() {
    let mut builder = CircuitBuilder::new();

    // 1. Add components for a 3.3V LDO regulator circuit
    builder
        .add_component_with_footprint(
            "U1",
            "Regulator_Linear:AP2112K-3.3",
            "3.3V LDO",
            "Package_TO_SOT_SMD:SOT-23-5",
        )
        .add_component_with_footprint("C1", "Device:C", "1uF", "Capacitor_SMD:C_0805_2012Metric")
        .add_component_with_footprint("C2", "Device:C", "2.2uF", "Capacitor_SMD:C_0805_2012Metric")
        .add_component_with_footprint("R1", "Device:R", "100k", "Resistor_SMD:R_0805_2012Metric");

    // 2. Connect input power rail & input decoupling capacitor
    builder
        .connect("U1", "VIN", "C1", "1", "VIN")
        .expect("Failed to connect U1.VIN to C1.1");
    builder
        .connect("U1", "EN", "R1", "1", "VIN")
        .expect("Failed to connect U1.EN to R1.1");

    // 3. Connect output power rail & output capacitor
    builder
        .connect("U1", "VOUT", "C2", "1", "+3.3V")
        .expect("Failed to connect U1.VOUT to C2.1");

    // 4. Connect common ground
    builder
        .connect("U1", "GND", "C1", "2", "GND")
        .expect("Failed to connect U1.GND to C1.2");
    builder
        .connect_net("C2", "2", "GND")
        .expect("Failed to connect C2.2 to GND");

    let graph = builder.build();

    // Verify nodes: 4 components + 3 nets (VIN, +3.3V, GND) = 7 nodes
    assert_eq!(graph.node_count(), 7);

    // Verify ERC on complete circuit
    let report = run_erc(&graph);
    assert!(
        report.is_clean(),
        "Expected clean ERC report, got errors: {:?}",
        report.errors
    );
    assert!(
        !report.has_warnings(),
        "Expected no warnings, got: {:?}",
        report.warnings
    );
}

#[test]
fn test_json_dsl_command_execution() {
    let commands = vec![
        CircuitCommand::AddComponent {
            ref_des: "R1".to_string(),
            lib_id: "Device:R".to_string(),
            value: "10k".to_string(),
            footprint: Some("Resistor_SMD:R_0805_2012Metric".to_string()),
        },
        CircuitCommand::AddComponent {
            ref_des: "D1".to_string(),
            lib_id: "Device:LED".to_string(),
            value: "Green".to_string(),
            footprint: Some("LED_SMD:LED_0805_2012Metric".to_string()),
        },
        CircuitCommand::Connect {
            comp1: "R1".to_string(),
            pin1: "2".to_string(),
            comp2: "D1".to_string(),
            pin2: "1".to_string(),
            net_name: "LED_ANODE".to_string(),
        },
        CircuitCommand::ConnectNet {
            comp: "R1".to_string(),
            pin: "1".to_string(),
            net_name: "+3.3V".to_string(),
        },
        CircuitCommand::ConnectNet {
            comp: "D1".to_string(),
            pin: "2".to_string(),
            net_name: "GND".to_string(),
        },
    ];

    let script = CircuitScript { commands };
    let json_dsl = serde_json::to_string_pretty(&script).expect("Failed to serialize script");

    let graph = CircuitBuilder::from_json(&json_dsl).expect("Failed to parse and build from JSON");
    assert_eq!(graph.node_count(), 5); // 2 components + 3 nets
}

#[test]
fn test_erc_floating_net_detection() {
    let mut builder = CircuitBuilder::new();
    builder.add_component("R1", "Device:R", "10k");

    // Connect R1 pin 1 to a net, but don't connect anything else to this net
    builder
        .connect_net("R1", "1", "FLOATING_SIGNAL")
        .expect("Failed to connect net");

    let graph = builder.build();
    let report = run_erc(&graph);

    assert!(
        !report.is_clean(),
        "ERC should have caught floating net error"
    );
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.contains("FLOATING_SIGNAL") && e.contains("floating")),
        "Error message did not match expected floating net format: {:?}",
        report.errors
    );
}

#[test]
fn test_erc_isolated_component() {
    let mut builder = CircuitBuilder::new();
    builder.add_component("U1", "MCU_ST:STM32F401CCU6", "STM32F401");

    let graph = builder.build();
    let report = run_erc(&graph);

    assert!(
        !report.is_clean(),
        "ERC should have caught isolated component error"
    );
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.contains("U1") && e.contains("no pin connections")),
        "Error message did not match expected isolated component: {:?}",
        report.errors
    );
}

#[test]
fn test_erc_missing_decoupling_cap() {
    let mut builder = CircuitBuilder::new();
    builder
        .add_component("U1", "Regulator_Linear:AP2112K-3.3", "3.3V")
        .add_component("R1", "Device:R", "1k");

    builder
        .connect("U1", "VIN", "R1", "1", "VCC")
        .expect("Failed to connect");
    builder
        .connect("U1", "GND", "R1", "2", "GND")
        .expect("Failed to connect");

    let graph = builder.build();
    let report = run_erc(&graph);

    assert!(
        report.has_warnings(),
        "Expected ERC warning for missing decoupling cap on VCC"
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("VCC") && w.contains("decoupling capacitor")),
        "Warning message did not match expected decoupling check: {:?}",
        report.warnings
    );
}

#[test]
fn test_kicad_s_expression_serialization() {
    let mut builder = CircuitBuilder::new();
    builder
        .add_component_with_footprint(
            "U1",
            "Regulator_Linear:AP2112K-3.3",
            "3.3V LDO",
            "Package_TO_SOT_SMD:SOT-23-5",
        )
        .add_component_with_footprint("C1", "Device:C", "10uF", "Capacitor_SMD:C_0805_2012Metric");

    builder
        .connect("U1", "VIN", "C1", "1", "VIN")
        .expect("Failed to connect");
    builder
        .connect("U1", "GND", "C1", "2", "GND")
        .expect("Failed to connect");

    let graph = builder.build();

    let layout = calculate_grid_layout(&graph);
    assert_eq!(layout.len(), 2);

    let sch_sexpr = serialize_to_kicad_sch(&graph);

    // Verify KiCad 8 format markers
    assert!(sch_sexpr.starts_with("(kicad_sch\n"));
    assert!(sch_sexpr.contains("(version 20231120)"));
    assert!(sch_sexpr.contains("(generator \"NexusForge_CircuitForge\")"));
    assert!(sch_sexpr.contains("(generator_version \"8.0\")"));
    assert!(sch_sexpr.contains("(symbol (lib_id \"Regulator_Linear:AP2112K-3.3\")"));
    assert!(sch_sexpr.contains("(property \"Reference\" \"U1\""));
    assert!(sch_sexpr.contains("(property \"Value\" \"3.3V LDO\""));
    assert!(sch_sexpr.contains("(property \"Footprint\" \"Package_TO_SOT_SMD:SOT-23-5\""));
    assert!(sch_sexpr.contains("(wire (pts"));
    assert!(sch_sexpr.contains("(label \"VIN\""));
    assert!(sch_sexpr.contains("(label \"GND\""));
    assert!(sch_sexpr.ends_with(")\n"));
}
