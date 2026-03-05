use std::io::{self, Write};
use std::path::Path;
use crate::error::PsgcliError;

pub fn ask_overwrite(path: &Path) -> Result<bool, PsgcliError> {
    print!("File {} already exists. Overwrite? [y/N] ", path.display());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}