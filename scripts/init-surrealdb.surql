-- ============================================================================
-- Oxide-Tech-Local-Agent: Production SurrealDB Declarative Schema Initializer
-- Namespace: oxide_agent | Database: master
-- ============================================================================

DEFINE NAMESPACE oxide_agent;
USE NS oxide_agent;

DEFINE DATABASE master;
USE DB master;

-- ----------------------------------------------------------------------------
-- Table: project (Multi-domain systems engineering workspace)
-- ----------------------------------------------------------------------------
DEFINE TABLE project SCHEMAFULL;
DEFINE FIELD name ON TABLE project TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD version ON TABLE project TYPE string DEFAULT "0.1.0";
DEFINE FIELD domain ON TABLE project TYPE string ASSERT $value IN ["firmware_ide", "oxide_eda", "oxide_3d", "cross_domain_verifier", "mesh_net"];
DEFINE FIELD root_path ON TABLE project TYPE string;
DEFINE FIELD config ON TABLE project TYPE object;
DEFINE FIELD created_at ON TABLE project TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE project TYPE datetime DEFAULT time::now();
DEFINE INDEX project_name_idx ON TABLE project COLUMNS name UNIQUE;

-- ----------------------------------------------------------------------------
-- Table: netlist (Circuit & PCB schematic connectivity)
-- ----------------------------------------------------------------------------
DEFINE TABLE netlist SCHEMAFULL;
DEFINE FIELD project_id ON TABLE netlist TYPE record<project>;
DEFINE FIELD nets ON TABLE netlist TYPE array<object>;
DEFINE FIELD components ON TABLE netlist TYPE array<object>;
DEFINE FIELD impedance_profiles ON TABLE netlist TYPE array<object>;
DEFINE FIELD created_at ON TABLE netlist TYPE datetime DEFAULT time::now();
DEFINE INDEX netlist_project_idx ON TABLE netlist COLUMNS project_id;

-- ----------------------------------------------------------------------------
-- Table: component (Electronic / Mechanical parametric components)
-- ----------------------------------------------------------------------------
DEFINE TABLE component SCHEMAFULL;
DEFINE FIELD project_id ON TABLE component TYPE record<project>;
DEFINE FIELD ref_des ON TABLE component TYPE string;
DEFINE FIELD package ON TABLE component TYPE string;
DEFINE FIELD x_mm ON TABLE component TYPE float;
DEFINE FIELD y_mm ON TABLE component TYPE float;
DEFINE FIELD rotation_deg ON TABLE component TYPE float DEFAULT 0.0;
DEFINE FIELD layer ON TABLE component TYPE string DEFAULT "F.Cu";
DEFINE FIELD thermal_watts ON TABLE component TYPE float DEFAULT 0.0;
DEFINE FIELD created_at ON TABLE component TYPE datetime DEFAULT time::now();
DEFINE INDEX component_ref_idx ON TABLE component COLUMNS project_id, ref_des UNIQUE;

-- ----------------------------------------------------------------------------
-- Table: episode (Autonomous agent trajectory & reasoning rollout)
-- ----------------------------------------------------------------------------
DEFINE TABLE episode SCHEMAFULL;
DEFINE FIELD session_id ON TABLE episode TYPE string;
DEFINE FIELD user_intent ON TABLE episode TYPE string;
DEFINE FIELD hunks ON TABLE episode TYPE array<object>;
DEFINE FIELD reward ON TABLE episode TYPE float DEFAULT 0.0;
DEFINE FIELD status ON TABLE episode TYPE string ASSERT $value IN ["pending", "in_progress", "committed", "rejected", "failed"];
DEFINE FIELD created_at ON TABLE episode TYPE datetime DEFAULT time::now();
DEFINE INDEX episode_session_idx ON TABLE episode COLUMNS session_id;

-- ----------------------------------------------------------------------------
-- Table: assertion (Z3 SMT formal verification theorems & proofs)
-- ----------------------------------------------------------------------------
DEFINE TABLE assertion SCHEMAFULL;
DEFINE FIELD domain ON TABLE assertion TYPE string;
DEFINE FIELD smt_formula ON TABLE assertion TYPE string;
DEFINE FIELD proved ON TABLE assertion TYPE bool;
DEFINE FIELD proof_time_ms ON TABLE assertion TYPE float;
DEFINE FIELD solver ON TABLE assertion TYPE string DEFAULT "z3-4.13";
DEFINE FIELD created_at ON TABLE assertion TYPE datetime DEFAULT time::now();
DEFINE INDEX assertion_domain_idx ON TABLE assertion COLUMNS domain;

-- ----------------------------------------------------------------------------
-- Table: simulation_run (Thermal GNN / SPICE / FEA simulation metrics)
-- ----------------------------------------------------------------------------
DEFINE TABLE simulation_run SCHEMAFULL;
DEFINE FIELD project_id ON TABLE simulation_run TYPE record<project>;
DEFINE FIELD kind ON TABLE simulation_run TYPE string ASSERT $value IN ["thermal_gnn", "spice_transient", "cad_collision", "drc_erc"];
DEFINE FIELD max_temp_c ON TABLE simulation_run TYPE float;
DEFINE FIELD passes_threshold ON TABLE simulation_run TYPE bool;
DEFINE FIELD metrics ON TABLE simulation_run TYPE object;
DEFINE FIELD created_at ON TABLE simulation_run TYPE datetime DEFAULT time::now();
DEFINE INDEX sim_project_idx ON TABLE simulation_run COLUMNS project_id, kind;

-- ----------------------------------------------------------------------------
-- Table: audit_log (Zero-trust HITL intervention & security logs)
-- ----------------------------------------------------------------------------
DEFINE TABLE audit_log SCHEMAFULL;
DEFINE FIELD operator ON TABLE audit_log TYPE string;
DEFINE FIELD action ON TABLE audit_log TYPE string;
DEFINE FIELD risk_tier ON TABLE audit_log TYPE string ASSERT $value IN ["Low", "Medium", "Critical"];
DEFINE FIELD signature ON TABLE audit_log TYPE string;
DEFINE FIELD created_at ON TABLE audit_log TYPE datetime DEFAULT time::now();
DEFINE INDEX audit_operator_idx ON TABLE audit_log COLUMNS operator;

-- ----------------------------------------------------------------------------
-- Table: verification_proof (Distributed Transaction evidence bundles)
-- ----------------------------------------------------------------------------
DEFINE TABLE verification_proof SCHEMAFULL;
DEFINE FIELD dtx_id ON TABLE verification_proof TYPE string;
DEFINE FIELD kind ON TABLE verification_proof TYPE string;
DEFINE FIELD is_formal ON TABLE verification_proof TYPE bool;
DEFINE FIELD passed ON TABLE verification_proof TYPE bool;
DEFINE FIELD details ON TABLE verification_proof TYPE object;
DEFINE FIELD content_hash ON TABLE verification_proof TYPE string;
DEFINE FIELD created_at ON TABLE verification_proof TYPE datetime DEFAULT time::now();
DEFINE INDEX proof_dtx_idx ON TABLE verification_proof COLUMNS dtx_id;
