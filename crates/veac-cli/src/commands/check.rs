use std::path::Path;
use std::process;

use crate::pipeline;

/// Validate syntax and semantics of a .veac file (no rendering).
pub fn cmd_check(file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = read_source(file)?;

    match pipeline::compile(&source, file) {
        Ok(_) => {
            println!("No errors found in {}", file.display());
            Ok(())
        }
        Err(e) => {
            eprint!("{e}");
            process::exit(1);
        }
    }
}

fn read_source(file: &Path) -> Result<String, Box<dyn std::error::Error>> {
    if !file.exists() {
        return Err(format!("file not found: {}", file.display()).into());
    }
    Ok(std::fs::read_to_string(file)?)
}
