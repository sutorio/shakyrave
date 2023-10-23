use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Location of config file.
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// What do you want to do?
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the HTML site.
    Build,
    /// Serve the site for local development.
    Serve,
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.config.as_deref() {
        let conf = shakyrave::config::Config::acquire(path.to_path_buf()).unwrap();

        println!("Value for config: {conf:?}");
    }

    match &cli.command {
        Some(Commands::Build) => {
            println!("This should build the website!");
        }
        Some(Commands::Serve) => {
            println!("You should serve the website for development!");
        },
        None => {}
    }
}
