use super::common::GeneratorBase;
use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::project::Project;
use crate::error::ProjectError;
use log::{error, info};
use std::path::PathBuf;

pub struct WebGenerator {
    base: GeneratorBase,
}

impl Default for WebGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl WebGenerator {
    pub fn new() -> Self {
        let mut base = match GeneratorBase::new("web") {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to create generator base: {}", e);
                GeneratorBase::new("web").unwrap_or_else(|_| panic!("Cannot create generator"))
            }
        };

        let web_template_path = PathBuf::from("templates/python/web/fastapi/main.py.template");
        if let Err(e) = base
            .engine
            .register_template_file("main", web_template_path)
        {
            error!("Failed to load web main template: {}", e);
        }

        WebGenerator { base }
    }
}

impl ProjectGenerator for WebGenerator {
    fn generate(&self, project: &Project, config: &Config) -> Result<(), ProjectError> {
        info!("Generating Web project: {}", project.name);

        self.base.create_project_structure(project)?;
        self.base.generate_base_files(project, config)?;
        self.base.generate_main_file(project, config)?;

        let web_config = config.get_project_type_config("web");
        self.base.generate_requirements(
            project,
            &web_config.dependencies,
            Some(&config.test.framework),
        )?;

        info!("Web project generated successfully");
        Ok(())
    }
}
