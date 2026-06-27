use axum::{extract::State, Json};
use rumqttc::QoS;
use tracing::{info, warn};

use crate::{
    db,
    error::{ApiResult, AppError},
    models::{ControlRequest, ControlResponse},
    state::AppState,
};

const ALLOWED_COMMANDS: &[&str] = &[
    "LIGHT_ON",
    "LIGHT_OFF",
    "FAN_ON",
    "FAN_OFF",
    "RESTART",
    "STOP",
];

/// POST /api/control  body: { "command": "LIGHT_ON" }
pub async fn send_command(
    State(state): State<AppState>,
    Json(body): Json<ControlRequest>,
) -> ApiResult<Json<ControlResponse>> {
    // Validate command
    if !ALLOWED_COMMANDS.contains(&body.command.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Unknown command '{}'. Allowed: {:?}",
            body.command, ALLOWED_COMMANDS
        )));
    }

    let cmd_topic = std::env::var("MQTT_CMD_TOPIC").unwrap_or_else(|_| "esp/cmd".into());

    // Publish via MQTT if client is available
    match state.mqtt_client().await {
        Some(client) => {
            client
                .publish(&cmd_topic, QoS::AtLeastOnce, false, body.command.as_bytes())
                .await
                .map_err(|e| AppError::Mqtt(e.to_string()))?;

            info!("Published command '{}' to {cmd_topic}", body.command);
            db::log_command(state.pool(), &body.command, "sent").await.ok();

            Ok(Json(ControlResponse { ok: true, command: body.command }))
        }
        None => {
            warn!("MQTT client not available – command '{}' dropped", body.command);
            db::log_command(state.pool(), &body.command, "failed").await.ok();

            Err(AppError::Mqtt("MQTT client not connected".into()))
        }
    }
}
