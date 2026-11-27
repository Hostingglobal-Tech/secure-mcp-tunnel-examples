use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum DemoService {
    DemoApi,
    DemoQueue,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ServiceQuery {
    /// Select a service from the synthetic catalog.
    service: DemoService,
}

struct ServiceCatalog {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ServiceCatalog {
    #[tool(
        description = "Look up a synthetic service status for the webinar demo.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn get_demo_service_status(&self, Parameters(query): Parameters<ServiceQuery>) -> String {
        match query.service {
            DemoService::DemoApi => "demo_api: healthy".into(),
            DemoService::DemoQueue => "demo_queue: degraded (synthetic backlog)".into(),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for ServiceCatalog {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = ServiceCatalog {
        tool_router: ServiceCatalog::tool_router(),
    };
    server
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
