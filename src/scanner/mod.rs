#[allow(dead_code)]
use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};
use utils::{paths, Shield};

#[path = "../utils/mod.rs"]
mod utils;

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

pub fn scan_configs() -> Result<Vec<String>, Error> {
    let contents =
        fs::read_to_string(paths::CONFIGS).expect("Something went wrong reading the file");
    let all_configs: HashMap<String, Vec<String>> = serde_yaml::from_str(&contents).unwrap();
    let all_configs: Vec<String> = all_configs
        .values()
        .flatten()
        .map(|c| format!(r"{}$", c))
        .collect();

    let regex_set = regex::RegexSet::new(all_configs).unwrap();

    let paths = glob::glob("./**/*").unwrap();

    let mut configs_present: Vec<String> = vec![];
    for path in paths {
        let path = path.unwrap();
        let path_str = path.to_str().unwrap();
        let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();
        if !matches.is_empty() {
            configs_present.push(path_str.to_string());
        }
    }
    Ok(configs_present)
}

pub fn scan_techs() {}

pub fn list_project_technologies() -> Result<Vec<Project_Tech>, Error> {
    let techs: HashMap<String, Tech> = list_technologies().unwrap();
    let mut project_techs: Vec<Project_Tech> = vec![];
    let paths = glob::glob("./**/*").unwrap();
    let paths: Vec<std::path::PathBuf> = paths.into_iter().map(|x| x.unwrap()).collect();

    for (name, tech) in techs {
        let config_files = tech.config_files.clone();
        let reg_set = regex::RegexSet::new(config_files).unwrap();

        for path in &paths {
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
    let file = File::open(paths::TECHS).expect("file not found");
    let reader = BufReader::new(file);
    let objects: Value = serde_json::from_reader(reader).unwrap();

    let mut techs: HashMap<String, Tech> = HashMap::new();
    for (name, obj) in objects.as_object().unwrap().iter() {
        let tech: Tech = serde_json::from_value(obj.clone()).unwrap();
        techs.insert(name.to_string(), tech);
    }
    Ok(techs)
}
