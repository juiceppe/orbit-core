# 📚 orbit-core

**Shared library for the Orbit ecosystem**

---

## 🎯 Purpose

`orbit-core` provides the foundational components used by both `orbit-cli` and `orbit-tui`:

- 🔌 **HiveClient** — GraphQL Hive API client
- ⚙️ **Configuration** — TOML-based config with profile support
- 📝 **Types** — Shared data structures for API responses
- ❌ **Error Handling** — Unified error types

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
orbit-core = { path = "../orbit-core" }
```

---

## 🔧 Usage

### Loading Configuration

```rust
use orbit_core::{load_config, OrbitConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads from ./orbit.toml or ~/.config/orbit/orbit.toml
    let config = load_config()?;

    // Get current profile
    if let Some(profile) = config.active_profile() {
        println!("Organization: {}", profile.org);
    }

    Ok(())
}
```

### Using the Hive Client

```rust
use orbit_core::{load_config, HiveClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let client = HiveClient::new(&config, None)?;

    // List projects
    let org_data = client.list_projects().await?;
    for edge in org_data.organization.projects.edges {
        if let Some(slug) = edge.node.slug {
            println!("Project: {}", slug);
        }
    }

    // Get targets for a project
    let project_data = client.targets_by_project("my-project").await?;

    // Get services for a target
    let version_data = client.services_by_target("my-project", "production").await?;

    Ok(())
}
```

---

## ⚙️ Configuration

### Config File Location

1. **Local** (project-specific): `./orbit.toml`
2. **Global** (user-wide): `~/.config/orbit/orbit.toml`

Local config takes precedence over global.

### Config Format

```toml
current_profile = "default"

[profiles.default]
token = "your-hive-token"
endpoint = "https://app.graphql-hive.com/graphql"
org = "your-organization"
vendor = "hive"

[profiles.production]
token = "prod-token"
endpoint = "https://app.graphql-hive.com/graphql"
org = "prod-org"
vendor = "hive"
```

### Token Resolution Priority

1. CLI flag `--token` (highest priority)
2. Environment variable `HIVE_TOKEN`
3. Config file token (lowest priority)

---

## 📊 Data Hierarchy

```
Organization
└── Projects
    └── Targets (environments)
        └── Services (subgraphs)
            └── Schema (SDL)
                └── Supergraph (composed)
```

---

## 🏗️ Module Structure

```
orbit-core/src/
├── lib.rs              # Public exports
├── error.rs            # OrbitError enum
├── config/
│   ├── mod.rs          # Config module
│   ├── settings.rs     # OrbitConfig, Profile structs
│   └── loader.rs       # Config file loading
├── core/
│   ├── mod.rs          # Core module
│   ├── client.rs       # HiveClient implementation
│   ├── types.rs        # API response types
│   └── graphql/
│       ├── mod.rs
│       ├── hive_queries.rs   # GraphQL query strings
│       └── hive_mutations.rs # GraphQL mutations
└── utils/
    ├── mod.rs
    └── client_helper.rs # HTTP utilities
```

---

## 🔮 Future Plans

- [ ] Apollo GraphOS provider support
- [ ] Provider trait abstraction
- [ ] Schema publishing mutations
- [ ] Schema validation utilities
