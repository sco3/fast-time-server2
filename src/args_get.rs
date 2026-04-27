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
                println!("Starting streaming http on {}:{}", args.listen, args.port);
            }
            Transport::Http => {
                let addr = &args
                    .addr
                    .as_ref()
                    .expect("Addr is required for HTTP transport");
                println!("Starting HTTP on {}", addr);
            }
            Transport::Dual => {
                let token = args.auth_token.as_deref().unwrap_or("no-token");
                println!(
                    "Starting Dual mode on port {} with token {}",
                    args.port, token
                );
            }
            Transport::Rest => {
                println!("Starting REST server on port {}", args.port);
            }
        }
        args
    }
}
