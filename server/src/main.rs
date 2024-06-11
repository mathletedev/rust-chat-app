use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
}
