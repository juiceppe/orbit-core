use crate::core::types::RootResponse;
use crate::error::OrbitError;
use reqwest::Client;

pub async fn execute_query<T>(
    client: &Client,
    endpoint: &str,
    token: &str,
    query: &str,
    variables: serde_json::Value,
) -> Result<T, OrbitError>
where
    T: serde::de::DeserializeOwned,
{
    let response = client
        .post(endpoint)
        .bearer_auth(token)
        .json(&serde_json::json!({ "query": query, "variables": variables }))
        .send()
        .await?;

    let body: RootResponse<T> = response.json().await?;

    if let Some(errors) = body.errors {
        let msg = errors
            .first()
            .map(|e| e.message.clone())
            .unwrap_or_else(|| "Unknown GraphQL Error".to_string());
        return Err(OrbitError::GraphQLError(msg));
    }
    body.data
        .ok_or(OrbitError::Other("No data returned".to_string()))
}
