use anyhow::Error;
use converter::{Component, Decorator};
#[allow(dead_code)]
use handlebars::Handlebars;
use rand::prelude::*;
use serde_json::{json, Map, Value};
use std::fs::File;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod converter;
mod merger;
mod scanner;

const EMOJI_LIST: [&str; 16] = [
    "🖋️", "📝", "📄", "📚", "📖", "📓", "📒", "📃", "📜", "📰", "📑", "🔖", "🔗", "📎", "📐", "📏",
];

fn merge_json_objects(obj1: Value, obj2: Value) -> Result<Value, Error> {
    let obj1: Map<String, Value> = obj1.as_object().unwrap().clone();
    let obj2: Map<String, Value> = obj2.as_object().unwrap().clone();

    let mut merged = Map::new();

    for (k, v) in obj1 {
        merged.insert(k, v);
    }

    for (k, v) in obj2 {
        let existing_val = merged.get(&k);

        match existing_val {
            Some(val) => {
                let conflicts = [val.to_string(), v.to_string()];

                dialoguer::console::Term::stdout()
                    .write_line(format!("Found conflicting key: {k}").as_str())
                    .unwrap();

                let selection = dialoguer::Select::new()
                    .with_prompt("Which value would you like to keep?")
                    .items(&conflicts)
                    .default(0)
                    .interact_on_opt(&dialoguer::console::Term::stderr())?;

                match selection {
                    Some(index) => println!("User selected item : {}", conflicts[index]),
                    None => println!("User did not select anything"),
                }
            }
            None => {
                merged.insert(k, v);
            }
        }
    }

    return Ok(json!(merged));
}

fn random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let random_emoji = *EMOJI_LIST.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

fn learn_about_project() -> Result<(), Error> {
    let converter = converter::Converter::new();

    let cargo_result = converter.convert("./assets/Cargo.toml");
    if let Err(e) = cargo_result {
        println!("Error: Failed to convert './assets/Cargo.toml': {}", e);
    }

    let package_result = converter.convert("./assets/package_1.json");
    if let Err(e) = package_result {
        println!("Error: Failed to convert './assets/package_1.json': {}", e);
    }

    let package_1_result = converter.convert("./assets/composer.json");
    if let Err(e) = package_1_result {
        println!("Error: Failed to convert './assets/composer.json': {}", e);
    }

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
    let converter = converter::Converter::new();
    let merger = merger::Merger::new();
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Writing README.md")
        .start();

    let cargo = converter
        .convert("./assets/Cargo.toml")
        .unwrap_or_else(|e| {
            println!("Error: Failed to convert './assets/Cargo.toml': {}", e);
            std::process::exit(1);
        });

    let package = converter
        .convert("./assets/package.json")
        .unwrap_or_else(|e| {
            println!("Error: Failed to convert './assets/package.json': {}", e);
            std::process::exit(1);
        });

    let package_1 = converter
        .convert("./assets/composer.json")
        .unwrap_or_else(|e| {
            println!("Error: Failed to convert './assets/composer.json': {}", e);
            std::process::exit(1);
        });

    let merged = merger.merge(vec![cargo, package, package_1]);

    println!("{:?}", merged);

    handle.done();

    scanner::scanner::list_technologies();
}
