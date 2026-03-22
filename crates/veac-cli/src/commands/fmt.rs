use std::path::Path;

/// Format a .veac source file (pretty-print).
/// Currently a stub - not yet implemented.
pub fn cmd_fmt(file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !file.exists() {
        return Err(format!("file not found: {}", file.display()).into());
    }
    println!("fmt not yet implemented ({})", file.display());
    Ok(())
}
