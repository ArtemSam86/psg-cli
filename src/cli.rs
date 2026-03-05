use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "psgcli")]
#[command(about = "Generate files/folders from tree description", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate from a structure file
    Generate {
        /// Path to the input file (txt, json, toml, yaml)
        file: PathBuf,

        /// Output directory (defaults to current)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Force overwrite existing files without asking
        #[arg(short, long)]
        force: bool,

        /// Interactive mode – ask before overwriting each existing file
        #[arg(short, long)]
        interactive: bool,
    },

    /// Initialize a project from a built-in template
    Init {
        /// Template name (node, rust, python, go, generic)
        template: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List available templates
    ListTemplates,
}