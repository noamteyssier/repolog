use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

/// Exports files of a specified language from a given path.
///
/// This function walks through the directory structure starting from the specified path,
/// identifies files of the given language, and either writes their contents to stdout
/// or to a specified output file.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language identifier (e.g., "rust", "python")
/// * `output` - An optional string slice that holds the path to the output file
/// * `path` - A string slice that holds the path to the directory to search for files
///
/// # Returns
///
/// Returns `Ok(())` if the operation was successful, or an `Error` if something went wrong.
///
/// # Errors
///
/// This function will return an error if:
/// * The specified language is not supported
/// * There are issues reading the input files
/// * There are issues writing to the output (either stdout or file)
pub fn export_files(lang: &str, output: Option<&str>, path: &str) -> Result<()> {
    let extension = match lang {
        "rust" => "rs",
        "python" => "py",
        _ => bail!("Unsupported language: {}", lang),
    };

    let mut writer: Box<dyn Write> = match output {
        Some(file_path) => Box::new(
            File::create(file_path)
                .with_context(|| format!("Failed to create output file: {}", file_path))?,
        ),
        None => Box::new(io::stdout()),
    };

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |ext| ext == extension)
        {
            append_file_to_output(&mut writer, entry.path(), lang)?;
        }
    }

    Ok(())
}

/// Appends the contents of a file to the output, with a header.
///
/// This function reads the contents of the specified file, generates a header,
/// and writes both the header and file contents to the output.
///
/// # Arguments
///
/// * `writer` - A mutable reference to something that implements `Write`
/// * `file_path` - A reference to the `Path` of the file to be appended
/// * `lang` - A string slice that holds the language identifier
///
/// # Returns
///
/// Returns `Ok(())` if the operation was successful, or an `Error` if something went wrong.
///
/// # Errors
///
/// This function will return an error if:
/// * There are issues reading the input file
/// * There are issues writing to the output
fn append_file_to_output(writer: &mut dyn Write, file_path: &Path, lang: &str) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let file_header = generate_file_header(file_path, lang);
    writeln!(writer, "{}", file_header)?;
    writeln!(writer, "{}", file_content)?;
    writeln!(writer)?; // Add an empty line between files

    Ok(())
}

/// Generates a header for a file in the output.
///
/// This function creates a header that includes the file path and separators
/// using the appropriate comment syntax for the specified language.
///
/// # Arguments
///
/// * `file_path` - A reference to the `Path` of the file
/// * `lang` - A string slice that holds the language identifier
///
/// # Returns
///
/// Returns a `String` containing the generated header.
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

