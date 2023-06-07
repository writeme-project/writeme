use crate::{
    converter::{ConverterOutput, RepositoryPlatform},
    scanner::{self, scan_dependencies, scan_techs},
    utils::{fantasy_description, paths, shields, Aligment, GenMarkdown},
};
use anyhow::Error;

use handlebars::Handlebars;
use serde_json::json;
use std::fs::File;
use std::io::Write;

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

    fn assemble_header(&mut self, to_make_shields: Vec<String>) -> String {
        let header_tpl = paths::read_util_file_contents(paths::UtilityPath::Header);

        let shields = shields(to_make_shields, Aligment::Row).unwrap();

        // if name is none or empty, set it to default "Project Name"
        if self.converted_config.name.is_none()
            || self.converted_config.name.as_ref().unwrap().is_empty()
        {
            self.converted_config.name = Some("Project Name".to_string());
        }

        let header = json!({
            "title": self.converted_config.name,
            "description": self.converted_config.description.clone(),
            "shields": Some(shields),
            "fantasy_description": Some(fantasy_description()),
        });

        self.handlebars
            .register_template_string("header_tpl", header_tpl)
            .unwrap();

        self.handlebars.render("header_tpl", &header).unwrap()
    }

    fn assemble_table_of_contents(&self) -> String {
        paths::read_util_file_contents(paths::UtilityPath::Toc)
    }

    fn assemble_body(&mut self, paths: &Vec<String>) -> String {
        let body_tpl = paths::read_util_file_contents(paths::UtilityPath::Body);

        let license = match scanner::find_license_file(&paths) {
            Ok(license_path) => {
                let license_tpl = paths::read_util_file_contents(paths::UtilityPath::License);

                self.handlebars
                    .register_template_string("license_tpl", license_tpl)
                    .unwrap();

                let license = json!({
                    "name": license_path.split('/').last().unwrap(),
                    "target": license_path,
                });

                self.handlebars.render("license_tpl", &license).unwrap()
            }
            Err(_) => self
                .converted_config
                .license
                .clone()
                .unwrap_or("Not found".to_string()),
        };

        let body = json!({
            "license": license.clone(),
            "repository_url": self.converted_config.repository.as_ref().unwrap().url.clone(),
        });

        self.handlebars
            .register_template_string("body_tpl", body_tpl)
            .unwrap();

        self.handlebars.render("body_tpl", &body).unwrap()
    }

    fn assemble_footer(&mut self) -> String {
        let footer_tpl = paths::read_util_file_contents(paths::UtilityPath::Footer);

        let contrib_section;
        let repository = self.converted_config.repository.as_ref().unwrap();
        if repository.platform == RepositoryPlatform::Github {
            contrib_section = Some(repository.gen_md().unwrap());
        } else {
            contrib_section = match self.converted_config.contributors.clone() {
                Some(contributors) => {
                    let mut authors = String::new();

                    for c in contributors {
                        let author = "- ".to_string();

                        match c.gen_md() {
                            Ok(md) => {
                                authors.push_str(&author);
                                authors.push_str(&md);
                                authors.push('\n');
                            }
                            // if there is an error to generate markdown, just skip this contributor
                            Err(_) => continue,
                        }
                    }

                    Some(authors)
                }
                None => None,
            };
        }

        let funding: Option<String> = match self.converted_config.funding.clone() {
            Some(funding) => {
                let mut supports: String = String::new();

                for f in funding {
                    match f.gen_md() {
                        Ok(md) => {
                            supports.push_str(&md);
                            supports.push(' ');
                        }
                        // if there is an error to generate markdown, just skip this funding
                        Err(_) => continue,
                    }
                }

                Some(supports)
            }
            None => None,
        };

        let footer = json!({
            "name": self.converted_config.name.clone(),
            "authors": contrib_section.unwrap_or("".to_string()),
            "funding": funding,
        });

        self.handlebars
            .register_template_string("footer_tpl", footer_tpl)
            .unwrap();

        self.handlebars.render("footer_tpl", &footer).unwrap()
    }

    pub fn assemble(&mut self, output_path: &str, path: &Vec<String>) -> Result<(), Error> {
        let mut readme_file = match File::create(output_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        let techs: Vec<String> = scan_techs(path).unwrap();
        let deps: Vec<String> =
            scan_dependencies(self.converted_config.dependencies.clone().unwrap()).unwrap();

        let to_make_shields: Vec<String> = techs.iter().chain(deps.iter()).cloned().collect();

        let header = self.assemble_header(to_make_shields.clone());
        let toc = self.assemble_table_of_contents();
        let body = self.assemble_body(path);
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
