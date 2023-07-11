use std::{collections::HashMap, fmt::Display, fs, io::Write, str::FromStr};

use anyhow::anyhow;
use anyhow::Error;
use chrono::{self, Datelike};
use enum_assoc::Assoc;
use handlebars::Handlebars;
use serde_json::json;
use std::fs::File;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::{
    converter::ConverterOutput,
    utils::{
        paths::{self, read_util_file_contents, UtilityPath},
        GenMarkdown,
    },
};

#[derive(Debug, Clone, PartialEq, EnumIter, Copy, Eq, Hash, Assoc)]
#[func(pub const fn keywords(&self) -> &str)]
/// The available licenses for a project which a user can choose from
pub enum SupportedLicense {
    #[assoc(keywords = "unknown")]
    Unknown,
    #[assoc(keywords = "apache2, apache-2.0, apache-2, apache2.0, apache")]
    Apache20,
    #[assoc(keywords = "mit, mit license")]
    MIT,
    #[assoc(keywords = "gnu general public license, gnu gpl, gpl")]
    GNUGeneralPublicLicense,

    #[assoc(
        keywords = "gnu lesser general public license, Attribution-ShareAlike 4.0, cc-by-sa-4.0, cc-by-sa"
    )]
    CreativeCommonsAttributionShareAlike40,
}

impl ToString for SupportedLicense {
    fn to_string(&self) -> String {
        match self {
            SupportedLicense::Unknown => "Unknown",
            SupportedLicense::Apache20 => "Apache-2.0",
            SupportedLicense::MIT => "MIT",
            SupportedLicense::GNUGeneralPublicLicense => "GNU General Public License",
            SupportedLicense::CreativeCommonsAttributionShareAlike40 => {
                "Creative Commons Attribution-ShareAlike 4.0"
            }
        }
        .to_string()
    }
}

impl FromStr for SupportedLicense {
    type Err = ();

    /// Returns a License object from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        let found = SupportedLicense::iter().find(|option| {
            // regex is case insensitive and considers only whole words, example "mit" will not match emit
            let options: Vec<String> = option
                .keywords()
                .split(",")
                .map(|f| f.trim())
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| format!(r"(?i)\b{}\b", c))
                .collect();

            let regex_set = match regex::RegexSet::new(options) {
                Ok(regex_set) => regex_set,
                Err(_) => return false,
            };

            regex_set.is_match(s.as_str())
        });

        match found {
            Some(license) => Ok(license),
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The license object and related information
pub struct License {
    /// The license name
    pub name: SupportedLicense,
    /// The location in the project structure
    pub path: Option<String>,
    /// The location on the web (github repository for example)
    pub url: Option<String>,
}

impl License {
    /// Create a new License object from a name (string) base on the SupportedLicense methods
    ///
    /// If the name of the license is not recognized, the license will be set to `SupportedLicense::Unknown`
    pub fn from_name(name: String) -> Self {
        let license = match SupportedLicense::from_str(&name) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        Self {
            name: license,
            path: None,
            url: None,
        }
    }

    /// Create a new License object from a file, the new License object will contain the license name and the path to the file
    ///
    /// If the license file is from a known platform (eg: github), the url will be set to the file location on the platform
    pub fn from_file(path: String) -> Self {
        let content = match fs::read_to_string(&path) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        match SupportedLicense::from_str(&content) {
            Ok(license) => Self {
                name: license,
                path: Some(path),
                url: None,
            },
            Err(_) => Self {
                name: SupportedLicense::Unknown,
                path: Some(path),
                url: None,
            },
        }
    }

    /// Scan the project for a license file and return a list of converter output
    /// each converter output will contain a License object with name and path set
    ///
    pub fn scan(paths: &Vec<String>) -> Result<Vec<ConverterOutput>, Error> {
        let contents: String = paths::read_util_file_contents(paths::UtilityPath::Lincenses);
        let all_licenses: HashMap<String, Vec<String>> = serde_yaml::from_str(&contents).unwrap();

        let mut converter_outputs: Vec<ConverterOutput> = vec![];
        let mut license_present: Vec<String> = vec![];

        // get all the possible license file names and create a regex set
        // the regex set is case insensitive and always consider the name as the last
        // part of the path
        let all_license_file: Vec<String> = all_licenses
            .values()
            .flatten()
            .map(|c| format!(r"(?i){}$", c))
            .collect();

        let regex_set: regex::RegexSet = match regex::RegexSet::new(all_license_file) {
            Ok(regex_set) => regex_set,
            Err(_) => return Err(anyhow!("Error creating regex set")),
        };

        for path in paths {
            let path_str = path.as_str();

            let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();
            if !matches.is_empty() {
                license_present.push(path_str.to_string());
            }
        }

        license_present.iter().for_each(|p| {
            let mut converter = ConverterOutput::empty();
            converter.source_config_file_path = p.to_string();
            converter.license = Option::from(License::from_file(p.to_string()));

            converter_outputs.push(converter);
        });

        Ok(converter_outputs)
    }

    /// Create a license file in the project
    ///
    pub fn create(
        project_location: &str,
        license: &License,
        project_name: Option<String>,
    ) -> Result<Option<String>, Error> {
        // if there is a path for the license or the license is unknown, dont create a license
        if license.path.is_some() || license.name == SupportedLicense::Unknown {
            return Ok(None);
        }

        let output_path = format!("{}/LICENSE", project_location);
        let mut license_file = match File::create(output_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        // load the right license template based on the license name
        let license_contents = match license.name {
            SupportedLicense::Apache20 => read_util_file_contents(UtilityPath::Apache20),
            SupportedLicense::MIT => read_util_file_contents(UtilityPath::MIT),
            SupportedLicense::GNUGeneralPublicLicense => {
                read_util_file_contents(UtilityPath::GNUGPL)
            }
            SupportedLicense::CreativeCommonsAttributionShareAlike40 => {
                read_util_file_contents(UtilityPath::CreativeCommonsAttributionShareAlike40)
            }
            SupportedLicense::Unknown => {
                return Ok(None);
            }
        };

        let year = chrono::Utc::now().year().to_string();

        // some license file require project info
        let license_data = json!({
            "year": year,
            "project_name": project_name.unwrap_or("".to_string())
        });

        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_string("license_tpl", license_contents)
            .unwrap();

        let render = handlebars.render("license_tpl", &license_data).unwrap();

        license_file.write_all(render.as_bytes())?;

        Ok(Option::from(output_path))
    }
}

impl GenMarkdown for License {
    fn gen_md(&self) -> Result<String, anyhow::Error> {
        let license_tpl = paths::read_util_file_contents(paths::UtilityPath::LicenseReadme);
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.register_template_string("license_tpl", license_tpl)?;

        let data = if let Some(url) = &self.url {
            json!({
                "name": self.name.to_string(),
                "target": url.clone()
            })
        } else {
            json!({
                "name": self.name.to_string()
            })
        };

        Ok(handlebars.render("license_tpl", &data).unwrap())
    }
}

/// Implement the Display trait for License
/// This will allow to print the license name
impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // we dont want to print the path or the url
        // to avoid duplicate information during merge process
        write!(f, "{}", self.name.to_string())
    }
}
