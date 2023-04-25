mod assembler;
mod converter;
mod dialoguer;
mod merger;
mod scanner;
mod utils;

use clap::Parser;
use std::path::Path;
use utils::outputs;

/// Writeme helps you generate a fully fledged markdown files (README, CONTRIBUTING, etc.) for your project in a matter
/// of seconds.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the project to scan
    #[arg(short, long, default_value = "./")]
    path: String,
}

/// Method used to Scan the project merges the data found and assembles it to create a README file
fn writeme(project_location: &str) {
    let converter = converter::Converter::new();
    let merger = merger::Merger::new();

    let configs = scanner::scan_configs(project_location).unwrap();
    let mut outputs = vec![];

    for config in configs {
        let output = converter.convert(&config);

        // if unable to convert the file skip it
        if output.is_err() {
            continue;
        }
        outputs.push(output.unwrap());
    }

    let merged = match merger.merge(outputs) {
        Ok(merged) => merged,
        Err(e) => {
            eprintln!("Error: Failed to merge: {}", e);
            return;
        }
    };

    match assembler::Assembler::new(merged).assemble(&format!(
        "{}/{}",
        project_location,
        outputs::README
    )) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Failed to assemble: {}", e);
            return;
        }
    };
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    // check if path is valid
    if !path.exists() || !path.is_dir() {
        eprintln!("Error: Invalid path: {}", args.path);
        return;
    }

    dialoguer::hello();
    writeme(path.to_str().unwrap());
    dialoguer::bye();
}
