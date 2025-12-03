use crate::config::Config;
use crate::core::generator::ProjectGenerator;
use crate::core::project::Project;
use crate::error::ProjectError;

pub trait Language {
    fn get_generator(&self, config: &Config, project_type: &str) -> Box<dyn ProjectGenerator>;
    fn setup_environment(&self, project: &Project, config: &Config) -> Result<(), ProjectError>;
}
