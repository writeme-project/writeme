use crate::{
    converter::{Contributor, Contributors, ConverterOutput, Dependencies, SupportedFile},
    utils::{paths, Tech},
};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::{
    collections::HashMap,
    vec,
};

use git2::Repository;

// Returns list of config files present in the project
pub fn scan_configs(paths: &Vec<String>) -> Result<Vec<String>, Error> {
    let contents: String = paths::read_util_file_contents(paths::UtilityPath::Configs);
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
    let contents: String = paths::read_util_file_contents(paths::UtilityPath::Techs);
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
    let contents: String = paths::read_util_file_contents(paths::UtilityPath::Techs);
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

/// Returns a ConverterOutput struct with the data found in the .git folder
pub fn scan_git(project_location: &str) -> Result<ConverterOutput, Error> {
    let mut git_converter = ConverterOutput::empty();

    git_converter.source_config_file = SupportedFile::GitRepository;

    // Open the repository
    let repo = match Repository::open(project_location) {
        Ok(repo) => repo,
        Err(e) => {
            return Err(anyhow!("Failed to open repository: {}", e));
        }
    };

    // Get the head commit
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => {
            return Err(anyhow!("Failed to get head: {}", e));
        }
    };

    let head_commit = match head.peel_to_commit() {
        Ok(commit) => commit,
        Err(e) => {
            return Err(anyhow!("Failed to peel to commit: {}", e));
        }
    };

    // Iterate over the commits in the repository
    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(e) => {
            return Err(anyhow!("Failed to get revwalk: {}", e));
        }
    };

    revwalk.push(head_commit.id()).unwrap();

    let mut contributors: HashMap<Contributor, i32> = std::collections::HashMap::new();

    // fill contributors hashmap counting the number of commits for each contributor
    for oid in revwalk {
        let oid = match oid {
            Ok(oid) => oid,
            Err(e) => {
                return Err(anyhow!("Failed to get oid: {}", e));
            }
        };

        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(e) => {
                return Err(anyhow!("Failed to find commit: {}", e));
            }
        };

        let author = commit.author();
        let name = author.name().unwrap();
        let email = author.email().unwrap();

        let contributor = Contributor {
            name: Some(name.to_string()),
            email: Some(email.to_string()),
            url: None,
        };

        let count = contributors.entry(contributor).or_insert(0);
        *count += 1;
    }

    // sort contributors by number of commits
    let contributors: Contributors = contributors
        .iter()
        .sorted_by(|a, b| b.1.cmp(a.1))
        .map(|(contributor, _)| contributor.clone())
        .collect();

    git_converter.contributors = Option::from(contributors);

    git_converter.repository_url = Option::from(
        repo.find_remote("origin")
            .unwrap()
            .url()
            .unwrap()
            .to_string(),
    );

    Ok(git_converter)
}
