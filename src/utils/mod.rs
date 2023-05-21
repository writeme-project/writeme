use anyhow::Error;
use handlebars::Handlebars;
use rust_search::{FilterExt, SearchBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Paths to output files saved to disk produced by the application
pub mod outputs {
    pub const README: &str = "README.md";
}

/// Paths to significant files
pub mod paths {
    pub enum UtilityPath {
        Configs,
        Techs,

        // small pieces of markdown which require some data to be filled in
        Shield,
        Author,
        Support,

        // large macro templates of the README file
        Header,
        Toc,
        Body,
        Footer,
    }

    /// Returns the path of the given file for the given utility type
    pub fn read_util_file_contents(path: UtilityPath) -> String {
        let target = match path {
            UtilityPath::Configs => include_str!("../../conf/configs.yml"),
            UtilityPath::Techs => include_str!("../../conf/techs.yml"),
            UtilityPath::Shield => include_str!("../../conf/tpl/SHIELD.md"),
            UtilityPath::Author => include_str!("../../conf/tpl/AUTHOR.md"),
            UtilityPath::Support => include_str!("../../conf/tpl/SUPPORT.md"),
            UtilityPath::Header => include_str!("../../conf/tpl/HEADER.md"),
            UtilityPath::Toc => include_str!("../../conf/tpl/TABLE_OF_CONTENT.md"),
            UtilityPath::Body => include_str!("../../conf/tpl/BODY.md"),
            UtilityPath::Footer => include_str!("../../conf/tpl/FOOTER.md"),
        };

        target.to_string()
    }
}

/// Used from entities that can be displayed as markdown
pub trait GenMarkdown {
    /// Generates the markdown code for the given object
    fn gen_md(&self) -> Result<String, Error>;
}

/// Structure used to represent shields.io badges
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

/// Structure used to represent a technology possibly used in the project
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tech {
    pub config_files: Vec<String>,
    pub dependency_names: Vec<String>,
    pub shield: Shield,
}

impl GenMarkdown for Shield {
    fn gen_md(&self) -> Result<String, Error> {
        let shield_tpl = paths::read_util_file_contents(paths::UtilityPath::Shield);
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shield_tpl", shield_tpl)
            .unwrap();

        let data: Value = json!(self);

        Ok(handlebars.render("shield_tpl", &data).unwrap())
    }
}

/// Used to trim string removing quotes and spaces from the extremities
/// This is used only in cargo_toml.rs for now
pub fn trim(s: String) -> Result<String, Error> {
    Ok(s.trim().trim_matches('"').to_string())
}

pub enum Aligment {
    Row,
    Column,
}

/// Returns the markdown of shields related with the technologies in the project
pub fn shields(techs: Vec<String>, aligment: Aligment) -> Result<String, Error> {
    let contents: String = paths::read_util_file_contents(paths::UtilityPath::Techs);
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();
    let mut shields: String = String::new();
    for (name, tech) in all_techs {
        if techs.contains(&name) {
            match tech.shield.gen_md() {
                Ok(md) => {
                    shields.push_str(&md);
                    match aligment {
                        Aligment::Row => shields.push(' '),
                        Aligment::Column => shields.push_str("</br>"),
                    }
                }
                // if there is an error to generate markdown, just skip this shield
                Err(_) => continue,
            }
        }
    }

    Ok(shields)
}

#[derive(Clone)]
/// Structure used to represent the project aka global stuff useful when scanning a project
pub struct Project {
    /// it contains the paths of all the files in the project
    pub paths: Vec<String>,
}

/// Filter used to blacklist some directories from the search
fn blacklist_filter(entry: &rust_search::DirEntry) -> bool {
    let blacklist = vec!["node_modules", "target", "dist", "build", "vendor", "bin"];
    !blacklist.contains(&entry.file_name().to_str().unwrap())
}

/// Implementation of the Project structure
/// - load: loads the project from the given location filling the paths vector
impl Project {
    pub fn load(project_location: &str) -> Result<Project, Error> {
        let paths: Vec<String> = SearchBuilder::default()
            .location(project_location)
            .custom_filter(blacklist_filter)
            .build()
            .collect();

        Ok(Project { paths })
    }
}
