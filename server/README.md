# ESP32 Backend — Rust + Axum + PostgreSQL + MQTT

A production-ready backend powering an ESP32 Home Automation Dashboard with real-time sensor monitoring, device control, WebSockets, MQTT integration, and PostgreSQL persistence.

---

# Dashboard Preview

## Top Dashboard

![Top Dashboard](images/top-dashboard.png)

## Bottom Dashboard

![Bottom Dashboard](images/bottom-dashboard.png)

---

## Architecture

```text
ESP32  ──MQTT──►  rumqttc subscriber  ──►  PostgreSQL (sensor_readings)
                        │
                        └──►  broadcast channel  ──►  WebSocket clients (frontend)

Frontend  ──REST──►  Axum routes  ──►  PostgreSQL / MQTT publish
```

---

## Technology Stack

| Layer | Technology |
|--------|------------|
| Language | Rust |
| HTTP API | Axum 0.7 |
| Async Runtime | Tokio |
| Database | PostgreSQL + SQLx |
| MQTT | rumqttc |
| WebSockets | Axum WebSocket |
| CORS | tower-http |

---

## Features

- Real-time ESP32 sensor streaming
- MQTT publish/subscribe
- PostgreSQL storage
- REST API
- WebSocket live updates
- Device control endpoints
- Connection status monitoring
- Responsive dashboard UI

---

# Quick Start

## 1. Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Install PostgreSQL

# Install an MQTT broker
# Example: Mosquitto
```

---

## 2. Database Setup

```bash
createdb esp32_db

sqlx migrate run --database-url postgres://user:password@localhost/esp32_db

# or

psql esp32_db < migrations/001_initial.sql
```

---

## 3. Configuration

```bash
cp .env.example .env
```

Configure:

- DATABASE_URL
- MQTT_HOST
- MQTT_PORT
- MQTT_USERNAME
- MQTT_PASSWORD

---

## 4. Run

```bash
cargo run --release
```

Server:

```
http://0.0.0.0:3000
```

---

# REST API

## GET `/api/sensors`

Returns the latest sensor reading.

```json
{
  "temperature": 24.5,
  "pressure": 1013.2,
  "airQuality": 42,
  "light": 320,
  "recorded_at": "2024-01-15T10:30:00Z"
}
```

---

## GET `/api/sensors/history?limit=30`

Returns the latest sensor history.

Maximum limit:

```
1000
```

---

## POST `/api/control`

Publishes commands to the ESP32 via MQTT.

Request

```json
{
  "command": "LIGHT_ON"
}
```

Allowed commands

```
LIGHT_ON
LIGHT_OFF
FAN_ON
FAN_OFF
RESTART
STOP
```

Response

```json
{
  "ok": true,
  "command": "LIGHT_ON"
}
```

---

## GET `/api/status`

Returns system status.

```json
{
  "esp32_online": true,
  "mqtt_connected": true,
  "last_seen": "2024-01-15T10:30:00Z"
}
```

---

## GET `/api/ws`

WebSocket endpoint for real-time updates.

### Sensor Update

```json
{
  "type": "sensor",
  "payload": {
    "temperature": 24.5
  }
}
```

### MQTT Message

```json
{
  "type": "mqtt",
  "topic": "esp/sensor",
  "payload": "{...}"
}
```

### Status Update

```json
{
  "type": "status",
  "esp32_online": true,
  "mqtt_connected": true
}
```

---

# ESP32 MQTT Payload

Publish JSON to:

```
esp/sensor
```

```json
{
  "temperature": 24.5,
  "pressure": 1013.2,
  "airQuality": 42,
  "light": 320
}
```

Missing fields default to `0.0`.

---

## Project Structure

```text
homeautomation/
├── images/
│   ├── top-dashboard.png
│   ├── bottom-dashboard.png
│   └── rust.png
│
├── mqtt_client/
│   ├── .env
│   ├── .gitignore
│   ├── platformio.ini
│   ├── include/
│   │   └── README
│   ├── lib/
│   │   └── README
│   ├── src/
│   │   └── main.cpp
│   ├── test/
│   │   └── README
│   └── .vscode/
│       ├── c_cpp_properties.json
│       ├── extensions.json
│       └── launch.json
│
├── server/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── .env.example
│   ├── migrations/
│   │   └── 001_initial.sql
│   ├── src/
│   │   ├── main.rs
│   │   ├── state.rs
│   │   ├── models.rs
│   │   ├── db.rs
│   │   ├── mqtt.rs
│   │   ├── ws.rs
│   │   ├── error.rs
│   │   └── routes/
│   │       ├── mod.rs
│   │       ├── sensors.rs
│   │       ├── control.rs
│   │       └── status.rs
│
├── index.html
├── simulation.html
├── test.html
├── README.md
└── .gitignore
```

# Built With

- Rust
- Axum
- Tokio
- SQLx
- PostgreSQL
- MQTT (rumqttc)
- WebSockets

---

## Rust Backend

<p align="center">
  <img src="images/rust.png" width="180" alt="Rust Logo">
</p>

---

## License

MIT License