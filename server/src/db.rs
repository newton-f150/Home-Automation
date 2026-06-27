use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use uuid::Uuid;

use crate::models::{DeviceStatus, SensorPayload, SensorReading};

pub async fn create_pool() -> sqlx::Result<PgPool> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new().max_connections(10).connect(&url).await
}

// ─── Internal row types (no query macros, fully runtime) ─────────────────────

#[derive(FromRow)]
struct SensorRow {
    id:          Uuid,
    temperature: f64,
    pressure:    f64,
    air_quality: f64,
    light:       f64,
    recorded_at: DateTime<Utc>,
}

impl From<SensorRow> for SensorReading {
    fn from(r: SensorRow) -> Self {
        SensorReading {
            id:          r.id,
            temperature: r.temperature,
            pressure:    r.pressure,
            air_quality: r.air_quality,
            light:       r.light,
            recorded_at: r.recorded_at,
        }
    }
}

#[derive(FromRow)]
struct StatusRow {
    esp32_online:   bool,
    mqtt_connected: bool,
    last_seen:      Option<DateTime<Utc>>,
    updated_at:     DateTime<Utc>,
}

impl From<StatusRow> for DeviceStatus {
    fn from(r: StatusRow) -> Self {
        DeviceStatus {
            esp32_online:   r.esp32_online,
            mqtt_connected: r.mqtt_connected,
            last_seen:      r.last_seen,
            updated_at:     r.updated_at,
        }
    }
}

// ─── Sensor readings ──────────────────────────────────────────────────────────

pub async fn insert_reading(pool: &PgPool, p: &SensorPayload) -> sqlx::Result<SensorReading> {
    let row = sqlx::query_as::<_, SensorRow>(
        r#"
        INSERT INTO sensor_readings (temperature, pressure, air_quality, light)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(p.temperature.unwrap_or(0.0))
    .bind(p.pressure.unwrap_or(0.0))
    .bind(p.air_quality.unwrap_or(0.0))
    .bind(p.light.unwrap_or(0.0))
    .fetch_one(pool)
    .await?;

    Ok(row.into())
}

pub async fn get_latest_reading(pool: &PgPool) -> sqlx::Result<Option<SensorReading>> {
    let row = sqlx::query_as::<_, SensorRow>(
        r#"
        SELECT * FROM sensor_readings
        ORDER BY recorded_at DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Into::into))
}

pub async fn get_reading_history(pool: &PgPool, limit: i64) -> sqlx::Result<Vec<SensorReading>> {
    let rows = sqlx::query_as::<_, SensorRow>(
        r#"
        SELECT * FROM sensor_readings
        ORDER BY recorded_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

// ─── Device status ────────────────────────────────────────────────────────────

pub async fn get_status(pool: &PgPool) -> sqlx::Result<DeviceStatus> {
    let row = sqlx::query_as::<_, StatusRow>(
        r#"
        SELECT esp32_online, mqtt_connected, last_seen, updated_at
        FROM device_status
        WHERE id = 1
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.into())
}

pub async fn update_status(
    pool: &PgPool,
    esp32_online: bool,
    mqtt_connected: bool,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE device_status
        SET esp32_online   = $1,
            mqtt_connected = $2,
            last_seen      = CASE WHEN $1::bool THEN NOW() ELSE last_seen END,
            updated_at     = NOW()
        WHERE id = 1
        "#,
    )
    .bind(esp32_online)
    .bind(mqtt_connected)
    .execute(pool)
    .await?;

    Ok(())
}

// ─── Command log ─────────────────────────────────────────────────────────────

pub async fn log_command(pool: &PgPool, command: &str, status: &str) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO command_log (command, status)
        VALUES ($1, $2)
        "#,
    )
    .bind(command)
    .bind(status)
    .execute(pool)
    .await?;

    Ok(())
}
