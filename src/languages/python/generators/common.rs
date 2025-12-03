use crate::config::Config;
use crate::core::project::Project;
use crate::error::ProjectError;
use crate::templating::TemplatingEngine;
use crate::utils::{
    create_directory, language_context, merge_contexts, project_context, write_file,
};
use log::{debug, error};
use serde_json::Value;
use std::path::PathBuf;

const GITIGNORE_TEMPLATE: &str = "gitignore";
const README_TEMPLATE: &str = "readme";
const MAIN_TEMPLATE: &str = "main";
const GITIGNORE_FILE: &str = ".gitignore";
const README_FILE: &str = "README.md";
const MAIN_FILE: &str = "main.py";
const REQUIREMENTS_FILE: &str = "requirements.txt";

pub struct GeneratorBase {
    pub engine: TemplatingEngine,
    template_dir: PathBuf,
}

impl GeneratorBase {
    pub fn new(project_type: &str) -> Result<Self, ProjectError> {
        let template_dir = PathBuf::from("templates/python").join(project_type);
        let mut engine = TemplatingEngine::new();

        let base_dir = PathBuf::from("templates/python/base");
        engine.load_templates_from_dir(
            &base_dir,
            &[
                (GITIGNORE_TEMPLATE, "gitignore.template"),
                (README_TEMPLATE, "readme.template"),
            ],
        )?;

        Ok(Self {
            engine,
            template_dir,
        })
    }

    #[allow(dead_code)]
    pub fn with_main_template(mut self, template_subpath: &str) -> Result<Self, ProjectError> {
        let main_template_path = self.template_dir.join(template_subpath);
        self.engine
            .register_template_file(MAIN_TEMPLATE, main_template_path)?;
        Ok(self)
    }

    pub fn load_main_template(&mut self, filename: &str) -> Result<(), ProjectError> {
        self.engine
            .register_template_file(MAIN_TEMPLATE, self.template_dir.join(filename))
    }

    pub fn create_project_structure(&self, project: &Project) -> Result<(), ProjectError> {
        debug!("Creating project directory: {:?}", project.path);
        create_directory(&project.path)
    }

    pub fn create_context(&self, project: &Project, config: &Config) -> Value {
        let project_ctx = project_context(&project.name);
        let language_ctx = language_context(&config.general.language, &config.general.version);
        merge_contexts(&[&project_ctx, &language_ctx])
    }

    pub fn render_and_write(
        &self,
        template: &str,
        filename: &str,
        context: &Value,
        project: &Project,
    ) -> Result<(), ProjectError> {
        let content = self.engine.render(template, context)?;
        write_file(project.path.join(filename), content).map_err(|e| {
            error!("Failed to write file '{}': {}", filename, e);
            e
        })
    }

    pub fn generate_base_files(
        &self,
        project: &Project,
        config: &Config,
    ) -> Result<(), ProjectError> {
        debug!("Generating base files");
        let context = self.create_context(project, config);
        self.render_and_write(GITIGNORE_TEMPLATE, GITIGNORE_FILE, &context, project)?;
        self.render_and_write(README_TEMPLATE, README_FILE, &context, project)?;
        Ok(())
    }

    pub fn generate_main_file(
        &self,
        project: &Project,
        config: &Config,
    ) -> Result<(), ProjectError> {
        debug!("Generating main.py");
        let context = self.create_context(project, config);
        self.render_and_write(MAIN_TEMPLATE, MAIN_FILE, &context, project)
    }

    pub fn generate_requirements(
        &self,
        project: &Project,
        dependencies: &[String],
        test_framework: Option<&str>,
    ) -> Result<(), ProjectError> {
        debug!("Generating requirements.txt");
        let mut content = dependencies.join("\n");
        if let Some(framework) = test_framework {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(framework);
        }
        content.push('\n');
        write_file(project.path.join(REQUIREMENTS_FILE), content).map_err(|e| {
            error!("Failed to write requirements file: {}", e);
            e
        })
    }
}
