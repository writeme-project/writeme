#[allow(dead_code)]
use anyhow::Error;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shield {
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
    pub fn gen_md(&self) -> String {
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tech {
    pub config_files: Vec<String>,
    pub dependency_names: Vec<String>,
    pub shield: Shield,
}

pub struct Project_Tech {
    pub name: String,
    pub config_file: String,
    pub tech: Tech,
}

pub fn list_project_technologies() -> Result<Vec<Project_Tech>, Error> {
    let techs: HashMap<String, Tech> = list_technologies().unwrap();
    let mut project_techs: Vec<Project_Tech> = vec![];

    for (name, tech) in techs {
        let config_files = tech.config_files.clone();
        let reg_set = regex::RegexSet::new(config_files).unwrap();
        let paths = glob::glob("./**/*").unwrap();

        for path in paths {
            let path = path.unwrap();
            let path_str = path.to_str().unwrap();
            let matches: Vec<_> = reg_set.matches(path_str).into_iter().collect();
            if !matches.is_empty() {
                let project_tech = Project_Tech {
                    name: name.clone(),
                    config_file: path_str.to_string(),
                    tech: tech.clone(),
                };
                project_techs.push(project_tech);
            }
        }
    }

    Ok(project_techs)
}

pub fn list_technologies() -> Result<HashMap<String, Tech>, Error> {
    let file = File::open("./src/tech_conf.json").expect("file not found");
    let reader = BufReader::new(file);
    let objects: Value = serde_json::from_reader(reader).unwrap();

    let mut techs: HashMap<String, Tech> = HashMap::new();
    for (name, obj) in objects.as_object().unwrap().iter() {
        let tech: Tech = serde_json::from_value(obj.clone()).unwrap();
        techs.insert(name.to_string(), tech);
    }
    Ok(techs)
}
