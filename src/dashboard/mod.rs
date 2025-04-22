use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use crate::protocol::SeabindMessage;
use tracing::info;

pub struct Dashboard {
    rx: broadcast::Receiver<SeabindMessage>,
}

impl Dashboard {
    pub fn new(tx: broadcast::Sender<SeabindMessage>) -> Self {
        Self {
            rx: tx.subscribe(),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/", get(Self::index_handler))
            .route("/dashboard", get(Self::dashboard_handler))
            .route("/static/*path", get(Self::static_handler));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        info!("Starting dashboard on http://{}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    async fn index_handler() -> Html<&'static str> {
        Html(include_str!("templates/index.html"))
    }

    async fn dashboard_handler() -> Html<&'static str> {
        Html(include_str!("templates/dashboard.html"))
    }

    async fn static_handler(path: axum::extract::Path<String>) -> impl axum::response::IntoResponse {
        let path = path.0;
        let content = match path.as_str() {
            "style.css" => include_str!("static/style.css"),
            "app.js" => include_str!("static/app.js"),
            _ => return axum::http::StatusCode::NOT_FOUND.into_response(),
        };

        let mime = if path.ends_with(".css") {
            "text/css"
        } else if path.ends_with(".js") {
            "application/javascript"
        } else {
            "text/plain"
        };

        ([(axum::http::header::CONTENT_TYPE, mime)], content).into_response()
    }
} 