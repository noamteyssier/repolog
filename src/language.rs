use clap::ValueEnum;

/// Represents the supported programming languages in the repolog tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Language {
    Rust,
    Python,
    // Add more languages as needed
}

impl Language {
    /// Returns the file extension associated with the language.
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Python => "py",
            // Add more languages as needed
        }
    }

    /// Returns the comment symbol used for the language.
    pub fn comment_symbol(&self) -> &'static str {
        match self {
            Language::Rust => "//",
            Language::Python => "#",
            // Add more languages as needed
        }
    }
}
