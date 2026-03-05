use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use crate::config::Node;
use crate::error::PsgcliError;
use crate::interactive::ask_overwrite;

pub struct Generator {
    root: PathBuf,
    force: bool,
    interactive: bool,
}

impl Generator {
    pub fn new(root: PathBuf, force: bool, interactive: bool) -> Self {
        Self {
            root,
            force,
            interactive,
        }
    }

    // Публичный метод для доступа к корневой папке
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn generate(&self, node: &Node, current_path: &Path) -> Result<(), PsgcliError> {
        match node {
            Node::File(content) => {
                let file_path = current_path;
                if file_path.exists() {
                    if self.force {
                        fs::remove_file(file_path)?;
                        self.write_file(file_path, content)?;
                        println!("{} {}", "FORCE".yellow(), file_path.display());
                    } else if self.interactive && ask_overwrite(file_path)? {
                        fs::remove_file(file_path)?;
                        self.write_file(file_path, content)?;
                        println!("{} {}", "OVERWRITTEN".yellow(), file_path.display());
                    } else {
                        println!("{} {}", "SKIP".blue(), file_path.display());
                    }
                } else {
                    if let Some(parent) = file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    self.write_file(file_path, content)?;
                    println!("{} {}", "CREATE".green(), file_path.display());
                }
            }
            Node::Directory(children) => {
                let dir_path = current_path;
                if !dir_path.exists() {
                    fs::create_dir_all(dir_path)?;
                    println!("{} {}", "CREATE".green(), dir_path.display());
                }
                for (name, child) in children {
                    self.generate(child, &dir_path.join(name))?;
                }
            }
        }
        Ok(())
    }

    fn write_file(&self, path: &Path, content: &Option<String>) -> Result<(), PsgcliError> {
        if let Some(text) = content {
            fs::write(path, text)?;
        } else {
            fs::File::create(path)?;
        }
        Ok(())
    }
}