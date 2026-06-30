use axum::{Json, extract::State};
use crate::app::errors::AppError;
use crate::app::services::index::index_service::{hello_handler, MessageResponse};
use crate::app::state::AppState;

pub async fn get_index(State(state): State<AppState>) -> Result<Json<MessageResponse>, AppError> {
    hello_handler(state).await
}
