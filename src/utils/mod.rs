use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

pub mod paths {
    pub const CONFIGS: &str = "./src/configs.yml";
    pub const TEMPLATE: &str = "./assets/TEMPLATE.md";
    pub const SHIELD: &str = "./assets/SHIELD.md";
    pub const README: &str = "./README.md";
    pub const TECHS: &str = "./src/techs.yml";
}

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
        let shield_tpl = fs::read_to_string(paths::SHIELD).unwrap();
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
