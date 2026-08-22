-- migrations/002_cloud_training.surql
BEGIN;

DEFINE TABLE cloud_training_sample SCHEMAFULL;
DEFINE FIELD prompt ON TABLE cloud_training_sample TYPE string;
DEFINE FIELD context ON TABLE cloud_training_sample TYPE object;
DEFINE FIELD cloud_response ON TABLE cloud_training_sample TYPE object;
DEFINE FIELD validation ON TABLE cloud_training_sample TYPE object;
DEFINE FIELD outcome ON TABLE cloud_training_sample TYPE bool;
DEFINE FIELD created_at ON TABLE cloud_training_sample TYPE datetime DEFAULT time::now();

COMMIT;
