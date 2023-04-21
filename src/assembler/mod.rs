use std::fs::{self, File};
use std::io::Write;

use anyhow::Error;
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::converter::{Contributors, ConverterOutput};
use crate::scanner;
use crate::utils::{paths, Shield};
use serde_json::{json, Value};

const EMOJI_LIST: [&str; 16] = [
    "🖋️", "📝", "📄", "📚", "📖", "📓", "📒", "📃", "📜", "📰", "📑", "🔖", "🔗", "📎", "📐", "📏",
];

fn random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let random_emoji = *EMOJI_LIST.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

#[derive(Debug, Serialize)]
struct Header {
    icon: Option<String>,
    title: Option<String>,
    description: Option<String>,
    about: Option<String>,
    shields: Option<String>,
}

#[derive(Debug)]
pub struct Assembler {
    converted_config: ConverterOutput,
    header: Option<Header>,
}

impl Assembler {
    pub fn new(converted_config: ConverterOutput) -> Self {
        Assembler {
            converted_config,
            header: None,
        }
    }

    fn assemble_header(&self) -> String {
        let header_tpl = fs::read_to_string(paths::HEADER).unwrap();

        let shields = scanner::scan_techs().unwrap().join("\n");

        let header = Header {
            icon: Some(random_emoji()),
            title: self.converted_config.name.clone(),
            description: self.converted_config.description.clone(),
            shields: Some(shields),
            about: Some("miao".to_string()),
        };

        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_string("header_tpl", header_tpl.clone())
            .unwrap();

        handlebars.render("header_tpl", &json!(header)).unwrap()
    }

    pub fn assemble(&mut self) -> Result<(), Error> {
        let mut readme_file = match File::create(paths::README) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        let header = self.assemble_header();

        readme_file.write_all(header.as_bytes())?;

        Ok(())
    }
}
