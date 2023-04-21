use anyhow::Error;
#[allow(dead_code)]
use handlebars::Handlebars;
use serde_json::Value;
use std::fs::File;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod assembler;
mod converter;
mod merger;
mod scanner;
mod utils;

fn learn_about_project() -> Result<(), Error> {
    let converter = converter::Converter::new();
    let merger = merger::Merger::new();

    let configs = scanner::scan_configs().unwrap();
    let mut outputs = vec![];

    for config in configs {
        let output = converter.convert(&config);

        // if unable to convert the file skip it
        if output.is_err() {
            println!(
                "Error: Failed to convert {}: {}",
                config,
                output.unwrap_err()
            );
            continue;
        }
        outputs.push(output.unwrap());
    }

    let merged = match merger.merge(outputs) {
        Ok(merged) => merged,
        Err(e) => {
            println!("Error: Failed to merge: {}", e);
            return Err(e);
        }
    };

    match assembler::Assembler::new(merged).assemble() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: Failed to assemble: {}", e);
            return Err(e);
        }
    };

    Ok(())
}

fn writeme(readme_contents: Value) -> Result<(), Error> {
    let mut handlebars = Handlebars::new();
    let mut readme_file = File::create(utils::paths::README)?;
    handlebars
        .register_template_file("template", utils::paths::TEMPLATE)
        .unwrap();
    handlebars.render_to_write("template", &readme_contents, &mut readme_file)?;
    Ok(())
}

fn main() {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Writing README.md")
        .start();

    let ok = learn_about_project();

    match ok {
        Ok(_) => {
            handle.done();
        }
        Err(e) => {
            handle.error();
        }
    }
}
