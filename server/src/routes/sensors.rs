use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::{
    db,
    error::{ApiResult, AppError},
    models::SensorResponse,
    state::AppState,
};

/// GET /api/sensors  →  latest reading
pub async fn get_latest(State(state): State<AppState>) -> ApiResult<Json<SensorResponse>> {
    let row = db::get_latest_reading(state.pool())
        .await
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;

    Ok(Json(SensorResponse::from(row)))
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 { 30 }

/// GET /api/sensors/history?limit=30
pub async fn get_history(
    State(state): State<AppState>,
    Query(q): Query<HistoryQuery>,
) -> ApiResult<Json<Vec<SensorResponse>>> {
    let limit = q.limit.clamp(1, 1000);
    let rows = db::get_reading_history(state.pool(), limit)
        .await
        .map_err(AppError::Database)?;

    let response: Vec<SensorResponse> = rows.into_iter().map(SensorResponse::from).collect();
    Ok(Json(response))
}
