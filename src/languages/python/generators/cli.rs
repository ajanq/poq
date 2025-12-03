use super::common::GeneratorBase;
use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::project::Project;
use crate::error::ProjectError;
use log::{error, info};

pub struct CliGenerator {
    base: GeneratorBase,
}

impl Default for CliGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CliGenerator {
    pub fn new() -> Self {
        let mut base = match GeneratorBase::new("cli") {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to create generator base: {}", e);
                GeneratorBase::new("cli").unwrap_or_else(|_| panic!("Cannot create generator"))
            }
        };

        if let Err(e) = base.load_main_template("main.py.template") {
            error!("Failed to load CLI main template: {}", e);
        }

        CliGenerator { base }
    }
}

impl ProjectGenerator for CliGenerator {
    fn generate(&self, project: &Project, config: &Config) -> Result<(), ProjectError> {
        info!("Generating Python CLI project: {}", project.name);

        self.base.create_project_structure(project)?;
        self.base.generate_base_files(project, config)?;
        self.base.generate_main_file(project, config)?;

        let cli_config = config.get_project_type_config("cli");
        self.base.generate_requirements(
            project,
            &cli_config.dependencies,
            Some(&config.test.framework),
        )?;

        info!("Python CLI project generated successfully");
        Ok(())
    }
}
