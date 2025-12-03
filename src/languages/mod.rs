pub mod python;
// pub mod go;  // Uncomment when Go is implemented

use crate::core::language::Language;

pub use python::Python;

pub fn get_language(name: &str) -> Option<Box<dyn Language>> {
    match name.to_lowercase().as_str() {
        "python" | "py" => Some(Box::new(Python)),
        // "go" | "golang" => Some(Box::new(go::Go)),
        _ => None,
    }
}

pub fn supported_languages() -> Vec<&'static str> {
    vec!["python"] // Add "go" when implemented
}
