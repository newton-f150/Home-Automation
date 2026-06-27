use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    db,
    error::{ApiResult, AppError},
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub esp32_online:   bool,
    pub mqtt_connected: bool,
    pub last_seen:      Option<String>,
}

/// GET /api/status
pub async fn get_status(State(state): State<AppState>) -> ApiResult<Json<StatusResponse>> {
    let s = db::get_status(state.pool())
        .await
        .map_err(AppError::Database)?;

    Ok(Json(StatusResponse {
        esp32_online:   s.esp32_online,
        mqtt_connected: s.mqtt_connected,
        last_seen:      s.last_seen.map(|t| t.to_rfc3339()),
    }))
}
