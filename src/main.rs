use anyhow::Error;
use converter::{Component, Decorator};
#[allow(dead_code)]
use handlebars::Handlebars;
use rand::prelude::*;
use serde_json::{json, Map, Value};
use std::fs::File;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod converter;
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

    let cargo = converter.convert("./assets/Cargo.toml");
    let package = converter.convert("./assets/package_1.json");
    let package = converter.convert("./assets/composer.json");

    // let mut config = match merge_json_objects(cargo, package) {
    //     Ok(merged) => merged,
    //     Err(_) => {
    //         return Err("Failed to merge JSON objects".into());
    //     }
    // };

    // config["icon"] = json!(random_emoji());

    // // write config to file
    // let fs = File::create("config.json")?;
    // serde_json::to_writer_pretty(fs, &config)?;

    // writeme(config)?;

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

    let res = learn_about_project();

    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }

    scanner::scanner::list_technologies();
}
