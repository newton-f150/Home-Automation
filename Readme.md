# Home Automation

## Overview

This project is a smart home automation system built using an ESP32 microcontroller and a Raspberry Pi or pc as the Edge Device. The system monitors environmental conditions, allows remote control of connected devices, and stores sensor data for future analysis in a Postgres Database.

## Features

* Real-time monitoring of sensor data
* Remote control of LEDs and servo motors
* Historical data storage and retrieval
* Web-based dashboard for visualization
* MQTT-based communication between devices
* Real-time updates between devices and dashboard

## Technology Stack

### Hardware

* ESP32
* Raspberry Pi
* LEDs
* Servo Motor
* Environmental Sensors

### Backend

* Rust
* Axum
* Tokio
* MQTT Client
* PostgreSQL
* SQLx

### Frontend

* HTML
* CSS
* JavaScript

### Communication

* MQTT
* WebSockets

## System Architecture

### ESP32

The ESP32 collects sensor readings and publishes them to the MQTT broker. It also subscribes to control topics to receive commands from the dashboard.

### Raspberry Pi / Server

The server is built with Rust and is responsible for:

* Receiving sensor data from MQTT topics
* Storing data in PostgreSQL
* Managing device control commands
* Serving the web dashboard
* Broadcasting real-time updates through WebSockets

## Project Structure

```text
HomeAutomation/
├── mqqt_client/
│   ├── .pio/
│   ├── .vscode/
│   ├── include/
│   ├── lib/
│   ├── src/
│   ├── test/
│   └── platformio.ini
│
├── server/
│   ├── src/
│   ├── target/
│   ├── Cargo.toml
│   └── Cargo.lock
|
|──Index.html
|──simulation.html
│
└── README.md
```

## Installation

### Clone the Repository

```bash
git clone https://github.com/newton-f150/Home-Automation.git
cd Home-Automation
```

### Configure PostgreSQL

Create a PostgreSQL database.

```sql
CREATE DATABASE homeAutomation;
```

Run Database Migration.
```sql
\i path/to/migrations/001_initial.sql
```

Insert Sample Data into the Database.
```sql
\i path/to/migrations/data.sql
```
Update your environment variables.

```env
DATABASE_URL=postgres://username:password@localhost/home_automation
MQTT_BROKER=mqtt://localhost:1883
```

### Build the Rust Backend

```bash
cargo build --release
```

### Run the Backend Server

```bash
cargo run
```

### Upload ESP32 Firmware

Open the Arduino IDE, configure Wi-Fi and MQTT settings, then upload the firmware to the ESP32.

## Dashboard Features

* Live sensor monitoring
* Device control panel
* Historical data visualization
* Custom date-range queries
* Real-time device status updates
