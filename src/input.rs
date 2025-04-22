use crate::config::InputConfig;
use crate::protocol::{SeabindMessage, ToSeabind};
use anyhow::Result;
use tokio::sync::broadcast;
use tokio_serial::SerialStream;
use tracing::{info, error};
use std::io::{BufRead, BufReader};

pub struct InputManager {
    config: InputConfig,
    tx: broadcast::Sender<SeabindMessage>,
}

impl InputManager {
    pub fn new(config: InputConfig, tx: broadcast::Sender<SeabindMessage>) -> Self {
        Self { config, tx }
    }

    pub async fn run(&self) -> Result<()> {
        match self.config.format.as_str() {
            "nmea0183" => self.run_nmea_input().await,
            "nmea2000" => todo!("NMEA 2000 support coming soon"),
            _ => Err(anyhow::anyhow!("Unsupported input format")),
        }
    }

    async fn run_nmea_input(&self) -> Result<()> {
        info!("Starting NMEA 0183 input from {}", self.config.source);

        let port = SerialStream::open(&tokio_serial::new(&self.config.source, 4800))?;
        let reader = BufReader::new(port);

        for line in reader.lines() {
            let line = line?;
            
            // Parse NMEA sentence using the parser
            let parser = nmea::Parser::new();
            if let Ok(sentence) = parser.parse_sentence(&line) {
                // Convert to Seabind messages
                let messages = sentence.to_seabind();
                
                // Broadcast each message
                for msg in messages {
                    if let Err(e) = self.tx.send(msg) {
                        error!("Failed to broadcast message: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
} 