use crate::scanner::scan_techs;
use anyhow::Error;

use handlebars::Handlebars;
use std::fs::{self, File};
use std::io::Write;

use crate::converter::ConverterOutput;
use crate::utils::{paths, shields, GenMarkdown};
use rand::seq::SliceRandom;
use serde_json::json;

const EMOJI_LIST: [&str; 16] = [
    "🖋️", "📝", "📄", "📚", "📖", "📓", "📒", "📃", "📜", "📰", "📑", "🔖", "🔗", "📎", "📐", "📏",
];

fn random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let random_emoji = *EMOJI_LIST.choose(&mut rng).unwrap();
    return random_emoji.to_string();
}

#[derive(Debug)]
pub struct Assembler<'a> {
    handlebars: Handlebars<'a>,
    converted_config: ConverterOutput,
}

impl<'a> Assembler<'a> {
    pub fn new(converted_config: ConverterOutput) -> Self {
        Assembler {
            handlebars: Handlebars::new(),
            converted_config,
        }
    }

    fn assemble_header(&mut self) -> String {
        let header_tpl = fs::read_to_string(paths::HEADER).unwrap();

        let techs = scan_techs().unwrap();
        let shields = shields(techs).unwrap();

        let header = json!({
            "icon": Some(random_emoji()),
            "title": self.converted_config.name.clone(),
            "description": self.converted_config.description.clone(),
            "shields": Some(shields),
            "about": Some("miao".to_string()),
        });

        self.handlebars
            .register_template_string("header_tpl", header_tpl.clone())
            .unwrap();

        self.handlebars.render("header_tpl", &header).unwrap()
    }

    fn assemble_table_of_contents(&self) -> String {
        let toc_tpl = fs::read_to_string(paths::TOC).unwrap();

        toc_tpl
    }

    fn assemble_body(&mut self) -> String {
        let body_tpl = fs::read_to_string(paths::BODY).unwrap();

        let body = json!({
            "license": self.converted_config.license.clone(),
        });

        self.handlebars
            .register_template_string("body_tpl", body_tpl.clone())
            .unwrap();

        self.handlebars.render("body_tpl", &body).unwrap()
    }

    fn assemble_footer(&mut self) -> String {
        let footer_tpl = fs::read_to_string(paths::FOOTER).unwrap();

        let authors: Option<String> = match self.converted_config.contributors.clone() {
            Some(contributors) => {
                let mut authors = String::new();

                for c in contributors {
                    let author = "- ".to_string();

                    match c.gen_md() {
                        Ok(md) => {
                            authors.push_str(&author);
                            authors.push_str(&md);
                            authors.push_str("\n");
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }

                Some(authors)
            }
            None => None,
        };

        let fundings: Option<String> = match self.converted_config.funding.clone() {
            Some(fundings) => {
                let mut supports: String = String::new();

                for f in fundings {
                    match f.gen_md() {
                        Ok(md) => {
                            supports.push_str(&md);
                            supports.push_str(" ");
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }

                Some(supports)
            }
            None => None,
        };

        let footer = json!({
            "name": self.converted_config.name.clone(),
            "authors": authors,
            "fundings": fundings,
        });

        self.handlebars
            .register_template_string("footer_tpl", footer_tpl.clone())
            .unwrap();

        self.handlebars.render("footer_tpl", &footer).unwrap()
    }

    pub fn assemble(&mut self) -> Result<(), Error> {
        let mut readme_file = match File::create(paths::README) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        let header = self.assemble_header();
        let toc = self.assemble_table_of_contents();
        let body = self.assemble_body();
        let footer = self.assemble_footer();

        readme_file.write_all(header.as_bytes())?;
        readme_file.write_all(b"\n")?;
        readme_file.write_all(toc.as_bytes())?;
        readme_file.write_all(b"\n")?;
        readme_file.write_all(body.as_bytes())?;
        readme_file.write_all(b"\n")?;
        readme_file.write_all(footer.as_bytes())?;

        Ok(())
    }
}