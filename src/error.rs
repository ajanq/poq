use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Generator error: {0}")]
    Generator(String),
}
