use anyhow::{bail, Result};
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

/// Adds titles to files of a specified language in a given directory.
///
/// This function walks through the directory structure starting from the specified path,
/// identifies files of the given language, and adds a title comment to each file if it
/// doesn't already have one.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language identifier (e.g., "rust", "python")
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
/// * There are issues reading or writing to the files
/// * There are issues walking the directory structure
pub fn add_titles(lang: &str, path: &str) -> Result<()> {
    let extension = match lang {
        "rust" => "rs",
        "python" => "py",
        _ => bail!("Unsupported language: {}", lang),
    };

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |ext| ext == extension)
        {
            add_title_to_file(entry.path(), lang)?;
        }
    }

    Ok(())
}

/// Adds a title to a single file if it doesn't already have one.
///
/// This function reads the content of the file, checks if it already has a title,
/// and if not, generates a title and prepends it to the file content.
///
/// # Arguments
///
/// * `file_path` - A reference to the `Path` of the file to be processed
/// * `lang` - A string slice that holds the language identifier
///
/// # Returns
///
/// Returns `Ok(())` if the operation was successful, or an `Error` if something went wrong.
///
/// # Errors
///
/// This function will return an error if:
/// * There are issues reading from or writing to the file
/// * There are issues generating the title
fn add_title_to_file(file_path: &Path, lang: &str) -> Result<()> {
    let mut content = String::new();
    fs::File::open(file_path)?.read_to_string(&mut content)?;

    if !has_title(&content, lang) {
        let title = generate_title(file_path, lang)?;
        content = format!("{}\n\n{}", title, content);
        fs::File::create(file_path)?.write_all(content.as_bytes())?;
    }

    Ok(())
}

/// Checks if the given content already has a title comment.
///
/// This function uses a regular expression to determine if the content
/// already starts with a title comment in the appropriate format for the
/// specified language.
///
/// # Arguments
///
/// * `content` - A string slice containing the file content to check
/// * `lang` - A string slice that holds the language identifier
///
/// # Returns
///
/// Returns `true` if the content already has a title, `false` otherwise.
fn has_title(content: &str, lang: &str) -> bool {
    let comment_pattern = match lang {
        "rust" => r"^//",
        "python" => r"^#",
        _ => return false,
    };

    let re = Regex::new(&format!(
        r"{}.*[a-zA-Z0-9_]+(\.[a-zA-Z0-9_]+)+",
        comment_pattern
    ))
    .unwrap();
    re.is_match(content)
}

/// Generates a title for a file based on its path and language.
///
/// This function creates a title comment using the appropriate comment syntax
/// for the specified language and the file's path within the project structure.
///
/// # Arguments
///
/// * `file_path` - A reference to the `Path` of the file
/// * `lang` - A string slice that holds the language identifier
///
/// # Returns
///
/// Returns a `Result` containing the generated title `String` if successful,
/// or an `Error` if something went wrong.
///
/// # Errors
///
/// This function will return an error if:
/// * The file path cannot be converted to a valid UTF-8 string
fn generate_title(file_path: &Path, lang: &str) -> Result<String> {
    let comment_symbol = match lang {
        "rust" => "//",
        "python" => "#",
        _ => "#",
    };

    let base_path = Path::new(".");
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
    let path_without_extension = relative_path.with_extension("");

    let re = Regex::new(r"[/\\]").unwrap();
    let path_string = re.replace_all(
        path_without_extension
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
        ".",
    );

    Ok(format!("{} {}", comment_symbol, path_string))
}
