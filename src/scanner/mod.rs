use crate::utils::{paths, Project, Tech};
use anyhow::Error;
use std::{
    collections::HashMap,
    fs::{self},
    vec,
};

// Returns list of config files present in the project
pub fn scan_configs(paths: &Vec<String>) -> Result<Vec<String>, Error> {
    let contents: String =
        fs::read_to_string(paths::CONFIGS).expect("Something went wrong reading the config file");
    let all_configs: HashMap<String, Vec<String>> = serde_yaml::from_str(&contents).unwrap();
    let all_configs: Vec<String> = all_configs
        .values()
        .flatten()
        .map(|c: &String| format!(r"{}$", c))
        .collect();

    let regex_set: regex::RegexSet = regex::RegexSet::new(all_configs).unwrap();

    let mut configs_present: Vec<String> = vec![];

    for path in paths {
        let path_str = path.as_str();
        let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();
        if !matches.is_empty() {
            configs_present.push(path_str.to_string());
        }
    }
    Ok(configs_present)
}

// Returns the list of shield urls for the technologies found through the config files
pub fn scan_techs(paths: &Vec<String>) -> Result<Vec<String>, Error> {
    let contents: String =
        fs::read_to_string(paths::TECHS).expect("Something went wrong reading the techs file");
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();

    let mut techs_present: Vec<String> = vec![];
    let index = 0;
    for (name, tech) in all_techs {
        if index > 40 {
            break;
        }
        let regex_set = regex::RegexSet::new(tech.config_files).unwrap();

        for path in paths {
            let path_str = path.as_str();
            let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();

            if !matches.is_empty() {
                techs_present.push(name);
                break;
            }
        }
    }
    Ok(techs_present)
}
