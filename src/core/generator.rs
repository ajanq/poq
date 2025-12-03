use crate::config::Config;
use crate::core::project::Project;
use crate::error::ProjectError;

pub trait ProjectGenerator {
    fn generate(&self, project: &Project, config: &Config) -> Result<(), ProjectError>;
}
