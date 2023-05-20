use anyhow::Error;
use handlebars::Handlebars;
use rust_search::{FilterExt, SearchBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fs};

/// Paths to output files saved to disk produced by the application
pub mod outputs {
    pub const README: &str = "README.md";
}

/// Paths to significant files
pub mod paths {
    pub const CONFIGS: &str = "./conf/configs.yml";
    pub const TECHS: &str = "./conf/techs.yml";

    // small pieces of markdown which require some data to be filled in
    pub const SHIELD: &str = "./tpl/SHIELD.md";
    pub const AUTHOR: &str = "./tpl/AUTHOR.md";
    pub const SUPPORT: &str = "./tpl/SUPPORT.md";

    // large macro templates of the README file
    pub const HEADER: &str = "./tpl/HEADER.md";
    pub const TOC: &str = "./tpl/TABLE_OF_CONTENT.md";
    pub const BODY: &str = "./tpl/BODY.md";
    pub const FOOTER: &str = "./tpl/FOOTER.md";
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
        let shield_tpl = fs::read_to_string(paths::SHIELD).unwrap();
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shield_tpl", shield_tpl.clone())
            .unwrap();

        let data: Value = json!(self);

        return Ok(handlebars.render("shield_tpl", &data).unwrap());
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
    let contents: String =
        fs::read_to_string(paths::TECHS).expect("Something went wrong reading the techs file");
    let all_techs: HashMap<String, Tech> = serde_yaml::from_str(&contents).unwrap();
    let mut shields: String = String::new();
    for (name, tech) in all_techs {
        if techs.contains(&name) {
            match tech.shield.gen_md() {
                Ok(md) => {
                    shields.push_str(&md);
                    match aligment {
                        Aligment::Row => shields.push_str(" "),
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
