/// BitVMX-Z Node Binary
use anyhow::Result;
use bitvmx_z_backend::{RollupNode, api};
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("bitvmx_z_backend=debug,tower_http=debug")
        .init();

    println!("🚀 Starting BitVMX-Z Node...");

    // Create rollup node
    let node = Arc::new(RollupNode::new());

    // Create API router
    let app = api::create_router(node);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("✅ BitVMX-Z Node listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
