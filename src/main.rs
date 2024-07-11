use anyhow::Result;
use clap::{Parser, Subcommand};
use export::export_files;
use title::add_titles;

mod export;
mod title;

/// Represents the command-line interface for the repolog tool.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Represents the available subcommands for the repolog tool.
#[derive(Subcommand)]
enum Commands {
    /// Adds titles to files of a specified language in a given directory.
    Title {
        /// The programming language to process (e.g., "rust", "python")
        #[clap(short, long)]
        lang: String,

        /// The path to the directory to process (default: current directory)
        #[clap(default_value = ".")]
        path: String,
    },
    /// Exports files of a specified language from a given directory.
    Export {
        /// The programming language to process (e.g., "rust", "python")
        #[clap(short, long)]
        lang: String,

        /// The path to the output file (optional, defaults to stdout if not provided)
        #[clap(short, long)]
        output: Option<String>,

        /// The path to the directory to process (default: current directory)
        #[clap(default_value = ".")]
        path: String,
    },
}

/// The main entry point for the repolog tool.
///
/// This function parses the command-line arguments and executes the appropriate
/// subcommand based on the user's input.
///
/// # Errors
///
/// This function will return an error if:
/// * There are issues parsing the command-line arguments
/// * The executed subcommand encounters an error
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Title { lang, path } => add_titles(&lang, &path),
        Commands::Export { lang, output, path } => export_files(&lang, output.as_deref(), &path),
    }
}
