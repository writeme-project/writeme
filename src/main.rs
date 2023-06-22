mod assembler;
mod converter;
mod dialoguer;
mod elements;
mod merger;
mod scanner;
mod utils;

use clap::Parser;
use elements::{license::License, repository::Repository};
use std::path::Path;
use utils::{outputs, Project};

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
    let project: Project = match Project::load(project_location) {
        Ok(project) => project,
        Err(e) => {
            dialoguer::error("Error: Failed to load project: {}", &e);
            return;
        }
    };

    let converter = converter::Converter::new();
    let merger = merger::Merger::new();

    let configs = match scanner::scan_configs(&project.paths) {
        Ok(configs) => configs,
        Err(e) => {
            dialoguer::error("Error: Failed to scan configs: {}", &e);
            return;
        }
    };

    dialoguer::processed_files(configs.clone());

    let mut outputs = vec![];

    for config in configs {
        let output = converter.convert(&config);

        // if unable to convert the file skip it
        if output.is_err() {
            continue;
        }

        outputs.push(output.unwrap());
    }

    match Repository::scan(project_location) {
        Ok(scan_git) => outputs.push(scan_git),
        Err(_) => {} // if unable to scan git do nothing
    };

    match License::scan(&project.paths) {
        Ok(licenses) => {
            licenses
                .iter()
                .for_each(|license| outputs.push(license.clone()));
        }
        Err(_) => {} // if unable to scan license file do nothing
    };

    let mut merged = match merger.merge(outputs) {
        Ok(merged) => merged,
        Err(e) => {
            dialoguer::error("Error: Failed to merge: {}", &e);
            return;
        }
    };

    match License::create(
        project_location,
        merged.license.as_ref().unwrap(),
        merged.name.clone(),
    ) {
        Ok(output_path) => {
            if output_path.is_some() {
                merged.license.as_mut().unwrap().path = output_path
            }
        }
        Err(e) => {
            dialoguer::error("Error: Failed to create license file: {}", &e);
        }
    }

    match assembler::Assembler::new(merged).assemble(
        &format!("{}/{}", project_location, outputs::README),
        &project.paths,
    ) {
        Ok(_) => {}
        Err(e) => {
            dialoguer::error("Error: Failed to assemble: {}", &e);
        }
    }
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);
    // check if path is valid
    if !path.exists() || !path.is_dir() {
        dialoguer::error("Error: Invalid path: {}", &args.path);
        return;
    }
    dialoguer::hello();
    writeme(path.to_str().unwrap());
    dialoguer::bye();
}
