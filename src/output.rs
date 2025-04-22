use crate::config::OutputConfig;
use crate::protocol::SeabindMessage;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
    response::Response,
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
};
use tokio::sync::broadcast;
use tracing::{info, error};
use std::net::SocketAddr;
use futures::{StreamExt, SinkExt};
use hyper::server::Server;

pub struct OutputManager {
    config: OutputConfig,
    rx: broadcast::Receiver<SeabindMessage>,
}

impl OutputManager {
    pub fn new(config: OutputConfig, tx: broadcast::Sender<SeabindMessage>) -> Self {
        Self {
            config,
            rx: tx.subscribe(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut tasks = vec![];

        // Start WebSocket server if enabled
        if self.config.websocket {
            let rx = self.rx.resubscribe();
            tasks.push(tokio::spawn(async move {
                if let Err(e) = Self::run_websocket_server(rx).await {
                    error!("WebSocket server error: {}", e);
                }
            }));
        }

        // Start REST API if enabled
        if self.config.rest_api {
            let rx = self.rx.resubscribe();
            tasks.push(tokio::spawn(async move {
                if let Err(e) = Self::run_rest_api(rx).await {
                    error!("REST API server error: {}", e);
                }
            }));
        }

        // Start file logger if configured
        if let Some(path) = &self.config.seabind_log {
            let rx = self.rx.resubscribe();
            let path = path.clone();
            tasks.push(tokio::spawn(async move {
                if let Err(e) = Self::run_file_logger(rx, path).await {
                    error!("File logger error: {}", e);
                }
            }));
        }

        // Wait for all tasks
        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    async fn run_websocket_server(mut rx: broadcast::Receiver<SeabindMessage>) -> Result<()> {
        let app = Router::new()
            .route("/ws", get(Self::handle_ws_upgrade));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
        info!("Starting WebSocket server on {}", addr);
        
        let server = Server::bind(&addr).serve(app.into_make_service());
        server.await?;

        Ok(())
    }

    async fn handle_ws_upgrade(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(|socket| async {
            Self::handle_ws_connection(socket).await
        })
    }

    async fn handle_ws_connection(mut socket: WebSocket) {
        while let Some(Ok(msg)) = socket.recv().await {
            match msg {
                Message::Text(text) => {
                    if let Err(e) = socket.send(Message::Text(text)).await {
                        error!("WebSocket send error: {}", e);
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }

    async fn run_rest_api(rx: broadcast::Receiver<SeabindMessage>) -> Result<()> {
        let app = Router::new()
            .route("/api/status", get(|| async { "OK" }))
            .route("/api/data", post(|| async { "Data received" }));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
        info!("Starting REST API server on {}", addr);
        
        let server = Server::bind(&addr).serve(app.into_make_service());
        server.await?;

        Ok(())
    }

    async fn run_file_logger(mut rx: broadcast::Receiver<SeabindMessage>, path: std::path::PathBuf) -> Result<()> {
        use tokio::fs::OpenOptions;
        use tokio::io::AsyncWriteExt;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;

        let mut writer = tokio::io::BufWriter::new(file);

        while let Ok(msg) = rx.recv().await {
            let data = serde_json::to_string(&msg)?;
            writer.write_all(data.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }

        Ok(())
    }
} 