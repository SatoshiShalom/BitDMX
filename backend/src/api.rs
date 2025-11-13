/// REST API for rollup node
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::rollup::{RollupNode, Transaction};

#[derive(Clone)]
pub struct ApiState {
    pub node: Arc<RollupNode>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitTxRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

pub fn create_router(node: Arc<RollupNode>) -> Router {
    let state = ApiState { node };

    Router::new()
        .route("/health", get(health))
        .route("/state", get(get_state))
        .route("/submit", post(submit_transaction))
        .route("/batches", get(get_batches))
        .with_state(state)
}

async fn health() -> &'static str {
    "OK"
}

async fn get_state(State(state): State<ApiState>) -> Json<crate::rollup::RollupState> {
    let rollup_state = state.node.get_state().await;
    Json(rollup_state)
}

async fn submit_transaction(
    State(state): State<ApiState>,
    Json(req): Json<SubmitTxRequest>,
) -> Result<StatusCode, StatusCode> {
    let tx = Transaction {
        id: uuid::Uuid::new_v4().to_string(),
        from: req.from,
        to: req.to,
        amount: req.amount,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    state.node.submit_transaction(tx).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

async fn get_batches() -> Json<Vec<u64>> {
    // TODO: Retrieve batch history from storage
    Json(vec![])
}
