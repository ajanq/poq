use serde_json::{json, Value};

pub fn project_context(project_name: &str) -> Value {
    json!({
        "project_name": project_name,
        "project_name_lowercase": project_name.to_lowercase(),
        "project_name_uppercase": project_name.to_uppercase(),
    })
}

pub fn language_context(language: &str, version: &str) -> Value {
    json!({
        "language": language,
        "language_version": version,
    })
}

pub fn merge_contexts(contexts: &[&Value]) -> Value {
    contexts.iter().fold(json!({}), |acc, &context| {
        acc.as_object()
            .unwrap()
            .into_iter()
            .chain(context.as_object().unwrap())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    })
}
