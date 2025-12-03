use poq::config::Config;
use poq::core::project::Project;
use poq::templating::TemplatingEngine;
use std::path::PathBuf;

#[test]
fn test_config_loading() {
    let config = Config::load("config/python/default.toml");
    assert!(config.is_ok(), "Config should load successfully");

    let config = config.unwrap();
    assert_eq!(config.general.language, "python");
    assert_eq!(config.general.version, "3.9");
}

#[test]
fn test_config_new() {
    let config = Config::new();
    assert!(config.is_ok(), "Config::new() should succeed");

    let config = config.unwrap();
    assert_eq!(config.general.language, "python");
}

#[test]
fn test_config_get_project_type() {
    let config = Config::load("config/python/default.toml").unwrap();

    let web_config = config.get_project_type_config("web");
    assert!(web_config.dependencies.contains(&"fastapi".to_string()));

    let cli_config = config.get_project_type_config("cli");
    assert!(cli_config.dependencies.contains(&"argparse".to_string()));

    let ds_config = config.get_project_type_config("data_science");
    assert!(ds_config.dependencies.contains(&"pandas".to_string()));
}

#[test]
fn test_template_rendering() {
    let mut engine = TemplatingEngine::new();
    engine
        .register_template("test", "Hello, {{name}}!")
        .unwrap();

    let result = engine.render("test", &serde_json::json!({"name": "World"}));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, World!");
}

#[test]
fn test_template_rendering_with_multiple_vars() {
    let mut engine = TemplatingEngine::new();
    engine
        .register_template("test", "Project: {{project_name}}, Language: {{language}}")
        .unwrap();

    let result = engine.render(
        "test",
        &serde_json::json!({
            "project_name": "my-app",
            "language": "python"
        }),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Project: my-app, Language: python");
}

#[test]
fn test_language_factory() {
    use poq::languages::{get_language, supported_languages};

    // Test supported languages list
    let languages = supported_languages();
    assert!(languages.contains(&"python"));

    // Test getting Python language
    let python = get_language("python");
    assert!(python.is_some());

    // Test case insensitivity
    let python_upper = get_language("Python");
    assert!(python_upper.is_some());

    // Test alias
    let py = get_language("py");
    assert!(py.is_some());

    // Test unsupported language
    let unknown = get_language("unknown");
    assert!(unknown.is_none());
}

#[test]
fn test_project_creation() {
    let project = Project::new("test-project", PathBuf::from("/tmp/test-project"));
    assert_eq!(project.name, "test-project");
    assert_eq!(project.path, PathBuf::from("/tmp/test-project"));
}

#[test]
fn test_runtime_context() {
    use poq::core::RuntimeContext;

    let config = Config::new().unwrap();
    let context = RuntimeContext::new(
        config,
        "python".to_string(),
        "web".to_string(),
        "my-project".to_string(),
    );

    assert_eq!(context.language, "python");
    assert_eq!(context.project_type, "web");
    assert_eq!(context.project_name, "my-project");
    assert_eq!(context.project_path, PathBuf::from("my-project"));
}

#[test]
fn test_config_validation() {
    let config = Config::new().unwrap();
    let result = config.validate();
    assert!(result.is_ok());
}
