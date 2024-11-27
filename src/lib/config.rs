// src/lib/config.rs

// configuration settings that aren't related to the database or OpenDAL storage

// dependencies
use anyhow::Result;
use shuttle_runtime::SecretStore;

// struct type to represent the app configuration
#[derive(Clone)]
pub struct ServiceConfig {}

// implement the TryFrom trait for the AppConfig type
impl TryFrom<SecretStore> for ServiceConfig {
    type Error = anyhow::Error;
    fn try_from(_value: SecretStore) -> Result<Self> {
        Ok(Self {})
    }
}
