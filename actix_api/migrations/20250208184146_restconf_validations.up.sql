-- Add up migration script here
CREATE TABLE IF NOT EXISTS restconf_validations (
  validation_datetime TIMESTAMP PRIMARY KEY,
  device_ip VARCHAR,
  uri_id INTEGER,
  status_code INTEGER,
  test_succesfull BOOLEAN,
  response_type VARCHAR
);