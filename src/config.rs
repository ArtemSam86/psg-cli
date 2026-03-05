use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use crate::error::PsgcliError;

/// Узел файловой структуры
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Node {
    /// Файл с опциональным содержимым (если None – пустой)
    File(Option<String>),
    /// Директория с потомками
    Directory(BTreeMap<String, Node>),
}

impl Node {
    /// Загрузить структуру из JSON
    pub fn from_json(content: &str) -> Result<Self, PsgcliError> {
        serde_json::from_str(content)
            .map_err(|e| PsgcliError::parse("JSON", e.to_string()))
    }

    /// Загрузить структуру из TOML
    pub fn from_toml(content: &str) -> Result<Self, PsgcliError> {
        toml::from_str(content)
            .map_err(|e| PsgcliError::parse("TOML", e.to_string()))
    }

    /// Загрузить структуру из YAML
    pub fn from_yaml(content: &str) -> Result<Self, PsgcliError> {
        serde_yaml::from_str(content)
            .map_err(|e| PsgcliError::parse("YAML", e.to_string()))
    }

    /// Загрузить структуру из текстового дерева (формат `tree`)
    pub fn from_txt(content: &str) -> Result<Self, PsgcliError> {
        let mut entries = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            // Нормализуем: заменяем символы псевдографики на пробелы
            let normalized = line
                .replace("├── ", "    ")
                .replace("└── ", "    ")
                .replace("│   ", "    ");
            let indent = normalized.chars().take_while(|c| *c == ' ').count();
            let name = normalized[indent..].trim_end_matches('/').to_string();
            let is_dir = line.trim_end().ends_with('/'); // проверяем оригинальную строку
            let depth = indent / 4; // считаем 4 пробела за уровень
            entries.push((depth, name, is_dir));
        }

        let mut root = BTreeMap::new();
        let mut path_stack: Vec<Vec<String>> = vec![]; // путь для каждой глубины

        for (depth, name, is_dir) in entries {
            // гарантируем, что стек имеет нужную длину
            while path_stack.len() <= depth {
                path_stack.push(Vec::new());
            }

            // строим путь текущего элемента: путь предыдущего уровня + имя
            let mut current_path = if depth == 0 {
                Vec::new()
            } else {
                path_stack[depth - 1].clone()
            };
            current_path.push(name.clone());

            // вставляем элемент в дерево
            insert_into_tree(&mut root, &current_path, is_dir)?;

            // обновляем стек для текущей глубины
            path_stack[depth] = current_path;
        }

        Ok(Node::Directory(root))
    }

    /// Загрузить структуру из файла (формат определяется по расширению)
    pub fn from_file(path: &Path) -> Result<Self, PsgcliError> {
        let content = fs::read_to_string(path)?;
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => Self::from_json(&content),
            Some("toml") => Self::from_toml(&content),
            Some("yaml") | Some("yml") => Self::from_yaml(&content),
            Some("txt") => Self::from_txt(&content),
            _ => Err(PsgcliError::UnsupportedFormat(
                path.extension().unwrap_or_default().to_string_lossy().to_string(),
            )),
        }
    }
}

/// Вспомогательная функция: вставка узла по пути в дерево
fn insert_into_tree(
    tree: &mut BTreeMap<String, Node>,
    path: &[String],
    is_dir: bool,
) -> Result<(), PsgcliError> {
    if path.is_empty() {
        return Err(PsgcliError::InvalidTree("Empty path".to_string()));
    }
    let (last, rest) = path.split_last().unwrap();

    // Сначала обходим все промежуточные компоненты, создавая недостающие директории
    let mut current = tree;
    for component in rest {
        // Проверяем, есть ли уже такой компонент
        if !current.contains_key(component) {
            // Создаём пустую директорию
            current.insert(component.clone(), Node::Directory(BTreeMap::new()));
        }
        // Теперь получаем изменяемую ссылку на этот компонент (должна быть директория)
        match current.get_mut(component) {
            Some(Node::Directory(map)) => {
                current = map;
            }
            Some(_) => {
                return Err(PsgcliError::InvalidTree(format!(
                    "Path component '{}' is not a directory",
                    component
                )));
            }
            None => unreachable!(), // мы только что вставили, если не было
        }
    }

    // Теперь current указывает на родительскую директорию последнего компонента
    if is_dir {
        // Если последний компонент - директория, проверяем/создаём
        if !current.contains_key(last) {
            current.insert(last.clone(), Node::Directory(BTreeMap::new()));
        } else {
            // Проверяем, что существующий узел - директория
            match current.get(last) {
                Some(Node::Directory(_)) => { /* уже директория - ок */ }
                _ => {
                    return Err(PsgcliError::InvalidTree(format!(
                        "Cannot create directory '{}' because a file with the same name exists",
                        last
                    )));
                }
            }
        }
    } else {
        // Файл - не должен существовать
        if current.contains_key(last) {
            return Err(PsgcliError::InvalidTree(format!(
                "File '{}' already exists",
                last
            )));
        }
        current.insert(last.clone(), Node::File(None));
    }
    Ok(())
}