use crate::args::Args;
use crate::time_server::TimeServer;
use axum::serve::ListenerExt;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::{StreamableHttpServerConfig, StreamableHttpService};
use tokio_util::sync;
use tracing::{error, info};

pub async fn serve_http(args: &Args) -> anyhow::Result<()> {
    let ct = sync::CancellationToken::new();

    let service = StreamableHttpService::new(
        || Ok(TimeServer {}),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
    );

    let router = axum::Router::new().route_service("/", service);
    let bind_address = format!("{}:{}", args.listen.clone(), args.port);
    println!("Log level {}", args.log_level);
    println!("Serve on {bind_address}");
    // Bind and serve
    let tcp_listener = tokio::net::TcpListener::bind(&bind_address)
        .await?
        .tap_io(|tcp_stream| {
            if let Err(err) = tcp_stream.set_nodelay(true) {
                error!("failed to set TCP_NODELAY on incoming connection: {err:#}");
            }
        });
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await;
    info!("ok");
    Ok(())
}
