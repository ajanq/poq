use super::common::GeneratorBase;
use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::project::Project;
use crate::error::ProjectError;
use log::{error, info};

pub struct DataScienceGenerator {
    base: GeneratorBase,
}

impl Default for DataScienceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DataScienceGenerator {
    pub fn new() -> Self {
        let mut base = match GeneratorBase::new("data_science") {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to create generator base: {}", e);
                GeneratorBase::new("data_science")
                    .unwrap_or_else(|_| panic!("Cannot create generator"))
            }
        };

        if let Err(e) = base.load_main_template("main.py.template") {
            error!("Failed to load data science main template: {}", e);
        }

        DataScienceGenerator { base }
    }
}

impl ProjectGenerator for DataScienceGenerator {
    fn generate(&self, project: &Project, config: &Config) -> Result<(), ProjectError> {
        info!("Generating Python Data Science project: {}", project.name);

        self.base.create_project_structure(project)?;
        self.base.generate_base_files(project, config)?;
        self.base.generate_main_file(project, config)?;

        let ds_config = config.get_project_type_config("data_science");
        self.base.generate_requirements(
            project,
            &ds_config.dependencies,
            Some(&config.test.framework),
        )?;

        info!("Python Data Science project generated successfully");
        Ok(())
    }
}
