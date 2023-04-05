use rand::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use terminal_spinners::{SpinnerBuilder, DOTS};

// static mut PK_MANAGER: &str = "";

fn animate_loading(pk_manager: String) {
    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text(" Writing README.md")
        .start();
    let res = learn_about_project(pk_manager);
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn writeme(keywords: HashMap<&str, &str>, pk_manager: String) -> Result<()> {
    let mut file = File::create("README.MD")?;
    // make a html string
    let mut readme = read_template();

    let license = make_a_shield(
        "license".to_string(),
        "MIT".to_string(),
        "red".to_string(),
        "https://opensource.org/licenses/MIT".to_string(),
    );
    let version = make_a_shield(
        "version".to_string(),
        "0.1.0".to_string(),
        "black".to_string(),
        "".to_string(),
    );

    readme = readme.replace("{shields}", format!("{}{}", license, version).as_str());
    readme = readme.replace("{pk_manager}", pk_manager.as_str());
    readme = readme.replace("{icon}", random_emoji().as_str());

    for (key, value) in keywords.iter() {
        println!("{{{}}}: {}", key, value);
        readme = readme.replace(format!("{{{}}}", key).as_str(), value);
    }

    file.write_all(readme.as_bytes())?;
    Ok(())
}

fn read_template() -> String {
    let contents =
        fs::read_to_string("TEMPLATE.md").expect("Should have been able to read the template file");
    return contents;
}

fn learn_about_project(pk_manager: String) -> Result<()> {
    let contents = fs::read_to_string("./assets/package_1.json")
        .expect("Should have been able to read the template file");

    let list_project_keys = vec![
        "name",
        "version",
        "description",
        "keywords",
        "author",
        "license",
        "dependencies",
        "devDependencies",
        "scripts",
        "contributors",
        "private",
        "repository",
        "bugs",
        "homepage",
    ];

    let _v: Value = serde_json::from_str(contents.as_str())?;

    let mut map_project_keys_present: HashMap<&str, &str> = HashMap::new();
    list_project_keys.iter().for_each(|key| {
        if _v[key].is_string() {
            map_project_keys_present.insert(key, _v[key].as_str().unwrap());
        } else if _v[key].is_object() {
            // let object = _v[key].as_object().unwrap();
            // for (key_name, value) in object.iter() {
            //     map_project_keys_present.insert(
            //         format!("{}.{}", key, key_name).as_str(),
            //         value.as_str().unwrap(),
            //     );
            // }
        }
    });

    println!("\n");

    writeme(map_project_keys_present, pk_manager)?;

    Ok(())
}

fn random_emoji() -> String {
    let list_emoji = vec![
        "🖋️", "📝", "📄", "📚", "📖", "📓", "📒", "📃", "📜", "📰", "📑", "🔖", "🔗", "📎", "📐",
        "📏",
    ];
    let mut rng = rand::thread_rng();
    let random_emoji = list_emoji.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

// make a fn to ask user a string
fn ask_user_pk_manager() -> String {
    let mut input = String::new();
    println!("{}", "What package manager do you use? (npm, yarn, cargo)");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

// make a struct shield with a url and a redirect and an output html
struct Shield {
    label: String,
    message: String,
    color: String,
    redirect: String,
}

// make an impl for the struct shield where the result is a string of html
impl Shield {
    fn result(&self) -> String {
        let shield_url = "https://img.shields.io/static/v1?label={label}&message={message}&color={color}"
            .replace("{label}", self.label.as_str())
            .replace("{message}", self.message.as_str())
            .replace("{color}", self.color.as_str());

        let mut shield = "<a href=\"{shield_redirect}\" target=\"_blank\">
        <img src=\"{shield_url}>\">
    </a>"
            .to_string();
        shield = shield.replace("{shield_redirect}", self.redirect.as_str());
        shield = shield.replace("{shield_url}", shield_url.as_str());
        return shield;
    }
}

fn make_a_shield(label: String, message: String, color: String, redirect: String) -> String {
    let shield = Shield {
        label: label,
        message: message,
        color: color,
        redirect: redirect,
    };

    return shield.result();
}

fn main() {
    let pk_manager = ask_user_pk_manager();
    animate_loading(pk_manager);
}
