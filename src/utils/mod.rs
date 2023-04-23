use anyhow::Error;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

/// Paths to significant files
pub mod paths {
    pub const CONFIGS: &str = "./src/configs.yml";
    pub const TEMPLATE: &str = "./assets/tpl/TEMPLATE.md";
    pub const README: &str = "./README.md";
    pub const TECHS: &str = "./src/techs.yml";

    // small pieces of markdown which require some data to be filled in
    pub const SHIELD: &str = "./assets/tpl/SHIELD.md";
    pub const AUTHOR: &str = "./assets/tpl/AUTHOR.md";

    // large macro templates of the README file
    pub const HEADER: &str = "./assets/tpl/HEADER.md";
    pub const TOC: &str = "./assets/tpl/TABLE_OF_CONTENT.md";
    pub const BODY: &str = "./assets/tpl/BODY.md";
    pub const FOOTER: &str = "./assets/tpl/FOOTER.md";
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

/// Implementation of the shield structure, with the following methods:
/// new() creates a new instance of the Shield struct
/// gen_md() generates the markdown code for the shield
impl Shield {
    pub fn new(
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
