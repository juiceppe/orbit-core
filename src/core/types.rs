use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RootResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct OrganizationData {
    pub organization: Organization,
}

#[derive(Debug, Deserialize)]
pub struct ProjectData {
    pub project: Project,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub projects: ProjectConnections,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConnections {
    pub edges: Vec<ProjectEdge>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectEdge {
    pub node: Project,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    //pub id: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "type")]
    pub project_type: Option<String>,
    #[serde(rename = "targets")]
    pub project_targets: Option<TargetConnections>,
}

#[derive(Debug, Deserialize)]
pub struct TargetConnections {
    pub edges: Vec<TargetEdge>,
}
#[derive(Debug, Deserialize)]
pub struct TargetEdge {
    pub node: Target,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    //pub id: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct LatestVersionData {
    #[serde(rename = "latestValidVersion")]
    pub latest_version: Option<LatestValidVersion>,
}

#[derive(Debug, Deserialize)]
pub struct LatestValidVersion {
    //pub id: String,
    // #[serde(rename = "isValid")]
    // is_valid: Option<bool>,
    // pub sdl: Option<String>,
    pub supergraph: Option<String>,
    pub schemas: SchemaConnection,
}

#[derive(Debug, Deserialize)]
pub struct SchemaConnection {
    pub edges: Vec<SchemaEdge>,
}

#[derive(Debug, Deserialize)]
pub struct SchemaEdge {
    pub node: Schema,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    //pub id: String,
    pub url: String,
    pub service: String,
    pub source: Option<String>,
    //pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}
