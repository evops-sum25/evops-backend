use tokio::select;
use tokio::signal;
use tracing::warn;

pub async fn signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    select! {
        () = ctrl_c => on_sigint(),
        () = terminate => on_sigterm(),
    }
}

#[allow(clippy::missing_const_for_fn)]
fn on_sigint() {
    warn!("received a SIGINT, gracefully shutting down...");
}

#[allow(clippy::missing_const_for_fn)]
fn on_sigterm() {
    warn!("received a SIGTERM, gracefully shutting down...");
}
