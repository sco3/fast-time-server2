use rmcp::{tool, tool_router};

pub struct TimeServer;

#[tool_router(server_handler)]
impl TimeServer {
    #[tool(description = "Get current time")]
    pub async fn get_time(&self) -> String {
        "12:00".into()
    }
}
