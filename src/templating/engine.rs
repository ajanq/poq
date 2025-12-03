use crate::error::ProjectError;
use handlebars::Handlebars;
use log::{debug, error};
use serde_json::Value;
use std::path::Path;

pub struct TemplatingEngine {
    handlebars: Handlebars<'static>,
}

impl Default for TemplatingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplatingEngine {
    pub fn new() -> Self {
        TemplatingEngine {
            handlebars: Handlebars::new(),
        }
    }

    pub fn render(&self, template_name: &str, data: &Value) -> Result<String, ProjectError> {
        debug!("Rendering template: {}", template_name);
        self.handlebars.render(template_name, data).map_err(|e| {
            error!("Failed to render template '{}': {}", template_name, e);
            ProjectError::Template(e)
        })
    }

    #[allow(dead_code)]
    pub fn render_string(&self, template: &str, data: &Value) -> Result<String, ProjectError> {
        debug!("Rendering string template");
        self.handlebars
            .render_template(template, data)
            .map_err(|e| {
                error!("Failed to render string template: {}", e);
                ProjectError::Template(e)
            })
    }

    pub fn register_template(&mut self, name: &str, template: &str) -> Result<(), ProjectError> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| ProjectError::Template(e.into()))
    }

    pub fn register_template_file<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
    ) -> Result<(), ProjectError> {
        debug!(
            "Registering template '{}' from file: {:?}",
            name,
            path.as_ref()
        );
        let content = std::fs::read_to_string(&path).map_err(|e| {
            error!(
                "Failed to read template file '{}': {}",
                path.as_ref().display(),
                e
            );
            ProjectError::Config(format!(
                "Failed to read template '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;
        self.register_template(name, &content)
    }

    pub fn load_templates_from_dir<P: AsRef<Path>>(
        &mut self,
        dir: P,
        mappings: &[(&str, &str)],
    ) -> Result<(), ProjectError> {
        debug!("Loading templates from directory: {:?}", dir.as_ref());
        for (name, filename) in mappings {
            self.register_template_file(name, dir.as_ref().join(filename))?;
        }
        Ok(())
    }
}
