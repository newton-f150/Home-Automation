-- migrations/001_initial.sql
-- Run with: sqlx migrate run

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Sensor readings table
CREATE TABLE IF NOT EXISTS sensor_readings (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    temperature DOUBLE PRECISION NOT NULL,
    pressure    DOUBLE PRECISION NOT NULL,
    air_quality DOUBLE PRECISION NOT NULL,
    light       DOUBLE PRECISION NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for time-series queries
CREATE INDEX idx_sensor_readings_recorded_at
    ON sensor_readings (recorded_at DESC);

-- Device commands log
CREATE TABLE IF NOT EXISTS command_log (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    command     TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'sent',  -- sent | ok | failed
    sent_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Device status table (single row, upserted)
CREATE TABLE IF NOT EXISTS device_status (
    id              SERIAL PRIMARY KEY,
    esp32_online    BOOLEAN NOT NULL DEFAULT FALSE,
    mqtt_connected  BOOLEAN NOT NULL DEFAULT FALSE,
    last_seen       TIMESTAMPTZ,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed one status row
INSERT INTO device_status (id, esp32_online, mqtt_connected)
VALUES (1, FALSE, FALSE)
ON CONFLICT (id) DO NOTHING;



