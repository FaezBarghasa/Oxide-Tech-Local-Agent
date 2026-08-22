-- migrations/001_init_schema.surql
BEGIN;

-- Projects
DEFINE TABLE project SCHEMAFULL;
DEFINE FIELD name ON TABLE project TYPE string;
DEFINE FIELD mcu_type ON TABLE project TYPE string;
DEFINE FIELD target_triple ON TABLE project TYPE string OPTIONAL;
DEFINE FIELD created_at ON TABLE project TYPE datetime DEFAULT time::now();

-- Components
DEFINE TABLE component SCHEMAFULL;
DEFINE FIELD ref_des ON TABLE component TYPE string UNIQUE;
DEFINE FIELD value ON TABLE component TYPE string;
DEFINE FIELD footprint ON TABLE component TYPE string;
DEFINE FIELD created_at ON TABLE component TYPE datetime;

-- Relationships
DEFINE TABLE contains SCHEMAFULL;
DEFINE FIELD in ON TABLE contains TYPE record<project>;
DEFINE FIELD out ON TABLE contains TYPE record<component>;

-- Indexes for fast queries
DEFINE INDEX project_name_idx ON TABLE project COLUMNS name UNIQUE;
DEFINE INDEX component_ref_idx ON TABLE component COLUMNS ref_des UNIQUE;

COMMIT;
