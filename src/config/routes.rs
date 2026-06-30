use axum::{routing::get, Router};
use crate::app::controllers::index::index_controller;
use crate::app::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_controller::get_index))
        .with_state(state)
}
