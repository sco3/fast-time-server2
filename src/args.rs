use crate::transport::Transport;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "fast-time-server", about = "A high-performance time server")]
pub struct Args {
    /// The communication transport to use
    #[arg(short, long, value_enum, default_value_t = Transport::Stdio)]
    pub transport: Transport,

    /// Log level for the application
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// IP address to listen on (required for sse)
    #[arg(long, default_value = "0.0.0.0")]
    pub listen: String,

    /// Port to bind to (used by sse, dual, rest)
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Specific address for http transport
    #[arg(long)]
    pub addr: Option<String>,

    /// Secret token for dual transport
    #[arg(long)]
    pub auth_token: Option<String>,
}
