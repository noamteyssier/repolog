use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub fn export_files(lang: &str, output: &str, path: &str) -> Result<()> {
    let extension = match lang {
        "rust" => "rs",
        "python" => "py",
        _ => bail!("Unsupported language: {}", lang),
    };

    let mut output_file = File::create(output)
        .with_context(|| format!("Failed to create output file: {}", output))?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |ext| ext == extension)
        {
            append_file_to_output(&mut output_file, entry.path(), lang)?;
        }
    }

    Ok(())
}

fn append_file_to_output(output_file: &mut File, file_path: &Path, lang: &str) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let file_header = generate_file_header(file_path, lang);
    writeln!(output_file, "{}", file_header)?;
    writeln!(output_file, "{}", file_content)?;
    writeln!(output_file)?; // Add an empty line between files

    Ok(())
}

fn generate_file_header(file_path: &Path, lang: &str) -> String {
    let comment_symbol = match lang {
        "rust" => "//",
        "python" => "#",
        _ => "#",
    };

    let separator = format!("{} {}", comment_symbol, "=".repeat(60));
    let file_name = file_path.display();

    format!(
        "{}\n{} File: {}\n{}",
        separator, comment_symbol, file_name, separator
    )
}
