mod cli;
mod config;
mod engine;
mod templates;
mod interactive;
mod error;

use clap::Parser;
use std::path::PathBuf;
use crate::cli::{Cli, Commands};
use crate::engine::Generator;
use crate::error::PsgcliError;
use crate::templates::TemplateManager;

fn main() -> Result<(), PsgcliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            file,
            output,
            force,
            interactive,
        } => {
            let root = output.unwrap_or_else(|| PathBuf::from("."));
            let node = config::Node::from_file(&file)?;
            let generator = Generator::new(root, force, interactive);
            generator.generate(&node, generator.root())?; // используем метод root()
        }

        Commands::Init { template, output } => {
            let tm = TemplateManager::new();
            let node = tm
                .get(&template)
                .ok_or_else(|| PsgcliError::TemplateNotFound(template))?;
            let root = output.unwrap_or_else(|| PathBuf::from("."));
            let generator = Generator::new(root, false, false);
            generator.generate(node, generator.root())?; // используем метод root()
        }

        Commands::ListTemplates => {
            let tm = TemplateManager::new();
            println!("Available templates:");
            for name in tm.list() {
                println!("  - {}", name);
            }
        }
    }

    Ok(())
}