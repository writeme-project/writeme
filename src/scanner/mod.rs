#[allow(dead_code)]
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self},
};
use utils::{paths, GenMarkdown, Shield};

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tech {
    pub config_files: Vec<String>,
    pub dependency_names: Vec<String>,
    pub shield: Shield,
}

// Returns list of config files present in the project
pub fn scan_configs() -> Result<Vec<String>, Error> {
    let contents =
        fs::read_to_string(paths::CONFIGS).expect("Something went wrong reading the config file");
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

// Returns the list of shield urls for the technologies found through the config files
pub fn scan_techs() -> Result<Vec<String>, Error> {
    let contents: String =
        fs::read_to_string(paths::TECHS).expect("Something went wrong reading the techs file");
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();
    let all_techs: Vec<Tech> = all_techs.values().cloned().collect();

    let mut techs_present: Vec<String> = vec![];
    for tech in all_techs {
        let regex_set = regex::RegexSet::new(tech.config_files).unwrap();

        let paths = glob::glob("./**/*").unwrap();
        for path in paths {
            let path = path.unwrap();
            let path_str = path.to_str().unwrap();
            let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();

            if !matches.is_empty() {
                match tech.shield.gen_md() {
                    Ok(md) => techs_present.push(md),
                    Err(e) => println!("Error: {}", e),
                }

                break;
            }
        }
    }
    Ok(techs_present)
}
