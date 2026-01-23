use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrbitError {
    //No Active profile
    #[error("No Active Profile: {0}")]
    NoActiveProfile(String),
    //Required for AUTH
    #[error("No token provided")]
    NoToken,
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
    #[error("GraphQL Error: {0}")]
    GraphQLError(String),
}
