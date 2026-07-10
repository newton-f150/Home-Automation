# ESP32 Backend — Rust + Axum + PostgreSQL + MQTT

A production-ready backend for the ESP32 Sensor Dashboard.

## Architecture

```
ESP32  ──MQTT──►  rumqttc subscriber  ──►  PostgreSQL (sensor_readings)
                        │
                        └──►  broadcast channel  ──►  WebSocket clients (frontend)

Frontend  ──REST──►  Axum routes  ──►  PostgreSQL / MQTT publish
```

## Stack

| Layer     | Crate                |
|-----------|----------------------|
| HTTP      | axum 0.7             |
| Async     | tokio                |
| Database  | sqlx + PostgreSQL    |
| MQTT      | rumqttc              |
| WebSocket | axum built-in        |
| CORS      | tower-http           |

---

## Quick Start

### 1. Prerequisites

```bash
# Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# sqlx-cli for migrations
cargo install sqlx-cli --no-default-features --features postgres

# PostgreSQL + an MQTT broker (e.g. Mosquitto)
```

### 2. Database setup

```bash
createdb esp32_db

# Apply migration
sqlx migrate run --database-url postgres://user:password@localhost/esp32_db

# Or manually:
psql esp32_db < migrations/001_initial.sql
```

### 3. Configuration

```bash
cp .env.example .env
# Edit .env with your DATABASE_URL, MQTT_HOST, etc.
```

### 4. Run

```bash
cargo run --release
# Server starts on http://0.0.0.0:3000
```

---

## API Reference

### `GET /api/sensors`
Returns the most recent sensor reading.

```json
{
  "temperature": 24.5,
  "pressure": 1013.2,
  "airQuality": 42.0,
  "light": 320.0,
  "recorded_at": "2024-01-15T10:30:00Z"
}
```

### `GET /api/sensors/history?limit=30`
Returns up to `limit` (max 1000) recent readings, newest first.

### `POST /api/control`
Publishes a command to the `esp/cmd` MQTT topic.

**Body:**
```json
{ "command": "LIGHT_ON" }
```

**Allowed commands:** `LIGHT_ON`, `LIGHT_OFF`, `FAN_ON`, `FAN_OFF`, `RESTART`, `STOP`

**Response:**
```json
{ "ok": true, "command": "LIGHT_ON" }
```

### `GET /api/status`
Returns device + MQTT connection status.

```json
{
  "esp32_online": true,
  "mqtt_connected": true,
  "last_seen": "2024-01-15T10:30:00Z"
}
```

### `GET /api/ws`  (WebSocket)
Upgrade to WebSocket to receive real-time push messages.

**Sensor update:**
```json
{ "type": "sensor", "payload": { "temperature": 24.5, ... } }
```

**Raw MQTT message:**
```json
{ "type": "mqtt", "topic": "esp/sensor", "payload": "{...}" }
```

**Status change:**
```json
{ "type": "status", "esp32_online": true, "mqtt_connected": true }
```

---

## ESP32 MQTT Payload Format

Publish JSON to `esp/sensor` topic:

```json
{
  "temperature": 24.5,
  "pressure": 1013.2,
  "airQuality": 42,
  "light": 320
}
```

All fields are optional; missing ones default to `0.0`.

---

## Project Structure

```
esp32-backend/
├── Cargo.toml
├── .env.example
├── migrations/
│   └── 001_initial.sql
└── src/
    ├── main.rs        # Server bootstrap, router
    ├── state.rs       # AppState (DB pool + WS broadcast + MQTT client)
    ├── models.rs      # DB rows + API DTOs
    ├── db.rs          # sqlx query helpers
    ├── mqtt.rs        # rumqttc subscriber task
    ├── ws.rs          # WebSocket upgrade handler
    ├── error.rs       # AppError → HTTP response
    └── routes/
        ├── mod.rs
        ├── sensors.rs
        ├── control.rs
        └── status.rs
```
