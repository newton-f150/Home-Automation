use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use tracing::{error, info, warn};

use crate::{
    db,
    models::{SensorPayload, SensorResponse, WsMessage},
    state::AppState,
};

pub fn spawn_mqtt_task(state: AppState) {
    tokio::spawn(async move {
        loop {
            match run_mqtt(state.clone()).await {
                Ok(_) => warn!("MQTT task exited cleanly – restarting"),
                Err(e) => error!("MQTT task error: {e} – restarting in 5 s"),
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}

async fn run_mqtt(state: AppState) -> anyhow::Result<()> {
    let host      = std::env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".into());
    let port: u16 = std::env::var("MQTT_PORT").unwrap_or_else(|_| "1883".into()).parse()?;
    let client_id = std::env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "esp32-backend".into());
    let sensor_topic = std::env::var("MQTT_SENSOR_TOPIC").unwrap_or_else(|_| "esp/sensor".into());

    let mut opts = MqttOptions::new(client_id, host, port);
    opts.set_keep_alive(std::time::Duration::from_secs(30));

    let (client, mut event_loop) = AsyncClient::new(opts, 128);

    // Store the publish client so routes can use it
    state.set_mqtt_client(client.clone()).await;

    client.subscribe(&sensor_topic, QoS::AtMostOnce).await?;
    info!("MQTT subscribed to {sensor_topic}");

    // Mark MQTT as connected in DB
    db::update_status(state.pool(), true, true).await.ok();
    let _ = state.ws_tx().send(WsMessage::Status {
        esp32_online: true,
        mqtt_connected: true,
    });

    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let topic   = p.topic.clone();
                let raw     = String::from_utf8_lossy(&p.payload).to_string();

                // Forward raw message to all WS clients
                let _ = state.ws_tx().send(WsMessage::Mqtt {
                    topic: topic.clone(),
                    payload: raw.clone(),
                });

                // Parse sensor payload and persist + broadcast
                if topic == sensor_topic {
                    match serde_json::from_str::<SensorPayload>(&raw) {
                        Ok(payload) => {
                            // Persist to DB
                            match db::insert_reading(state.pool(), &payload).await {
                                Ok(row) => {
                                    let response = SensorResponse::from(row);
                                    let _ = state.ws_tx().send(WsMessage::Sensor {
                                        payload: response,
                                    });
                                }
                                Err(e) => error!("DB insert failed: {e}"),
                            }
                        }
                        Err(e) => warn!("Could not parse sensor payload: {e} — raw: {raw}"),
                    }
                }
            }

            Ok(Event::Incoming(Incoming::Disconnect)) => {
                warn!("MQTT broker disconnected");
                db::update_status(state.pool(), false, false).await.ok();
                let _ = state.ws_tx().send(WsMessage::Status {
                    esp32_online: false,
                    mqtt_connected: false,
                });
                break;
            }

            Ok(_) => {}

            Err(e) => {
                error!("MQTT event loop error: {e}");
                db::update_status(state.pool(), false, false).await.ok();
                return Err(e.into());
            }
        }
    }

    Ok(())
}
