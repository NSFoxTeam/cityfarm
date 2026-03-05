CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS sensor_readings (
    time        TIMESTAMPTZ NOT NULL,
    sensor_type TEXT NOT NULL,
    value       DOUBLE PRECISION NOT NULL,
    unit        TEXT NOT NULL,
    level       SMALLINT NOT NULL DEFAULT 0
);

SELECT create_hypertable('sensor_readings', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_sensor_readings_type_time
    ON sensor_readings (sensor_type, time DESC);
