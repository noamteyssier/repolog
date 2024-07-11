use anyhow::Result;
use clap::{Parser, Subcommand};
use export::export_files;
use title::add_titles;

mod export;
mod title;
mod utils;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Title {
        #[clap(short, long)]
        lang: String,

        #[clap(default_value = ".")]
        path: String,
    },
    Export {
        #[clap(short, long)]
        lang: String,

        #[clap(short, long)]
        output: String,

        #[clap(default_value = ".")]
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Title { lang, path } => add_titles(&lang, &path),
        Commands::Export { lang, output, path } => export_files(&lang, &output, &path),
    }
}
