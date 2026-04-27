use crate::args::Args;
use crate::serve_http::serve_http;

pub async fn serve(args: &Args) -> anyhow::Result<()> {
    serve_http(args).await?;
    Ok(())
}
