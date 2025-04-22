//! # Nexmarine
//! 
//! Nexmarine is a blazing-fast, cross-platform marine data server written in Rust.
//! It provides a modern solution for handling marine data using the Seabind protocol.
//! 
//! ## Features
//! 
//! - Real-time data processing from NMEA 0183/2000 sources
//! - WebSocket and REST API endpoints
//! - Modern web dashboard
//! - Extensible plugin architecture
//! 
//! ## Example
//! 
//! ```rust
//! use nexmarine::config::Config;
//! use nexmarine::server::Server;
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Load configuration
//!     let config = Config::load("nexmarine.toml")?;
//!     
//!     // Create and run server
//!     let server = Server::new(config);
//!     server.run().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod protocol;
pub mod server;
pub mod input;
pub mod output;
pub mod dashboard;

#[cfg(test)]
mod tests; 