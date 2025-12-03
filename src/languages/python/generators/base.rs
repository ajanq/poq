use super::common::GeneratorBase;
use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::project::Project;
use crate::error::ProjectError;
use log::{error, info};

pub struct BaseGenerator {
    base: GeneratorBase,
}

impl Default for BaseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseGenerator {
    pub fn new() -> Self {
        let mut base = match GeneratorBase::new("base") {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to create generator base: {}", e);
                GeneratorBase::new("base").unwrap_or_else(|_| panic!("Cannot create generator"))
            }
        };

        if let Err(e) = base.load_main_template("main.py.template") {
            error!("Failed to load main template: {}", e);
        }

        BaseGenerator { base }
    }
}

impl ProjectGenerator for BaseGenerator {
    fn generate(&self, project: &Project, config: &Config) -> Result<(), ProjectError> {
        info!("Generating base Python project: {}", project.name);

        self.base.create_project_structure(project)?;
        self.base.generate_base_files(project, config)?;
        self.base.generate_main_file(project, config)?;

        info!("Base Python project generated successfully");
        Ok(())
    }
}
