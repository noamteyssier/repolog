use anyhow::{bail, Result};
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

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

