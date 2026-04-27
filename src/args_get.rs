use crate::args::Args;
use crate::transport::Transport;
use clap::Parser;

impl Args {
    pub fn get() -> Args {
        let args = Args::parse();
        // Logic to handle different transport modes
        match args.transport {
            Transport::Stdio => {
                // keep silent
            }
            Transport::Sse => {
                let listen = args.listen.as_deref().unwrap_or("0.0.0.0");
                let port = args.port.expect("Port is required for SSE transport");
                println!("Starting SSE on {}:{}", listen, port);
            }
            Transport::Http => {
                let addr = &args
                    .addr
                    .as_ref()
                    .expect("Addr is required for HTTP transport");
                println!("Starting HTTP on {}", addr);
            }
            Transport::Dual => {
                let port = args.port.expect("Port is required for Dual transport");
                let token = args.auth_token.as_deref().unwrap_or("no-token");
                println!("Starting Dual mode on port {} with token {}", port, token);
            }
            Transport::Rest => {
                let port = args.port.expect("Port is required for REST transport");
                println!("Starting REST server on port {}", port);
            }
        }
        args
    }
}
