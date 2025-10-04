// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rmcp::{
    handler::server::{
        router::tool::ToolRouter,
        wrapper::{Json, Parameters},
    },
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServerHandler,
    transport::streamable_http_server::{session::local::LocalSessionManager, StreamableHttpService},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, schemars::JsonSchema)]
struct GetAnimalDetailsRequest {
    animal_name: String,
}

#[derive(Serialize, schemars::JsonSchema)]
struct AnimalDetails {
    habitat: String,
    diet: String,
}

#[derive(Serialize, schemars::JsonSchema)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Clone)]
pub struct ZooServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ZooServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get details about an animal")]
    fn get_animal_details(
        &self,
        Parameters(GetAnimalDetailsRequest { animal_name }): Parameters<GetAnimalDetailsRequest>,
    ) -> Json<serde_json::Value> {
        let mut zoo_data = HashMap::new();
        zoo_data.insert(
            "lion",
            AnimalDetails {
                habitat: "savanna".to_string(),
                diet: "carnivore".to_string(),
            },
        );
        zoo_data.insert(
            "penguin",
            AnimalDetails {
                habitat: "antarctica".to_string(),
                diet: "piscivore".to_string(),
            },
        );
        zoo_data.insert(
            "giraffe",
            AnimalDetails {
                habitat: "savanna".to_string(),
                diet: "herbivore".to_string(),
            },
        );

        let animal_name_lower = animal_name.to_lowercase();

        let result = match zoo_data.get(animal_name_lower.as_str()) {
            Some(details) => serde_json::to_value(details).unwrap(),
            None => serde_json::to_value(&ErrorResponse {
                error: format!("Animal '{}' not found.", animal_name),
            })
            .unwrap(),
        };
        Json(result)
    }
}

#[tool_handler]
impl ServerHandler for ZooServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A server for getting animal details.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let service = StreamableHttpService::new(
        || Ok(ZooServer::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await?;

    Ok(())
}