use crate::config::Config;
use crate::input::InputManager;
use crate::output::OutputManager;
use anyhow::Result;
use tokio::sync::broadcast;
use tracing::{info, error};

pub struct Server {
    input_manager: InputManager,
    output_manager: OutputManager,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let (tx, _) = broadcast::channel(100);
        let input_manager = InputManager::new(config.input, tx.clone());
        let output_manager = OutputManager::new(config.output, tx);

        Self {
            input_manager,
            output_manager,
        }
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting server components...");

        // Start input processing
        let input_handle = {
            let input_manager = self.input_manager;
            tokio::spawn(async move {
                if let Err(e) = input_manager.run().await {
                    error!("Input manager error: {}", e);
                }
            })
        };

        // Start output processing
        let output_handle = {
            let output_manager = self.output_manager;
            tokio::spawn(async move {
                if let Err(e) = output_manager.run().await {
                    error!("Output manager error: {}", e);
                }
            })
        };

        // Wait for both components
        tokio::try_join!(input_handle, output_handle)?;

        Ok(())
    }
} 