use anyhow::Error;
use converter::{Component, Decorator};
#[allow(dead_code)]
use handlebars::Handlebars;
use rand::prelude::*;
use serde_json::{json, Map, Value};
use std::{collections::HashMap, fs::File};
use terminal_spinners::{SpinnerBuilder, DOTS};

mod converter;
mod merger;
mod scanner;

const EMOJI_LIST: [&str; 16] = [
    "🖋️", "📝", "📄", "📚", "📖", "📓", "📒", "📃", "📜", "📰", "📑", "🔖", "🔗", "📎", "📐", "📏",
];

fn random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let random_emoji = *EMOJI_LIST.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

fn learn_about_project() -> Result<(), Error> {
    let converter = converter::Converter::new();
    let merger = merger::Merger::new();

    let techs = scanner::list_project_technologies().unwrap();
    let mut outputs = vec![];

    for tech in techs {
        let output = converter.convert(&tech.config_file);

        // if unable to convert the file skip it
        if output.is_err() {
            println!(
                "Error: Failed to convert {}: {}",
                tech.config_file,
                output.unwrap_err()
            );
            continue;
        }
        outputs.push(output.unwrap());
    }

    let merged = merger.merge(outputs);

    println!("{:?}", merged);

    Ok(())
}

fn writeme(readme_contents: Value) -> Result<(), Error> {
    let mut handlebars = Handlebars::new();
    let mut readme_file = File::create("README.MD")?;
    handlebars
        .register_template_file("template", "README.tpl.md")
        .unwrap();
    handlebars.render_to_write("template", &readme_contents, &mut readme_file)?;
    Ok(())
}

fn main() {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Writing README.md")
        .start();

    learn_about_project();

    handle.done();
}
