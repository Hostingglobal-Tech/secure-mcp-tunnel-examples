use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
};

struct WorkstationView {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl WorkstationView {
    #[tool(
        description = "Read a fictional workstation screen from the webinar fixture.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn get_demo_workspace_view(&self) -> String {
        "Synthetic workspace\nWindow: Demo Console\nPanel: Overview\nStatus: Ready".into()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for WorkstationView {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = WorkstationView {
        tool_router: WorkstationView::tool_router(),
    };
    server
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
