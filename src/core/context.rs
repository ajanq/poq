use crate::config::Config;
use std::path::PathBuf;

pub struct RuntimeContext {
    pub config: Config,
    #[allow(dead_code)]
    pub language: String,
    pub project_type: String,
    #[allow(dead_code)]
    pub project_name: String,
    #[allow(dead_code)]
    pub project_path: PathBuf,
}

impl RuntimeContext {
    pub fn new(
        config: Config,
        language: String,
        project_type: String,
        project_name: String,
    ) -> Self {
        Self {
            config,
            language,
            project_type,
            project_path: PathBuf::from(&project_name),
            project_name,
        }
    }

    #[allow(dead_code)]
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.project_path = path;
        self
    }
}
