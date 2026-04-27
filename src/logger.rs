use crate::args::Args;
use crate::transport::Transport;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger(args: &Args) {
    if args.transport == Transport::Stdio {
        return;
    }

    let filter = EnvFilter::try_from_default_env() // info
        .unwrap_or_else(|_| EnvFilter::new(&args.log_level));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
