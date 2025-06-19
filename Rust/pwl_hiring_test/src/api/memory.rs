use axum::{Extension, extract::State};
use chrono::{DateTime, Utc};

use crate::{
    infra::{dto::InsertSave, repositories::MemoryRepository},
    models::Difficulty,
};

use super::{ApiResult, AppState, Json, MessageResponse, user::UserClaims};

#[derive(Debug, serde::Deserialize)]
pub struct SaveRequest {
    #[serde(rename = "gameDate")]
    pub game_date: DateTime<Utc>,
    pub failed: i32,
    pub difficulty: Difficulty,
    pub completed: i32,
    #[serde(rename = "timeTaken")]
    pub time_taken: i32,
}

pub async fn save(
    State(state): State<AppState>,
    Extension(claims): Extension<UserClaims>,
    Json(save): Json<SaveRequest>,
) -> ApiResult<MessageResponse> {
    tracing::debug!("Received data to save: {:?}", save);

    let save = InsertSave {
        user_id: claims.id,
        game_date: save.game_date,
        failed: save.failed,
        difficulty: save.difficulty,
        completed: save.completed,
        time_taken: save.time_taken,
    };

    state.db().save(save).await?;

    Ok(MessageResponse::new("Game data saved successfully"))
}
