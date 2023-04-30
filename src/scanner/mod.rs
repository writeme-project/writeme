use crate::{
    converter::Dependencies,
    utils::{paths, Tech},
};
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

    // list configs as they are always at the end of the path
    let all_configs: Vec<String> = all_configs
        .values()
        .flatten()
        .map(|c: &String| format!(r"{}$", c))
        .collect();

    let regex_set: regex::RegexSet = regex::RegexSet::new(all_configs).unwrap();

    let mut configs_present: Vec<String> = vec![];

    // for each file in the project check if it matches any of the config files
    // if it does add it to the list of configs present
    for path in paths {
        let path_str = path.as_str();
        let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();
        if !matches.is_empty() {
            configs_present.push(path_str.to_string());
        }
    }
    Ok(configs_present)
}

// Returns the list of techs present in the project found through the config files
pub fn scan_techs(paths: &Vec<String>) -> Result<Vec<String>, Error> {
    let contents: String =
        fs::read_to_string(paths::TECHS).expect("Something went wrong reading the techs file");
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();

    let mut techs_present: Vec<String> = vec![];
    let index = 0;

    // for each tech check if any of the config files match any of the files in the project
    // if it does add it to the list of techs present
    // the index is used to limit the number of techs to 40
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

/// Returns the list of dependencies present in the project found through the dependencies field in the configs files
pub fn scan_dependencies(dependencies: Dependencies) -> Result<Vec<String>, Error> {
    let contents: String =
        fs::read_to_string(paths::TECHS).expect("Something went wrong reading the techs file");
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();
    let mut dependencies_present: Vec<String> = vec![];

    let index = 0;
    for (name, tech) in all_techs {
        if index > 40 {
            break;
        }
        let regex_set = regex::RegexSet::new(tech.dependency_names).unwrap();

        for dependency in dependencies.clone() {
            let matches: Vec<_> = regex_set
                .matches(dependency.name.as_str())
                .into_iter()
                .collect();

            if !matches.is_empty() {
                dependencies_present.push(name);
                break;
            }
        }
    }

    Ok(dependencies_present)
}
