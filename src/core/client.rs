use crate::config::settings::OrbitConfig;
use crate::core::graphql::hive_queries;
use crate::core::types::{LatestVersionData, OrganizationData, ProjectData};
use crate::error::OrbitError;
use crate::utils::client_helper::execute_query;
use reqwest::Client;

pub struct HiveClient {
    http: Client,
    endpoint: String,
    token: String,
    org: String,
}

impl HiveClient {
    pub fn new(config: &OrbitConfig, token_override: Option<String>) -> Result<Self, OrbitError> {
        let profile = config
            .active_profile()
            .ok_or(OrbitError::NoActiveProfile("".to_string()))?;
        let token = token_override
            .or_else(|| std::env::var("HIVE_TOKEN").ok())
            .or_else(|| config.active_profile().map(|p| p.token.clone()))
            .ok_or(OrbitError::NoToken)?;

        let http = Client::builder()
            .user_agent(concat!("orbit/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| OrbitError::Other(e.to_string()))?;

        Ok(Self {
            http,
            endpoint: profile.endpoint.clone(),
            token,
            org: profile.org.clone(),
        })
    }

    pub async fn list_projects(&self) -> Result<OrganizationData, OrbitError> {
        let variables = serde_json::json!({"organizationSlug": self.org});
        execute_query(
            &self.http,
            &self.endpoint,
            &self.token,
            hive_queries::PROJECTS,
            variables,
        )
        .await
    }

    pub async fn targets_by_project(&self, project_slug: &str) -> Result<ProjectData, OrbitError> {
        let query = hive_queries::TARGETS;
        let variables =
            serde_json::json!({"organizationSlug": self.org, "projectSlug": project_slug});
        execute_query(&self.http, &self.endpoint, &self.token, query, variables).await
    }

    pub async fn services_by_target(
        &self,
        project_slug: &str,
        target_slug: &str,
    ) -> Result<LatestVersionData, OrbitError> {
        let query = hive_queries::SUBGRAPHS;
        let variables = serde_json::json!({
            "target": {
                "bySelector": {
                    "organizationSlug": self.org,
                    "projectSlug": project_slug,
                    "targetSlug": target_slug
                }
            }
        });
        execute_query(&self.http, &self.endpoint, &self.token, query, variables).await
    }
}
