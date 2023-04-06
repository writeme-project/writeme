use terminal_spinners::{ SpinnerBuilder, DOTS };
use std::collections::HashMap;
use handlebars::Handlebars;
use std::io::prelude::*;
use std::error::Error;
use serde_json::{ Value, json };
use rand::prelude::*;
use std::fs::File;
use std::fs;

fn animate_loading() {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(" Writing README.md").start();
    let res = learn_about_project();
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn writeme(readme_contents: Value) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    let mut readme_file = File::create("README.MD")?;
    handlebars.register_template_file("template", "README.tpl.md").unwrap();
    handlebars.render_to_write("template", &readme_contents, &mut readme_file)?;
    Ok(())
}

fn learn_about_project() -> Result<(), Box<dyn Error>> {
    // read the package.json file and create a json object
    let contents = fs
        ::read_to_string("./assets/package_1.json")
        .expect("Should have been able to read the template file");
    let mut package_json: Value = serde_json::from_str(contents.as_str()).unwrap();

    // add a property to the json object
    package_json["icon"] = json!(random_emoji());

    println!("package_json: {}", package_json);

    writeme(package_json)?;

    Ok(())
}

fn random_emoji() -> String {
    let list_emoji = vec![
        "🖋️",
        "📝",
        "📄",
        "📚",
        "📖",
        "📓",
        "📒",
        "📃",
        "📜",
        "📰",
        "📑",
        "🔖",
        "🔗",
        "📎",
        "📐",
        "📏"
    ];
    let mut rng = rand::thread_rng();
    let random_emoji = list_emoji.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

struct Shield {
    label: String,
    message: String,
    color: String,
    redirect: String,
}

impl Shield {
    fn result(&self) -> String {
        let shield_url =
            "https://img.shields.io/static/v1?label={label}&message={message}&color={color}"
                .replace("{label}", self.label.as_str())
                .replace("{message}", self.message.as_str())
                .replace("{color}", self.color.as_str());

        let mut shield =
            "<a href=\"{shield_redirect}\" target=\"_blank\">
        <img src=\"{shield_url}>\">
    </a>".to_string();
        shield = shield.replace("{shield_redirect}", self.redirect.as_str());
        shield = shield.replace("{shield_url}", shield_url.as_str());
        return shield;
    }
}

fn main() {
    animate_loading();
}