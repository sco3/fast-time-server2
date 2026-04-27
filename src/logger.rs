use crate::args::Args;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger(args: &Args) {
    // if args.transport == Transport::Stdio {
    //     return;
    // }

    println!("init log level: {}", args.log_level);

    let filter = EnvFilter::try_from_default_env() // info
        .unwrap_or_else(|_| EnvFilter::new(&args.log_level));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
