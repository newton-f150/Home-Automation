use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── DB row ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SensorReading {
    pub id:          Uuid,
    pub temperature: f64,
    pub pressure:    f64,
    pub air_quality: f64,
    pub light:       f64,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DeviceStatus {
    pub esp32_online:   bool,
    pub mqtt_connected: bool,
    pub last_seen:      Option<DateTime<Utc>>,
    pub updated_at:     DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CommandLog {
    pub id:      Uuid,
    pub command: String,
    pub status:  String,
    pub sent_at: DateTime<Utc>,
}

// ─── API request / response DTOs ─────────────────────────────────────────────

/// Incoming sensor payload from MQTT or REST (ESP32 publishes this)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SensorPayload {
    pub temperature: Option<f64>,
    pub pressure:    Option<f64>,
    #[serde(rename = "airQuality")]
    pub air_quality: Option<f64>,
    pub light:       Option<f64>,
}

/// What the frontend `GET /api/sensors` expects
#[derive(Debug, Clone, Serialize)]
pub struct SensorResponse {
    pub temperature: f64,
    pub pressure:    f64,
    #[serde(rename = "airQuality")]
    pub air_quality: f64,
    pub light:       f64,
    pub recorded_at: DateTime<Utc>,
}

impl From<SensorReading> for SensorResponse {
    fn from(r: SensorReading) -> Self {
        Self {
            temperature: r.temperature,
            pressure:    r.pressure,
            air_quality: r.air_quality,
            light:       r.light,
            recorded_at: r.recorded_at,
        }
    }
}

/// POST /api/control body
#[derive(Debug, Deserialize)]
pub struct ControlRequest {
    pub command: String,
}

/// POST /api/control response
#[derive(Debug, Serialize)]
pub struct ControlResponse {
    pub ok:      bool,
    pub command: String,
}

/// WebSocket push envelope – mirrors the frontend handler
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    Sensor  { payload: SensorResponse },
    Mqtt    { topic: String, payload: String },
    Status  { esp32_online: bool, mqtt_connected: bool },
}
