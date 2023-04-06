use terminal_spinners::{ SpinnerBuilder, DOTS };
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use serde_json::Value;
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

fn writeme(keywords: HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("README.MD")?;
    // make a html string
    let mut readme = read_template();

    let shield_license = (Shield {
        label: "license".to_string(),
        message: "MIT".to_string(),
        color: "black".to_string(),
        redirect: "https://opensource.org/licenses/MIT".to_string(),
    }).result();

    let shield_version = (Shield {
        label: "license".to_string(),
        message: "MIT".to_string(),
        color: "red".to_string(),
        redirect: "".to_string(),
    }).result();

    readme = readme.replace("{shields}", format!("{}{}", shield_license, shield_version).as_str());
    readme = readme.replace("{icon}", random_emoji().as_str());

    for (key, value) in keywords.iter() {
        println!("{{{}}}: {}", key, value);
        readme = readme.replace(format!("{{{}}}", key).as_str(), value);
    }

    file.write_all(readme.as_bytes())?;
    Ok(())
}

fn read_template() -> String {
    let contents = fs
        ::read_to_string("TEMPLATE.md")
        .expect("Should have been able to read the template file");
    return contents;
}

fn learn_about_project() -> Result<(), Box<dyn Error>> {
    let contents = fs
        ::read_to_string("./assets/package_1.json")
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
        "homepage"
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

    writeme(map_project_keys_present)?;

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