use crate::error::ProjectError;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTypeConfig {
    pub dependencies: Vec<String>,
    pub main_file_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub web: ProjectTypeConfig,
    pub cli: ProjectTypeConfig,
    pub data_science: ProjectTypeConfig,
    pub base: ProjectTypeConfig,
    pub test: TestConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub language: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConfig {
    pub framework: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, ProjectError> {
        toml::from_str(include_str!("../../config/python/default.toml"))
            .map_err(|e| ProjectError::Config(format!("Failed to parse default config: {}", e)))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ProjectError> {
        debug!("Loading configuration from: {:?}", path.as_ref());
        let config_str = fs::read_to_string(&path).map_err(|e| {
            error!(
                "Failed to read config file '{}': {}",
                path.as_ref().display(),
                e
            );
            ProjectError::Config(format!(
                "Failed to read config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;
        let config: Config = toml::from_str(&config_str).map_err(|e| {
            error!(
                "Failed to parse config file '{}': {}",
                path.as_ref().display(),
                e
            );
            ProjectError::Config(format!(
                "Failed to parse config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;
        debug!("Configuration loaded successfully");
        Ok(config)
    }

    pub fn get_project_type_config(&self, project_type: &str) -> &ProjectTypeConfig {
        match project_type {
            "web" => &self.web,
            "cli" => &self.cli,
            "data_science" => &self.data_science,
            _ => &self.base,
        }
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), ProjectError> {
        if self.general.language.is_empty() {
            return Err(ProjectError::Config("Language not specified".to_string()));
        }
        Ok(())
    }
}
