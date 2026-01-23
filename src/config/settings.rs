use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OrbitConfig {
    #[serde(default)]
    pub current_profile: Option<String>,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub token: String,
    pub endpoint: String,
    pub org: String,
    #[serde(default = "default_vendor")]
    pub vendor: String,
}

pub fn default_vendor() -> String {
    "hive".to_string()
}

impl OrbitConfig {
    pub fn active_profile(&self) -> Option<&Profile> {
        self.current_profile
            .as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    // pub fn get_token(&self) -> Option<&str> {
    //     self.active_profile().map(|p| p.token.as_str())
    // }
}

impl Profile {
    pub fn new_profile(token: String, endpoint: String, org: String, vendor: String) -> Self {
        Self {
            token,
            endpoint,
            org,
            vendor,
        }
    }
}
