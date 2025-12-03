pub mod generators;

use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::language::Language;
use crate::core::project::Project;
use crate::error::ProjectError;
use log::{debug, error, info};

pub struct Python;

impl Language for Python {
    fn get_generator(&self, _config: &Config, project_type: &str) -> Box<dyn ProjectGenerator> {
        let generator = match project_type {
            "web" => Box::new(generators::web::WebGenerator::new()) as Box<dyn ProjectGenerator>,
            "cli" => Box::new(generators::cli::CliGenerator::new()) as Box<dyn ProjectGenerator>,
            "data_science" => Box::new(generators::data_science::DataScienceGenerator::new())
                as Box<dyn ProjectGenerator>,
            _ => Box::new(generators::base::BaseGenerator::new()) as Box<dyn ProjectGenerator>,
        };
        debug!(
            "Selected generator type: {}",
            std::any::type_name_of_val(&generator)
        );
        generator
    }

    fn setup_environment(&self, project: &Project, _config: &Config) -> Result<(), ProjectError> {
        info!(
            "Setting up Python environment for project: {}",
            project.name
        );

        let venv_path = project.path.join("venv");
        debug!("Creating virtual environment at: {:?}", venv_path);
        std::process::Command::new("python")
            .args(["-m", "venv", venv_path.to_str().unwrap()])
            .output()
            .map_err(|e| {
                error!("Failed to create virtual environment: {}", e);
                ProjectError::Generator(format!("Failed to create virtual environment: {}", e))
            })?;

        let requirements_path = project.path.join("requirements.txt");
        if requirements_path.exists() {
            std::process::Command::new(venv_path.join("bin").join("pip"))
                .args(["install", "-r", requirements_path.to_str().unwrap()])
                .output()
                .map_err(|e| {
                    error!("Failed to install dependencies: {}", e);
                    ProjectError::Generator(format!("Failed to install dependencies: {}", e))
                })?;
        }

        info!("Python environment set up successfully");
        Ok(())
    }
}
