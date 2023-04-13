use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::BufReader,
};

#[derive(Debug, Serialize, Deserialize)]
struct Shield {
    label: String,
    message: String,
    color: String,
    logo: String,
    label_color: String,
    logo_color: String,
    style: String,
    logo_width: i32,
    alt_text: String,
    target: String,
}

impl Shield {
    fn new(
        label: String,
        message: String,
        color: String,
        logo: String,
        label_color: String,
        logo_color: String,
        style: String,
        logo_width: i32,
        alt_text: String,
        target: String,
    ) -> Self {
        Shield {
            label,
            message,
            color,
            logo,
            label_color,
            logo_color,
            style,
            logo_width,
            alt_text,
            target,
        }
    }
    fn result(&self) -> String {
        let shield_tpl = fs::read_to_string("./assets/tpl/SHIELD.md").unwrap();
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shield_tpl", shield_tpl.clone())
            .unwrap();
        let data = json!({
            "label": self.label,
            "message": self.message,
            "color": self.color,
            "logo": self.logo,
            "label_color": self.label_color,
            "logo_color": self.logo_color,
            "style": self.style,
            "logo_width": self.logo_width,
            "alt_text": self.alt_text,
            "target": self.target,
        });

        let shield = handlebars.render("shield_tpl", &data).unwrap();
        return shield;
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Tech {
    config_files: Vec<String>,
    dependency_names: Vec<String>,
    shield: Shield,
}

impl Tech {
    fn new(config_files: Vec<String>, dependency_names: Vec<String>, shield: Shield) -> Self {
        Tech {
            config_files,
            dependency_names,
            shield,
        }
    }
}

pub fn list_technologies() {
    let file = File::open("./src/tech_conf.json").expect("file not found");
    let reader = BufReader::new(file);
    let objects: Value = serde_json::from_reader(reader).unwrap();

    let mut config_files_present: Vec<String> = vec![];
    for (name, obj) in objects.as_object().unwrap().iter() {
        let tech: Tech = serde_json::from_value(obj.clone()).unwrap();
        scan_project(&tech.config_files, &mut config_files_present);
    }
}

fn scan_project(config_files: &Vec<String>, config_files_present: &mut Vec<String>) {
    let paths = glob::glob("./**/*").unwrap();

    for path in paths {
        let path = path.unwrap();
        let path_str = path.to_str().unwrap();

        for config in config_files {
            // TODO replace with regex
            if path_str.contains(config) {
                config_files_present.push(path_str.to_string());
            }
        }
    }
}
