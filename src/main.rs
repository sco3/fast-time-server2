use fast_time_server::main_init::init_main;
use fast_time_server::main_serve::serve;

// const APP_NAME: &str = "fast-time-server";
// const APP_VERSION: &str = "1.5.0";
// const DEFAULT_PORT: u16 = 8080;
// const MCP_VERSION: &str = "2025-03-26";

#[tokio::main]
async fn main() {
    let args = init_main();
    serve(&args);
}
