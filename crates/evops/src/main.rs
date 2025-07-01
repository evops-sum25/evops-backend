use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use axum_tonic::RestGrpcService;
use const_format::formatcp;
use tokio::io;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

mod config;
mod shutdown;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let dotenv_path = dotenvy::dotenv().ok();
    self::init_logging();

    if let Some(path) = dotenv_path {
        debug!("found .env: {}", path.display());
    } else {
        debug!(".env not found");
    }

    let config = self::config::from_env()?;

    let (addr, listener) = self::tcp_listener(config.port).await?;
    let app = self::rest_grpc_app(&config).await?;

    info!("listening on {addr}");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(self::shutdown::signal())
        .await?;

    Ok(())
}

fn init_logging() {
    let env_filter = {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(formatcp!(
                "info,tower_http=trace,{}=trace",
                env!("CARGO_CRATE_NAME"),
            ))
        })
    };

    let fmt_layer = {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .without_time()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

async fn tcp_listener(port: u16) -> io::Result<(SocketAddr, TcpListener)> {
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await?;
    let addr = {
        listener
            .local_addr()
            .expect("address should be received successfully")
    };

    Ok((addr, listener))
}

async fn rest_grpc_app(config: &self::config::Config) -> eyre::Result<RestGrpcService> {
    let state = {
        evops_core::AppState::try_new(
            &config.database_url,
            &config.storage_url,
            &config.storage_username,
            &config.storage_password,
        )
        .await?
    };

    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let grpc_router = evops_grpc::router(&state).layer((TraceLayer::new_for_http(), timeout_layer));
    let rest_router = evops_rest::router(state).layer((TraceLayer::new_for_grpc(), timeout_layer));

    Ok(RestGrpcService::new(rest_router, grpc_router))
}
