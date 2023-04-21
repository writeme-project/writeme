use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, io::Write};

// Paths to significant files
pub mod paths {
    pub const CONFIGS: &str = "./src/configs.yml";
    pub const TEMPLATE: &str = "./assets/tpl/TEMPLATE.md";
    pub const SHIELD: &str = "./assets/tpl/SHIELD.md";
    pub const README: &str = "./README.md";
    pub const TECHS: &str = "./src/techs.yml";
}

// Struct used to represent shields.io badges of a technology
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

/*
    Implementation of the Shield struct with the following methods:
    - new() creates a new instance of the Shield struct
    - gen_md() generates the markdown code for the shield
*/
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

    pub fn gen_md(&self) -> String {
        let shield_tpl = fs::read_to_string(paths::SHIELD).unwrap();
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shield_tpl", shield_tpl.clone())
            .unwrap();

        let data: Value = json!(self);

        return handlebars.render("shield_tpl", &data).unwrap();
    }
}
