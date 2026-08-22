-- migrations/004_cloud_training_capture.surql
BEGIN;

REMOVE TABLE cloud_training_sample;

DEFINE TABLE cloud_training_sample SCHEMAFULL;
DEFINE FIELD tenant_id ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD user_id ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD task_type ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD prompt ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD cloud_provider ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD cloud_response ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD cloud_reasoning ON TABLE cloud_training_sample TYPE option<string>;
DEFINE FIELD validation ON TABLE cloud_training_sample TYPE object;
DEFINE FIELD confidence ON TABLE cloud_training_sample TYPE float;
DEFINE FIELD cost_usd ON TABLE cloud_training_sample TYPE float;
DEFINE FIELD user_outcome ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD created_at ON TABLE cloud_training_sample TYPE datetime DEFAULT time::now();

COMMIT;
