pub const PROJECTS: &str = r#"
    query OrganizationProjects($organizationSlug: String!) {
      organization(reference: { bySelector: { organizationSlug: $organizationSlug } }) {
        id
        projects {
          edges {
            node {
              id
              type
              slug
            }
          }
          pageInfo {
            hasNextPage
            endCursor
          }
        }
      }
    }
"#;

pub const TARGETS: &str = r#"query ProjectTargets($organizationSlug: String!, $projectSlug: String!) {
  project(
    reference: {
      bySelector: {
        organizationSlug: $organizationSlug,
        projectSlug: $projectSlug
      }
    }
  ) {
    id
    targets {
      edges {
        node {
          id
          slug
        }
      }
    }
  }
}
"#;

pub const SUBGRAPHS: &str = r#"
query LatestSchemaVersion(
    $target: TargetReferenceInput) {
  latestValidVersion(target: $target) {
    id
    isValid
      sdl
      supergraph
      schemas  {
      edges {
        node {
          __typename
          ... on CompositeSchema {
            id
            date
            url
            service
            source
          }
        }
      }
    }
  }
}
"#;
