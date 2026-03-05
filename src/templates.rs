use std::collections::HashMap;
use crate::config::Node;

pub struct TemplateManager {
    templates: HashMap<String, Node>,
}

impl TemplateManager {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Макрос для загрузки шаблона из файла (пути относительно src/)
        macro_rules! load_template {
            ($name:expr, $file:expr) => {
                match Node::from_toml(include_str!($file)) {
                    Ok(node) => templates.insert($name.to_string(), node),
                    Err(e) => panic!("Failed to parse template {}: {}", $file, e),
                }
            };
        }

        load_template!("node", "../templates/node.toml");
        load_template!("rust", "../templates/rust.toml");
        load_template!("python", "../templates/python.toml");
        load_template!("go", "../templates/go.toml");
        load_template!("generic", "../templates/generic.toml");

        Self { templates }
    }

    pub fn get(&self, name: &str) -> Option<&Node> {
        self.templates.get(name)
    }

    pub fn list(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}