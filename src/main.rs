use handlebars::Handlebars;
use rand::prelude::*;
use serde_json::{ Map, Value, json };
use std::error::Error;
#[allow(dead_code)]
use std::fs;
use std::fs::File;
use terminal_spinners::{ SpinnerBuilder, DOTS };

const EMOJI_LIST: [&str; 16] = [
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
    "📏",
];

fn config_to_json(path: &str) -> Result<Value, Box<dyn Error>> {
    let contents = fs
        ::read_to_string(path)
        .expect("Should have been able to read the template file");

    let file_type = path.split(".").last().unwrap();
    match file_type {
        "json" => {
            let json: Value = serde_json::from_str(contents.as_str()).unwrap();
            return Ok(json);
        }
        "yml" => {
            let json: Value = serde_yaml::from_str(contents.as_str()).unwrap();
            return Ok(json);
        }
        "toml" => {
            let json: Value = toml::from_str(contents.as_str()).unwrap();
            // print!("{}", serde_json::to_string_pretty(&json).unwrap());
            return Ok(json);
        }
        _ => {
            return Err("File type not supported".into());
        }
    }
}

fn merge_json_objects(obj1: Value, obj2: Value) -> Value {
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
                if v.as_str().map_or(false, |s| s.len() > val.as_str().map_or(0, |v| v.len())) {
                    merged.insert(k, v);
                }
            }
            None => {
                merged.insert(k, v);
            }
        }
    }
    return json!(merged);
}

fn random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let random_emoji = *EMOJI_LIST.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

// struct Shield {
//     label: String,
//     message: String,
//     color: String,
//     redirect: String,
// }

// impl Shield {
//     fn result(&self) -> String {
//         let shield_url =
//             "https://img.shields.io/static/v1?label={label}&message={message}&color={color}"
//                 .replace("{label}", self.label.as_str())
//                 .replace("{message}", self.message.as_str())
//                 .replace("{color}", self.color.as_str());

//         let mut shield =
//             "<a href=\"{shield_redirect}\" target=\"_blank\">
//         <img src=\"{shield_url}>\">
//     </a>".to_string();
//         shield = shield.replace("{shield_redirect}", self.redirect.as_str());
//         shield = shield.replace("{shield_url}", shield_url.as_str());
//         return shield;
//     }
// }

fn animate_loading() {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(" Writing README.md").start();
    let res = learn_about_project();
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn learn_about_project() -> Result<(), Box<dyn Error>> {
    let cargo: Value = config_to_json("./assets/Cargo.tpl.toml")?;
    let package: Value = config_to_json("./assets/package_1.json")?;

    let mut config = merge_json_objects(cargo, package);
    config["icon"] = json!(random_emoji());

    // write config to file
    let fs = File::create("config.json")?;
    serde_json::to_writer_pretty(fs, &config)?;

    writeme(config)?;

    Ok(())
}

fn writeme(readme_contents: Value) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    let mut readme_file = File::create("README.MD")?;
    handlebars.register_template_file("template", "README.tpl.md").unwrap();
    handlebars.render_to_write("template", &readme_contents, &mut readme_file)?;
    Ok(())
}

fn main() {
    animate_loading();
}