mod config;
mod protocol;
mod server;
mod input;
mod output;
mod dashboard;
#[cfg(test)]
mod tests;

use anyhow::Result;
use tracing::info;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Nexmarine server...");

    // Load configuration
    let config = config::load_config()?;
    
    // Create broadcast channel for data
    let (tx, _) = broadcast::channel(100);
    
    // Initialize server
    let server = server::Server::new(config.clone());
    
    // Initialize dashboard
    let dashboard = dashboard::Dashboard::new(tx.clone());

    // Start both components
    tokio::try_join!(
        server.run(),
        dashboard.run(),
    )?;

    Ok(())
}
