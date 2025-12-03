use crate::config::Config;
use crate::core::project::Project;
use crate::core::RuntimeContext;
use crate::error::ProjectError;
use crate::languages::{get_language, supported_languages};
use clap::{Arg, Command};
use dialoguer::{Input, Select};
use log::{debug, error, info};
use std::path::PathBuf;

pub async fn run() -> Result<(), ProjectError> {
    debug!("Parsing command line arguments");
    let matches = Command::new("poq")
        .version("0.1.0")
        .author("Your Name")
        .about("Project generator")
        .arg(
            Arg::new("language")
                .help("Programming language")
                .required(false),
        )
        .arg(
            Arg::new("project_type")
                .help("Project type")
                .required(false),
        )
        .arg(Arg::new("name").help("Project name").required(false))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Custom configuration file"),
        )
        .get_matches();

    let config_path = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("config/python/default.toml");
    let config = Config::load(config_path).map_err(|e| {
        error!("Failed to load configuration: {}", e);
        ProjectError::Config(format!("Failed to load configuration: {}", e))
    })?;

    let language_options = supported_languages();
    let language_name = match matches.get_one::<String>("language") {
        Some(lang) => lang.to_string(),
        None => {
            let selection = Select::new()
                .with_prompt("Select programming language")
                .items(&language_options)
                .interact()
                .unwrap();
            language_options[selection].to_string()
        }
    };

    let project_type = match matches.get_one::<String>("project_type") {
        Some(pt) => pt.to_string(),
        None => {
            let project_types = vec!["web", "cli", "data_science", "base"];
            let selection = Select::new()
                .with_prompt("Select project type")
                .items(&project_types)
                .interact()
                .unwrap();
            project_types[selection].to_string()
        }
    };

    let project_name = match matches.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => Input::<String>::new()
            .with_prompt("Enter project name")
            .interact()
            .unwrap(),
    };

    info!(
        "Creating project: {} (Type: {}, Language: {})",
        project_name, project_type, language_name
    );

    // Create runtime context with user selections
    let context = RuntimeContext::new(
        config,
        language_name.clone(),
        project_type.clone(),
        project_name.clone(),
    );

    let project = Project::new(&project_name, PathBuf::from(&project_name));
    debug!("Project object created: {:?}", project.name);

    let language = get_language(&language_name)
        .ok_or_else(|| ProjectError::Config(format!("Unsupported language: {}", language_name)))?;

    let generator = language.get_generator(&context.config, &context.project_type);
    debug!(
        "Selected generator type: {}",
        std::any::type_name_of_val(&generator)
    );

    info!("Setting up environment");
    match language.setup_environment(&project, &context.config) {
        Ok(_) => {
            info!("Generating project");
            match generator.generate(&project, &context.config) {
                Ok(_) => {
                    info!("Project '{}' created successfully!", project_name);
                    println!("To run your project:");
                    println!("1. cd {}", project_name);
                    println!("2. pip install -r requirements.txt");
                    println!("3. python main.py");
                }
                Err(e) => {
                    error!("Failed to generate project: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            error!("Failed to set up environment: {}", e);
            return Err(e);
        }
    }
    Ok(())
}
