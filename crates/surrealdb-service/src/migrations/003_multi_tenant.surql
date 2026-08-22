-- migrations/003_multi_tenant.surql
BEGIN;

DEFINE TABLE tenant SCHEMAFULL;
DEFINE FIELD name ON TABLE tenant TYPE string;
DEFINE FIELD slug ON TABLE tenant TYPE string ASSERT string::matches::strict($value, /^[a-z0-9-]+$/);
DEFINE FIELD plan ON TABLE tenant TYPE string ASSERT $value INSIDE ["free", "pro", "enterprise"];
DEFINE FIELD quotas ON TABLE tenant TYPE object;
DEFINE FIELD quotas.gpu_hours ON TABLE tenant TYPE int DEFAULT 100;
DEFINE FIELD quotas.tokens_monthly ON TABLE tenant TYPE int DEFAULT 10000000;

DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password_hash ON TABLE user TYPE string;
DEFINE FIELD role ON TABLE user TYPE string;

DEFINE TABLE membership SCHEMAFULL;
DEFINE FIELD in ON TABLE membership TYPE record<user>;
DEFINE FIELD out ON TABLE membership TYPE record<tenant>;
DEFINE FIELD role ON TABLE membership TYPE string;

DEFINE TABLE api_key SCHEMAFULL;
DEFINE FIELD user ON TABLE api_key TYPE record<user>;
DEFINE FIELD tenant ON TABLE api_key TYPE record<tenant>;
DEFINE FIELD key_hash ON TABLE api_key TYPE string;
DEFINE FIELD scopes ON TABLE api_key TYPE array<string>;
DEFINE FIELD expires_at ON TABLE api_key TYPE option<datetime>;

-- Row-level security for project table
REMOVE TABLE project;
DEFINE TABLE project SCHEMAFULL
    PERMISSIONS
        FOR select WHERE tenant_id = $auth.tenant_id,
        FOR create WHERE $auth.role INSIDE ["owner", "admin", "engineer"];

COMMIT;
